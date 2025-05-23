use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, Method, StatusCode},
    routing::post,
    Json,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenv::var;
use regex::Regex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
    allow_list: Regex,
    clearing_site_data_allowed: bool,
}

type St = axum::extract::State<AppState>;

fn get_url(url: Option<&HeaderValue>) -> Option<String> {
    if let Some(inner) = url {
        return Some(inner.to_str().unwrap().to_string());
    }

    None
}

async fn logic(s: St, url: String, args: Vec<String>) -> (StatusCode, Json<Option<Vec<String>>>) {
    if !s.allow_list.is_match(&url) {
        println!("FORBIDDEN: {}", url);
        return (StatusCode::FORBIDDEN, Json(None));
    }
    println!("ACCEPTED: {}", url);

    // remoteStorage.clear()
    if args.len() == 0 {
        if !s.clearing_site_data_allowed {
            return (StatusCode::FORBIDDEN, Json(None));
        };
        if let Ok(_) = sqlx::query(r"delete from strings where str == ?")
            .bind(url)
            .execute(&s.pool)
            .await
        {
            (StatusCode::OK, Json(None))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
    // remoteStorage.get(key)
    else if args.len() == 1 {
        if let Ok(res) = sqlx::query_scalar(r"select val from data where url == ? and key == ?")
            .bind(url)
            .bind(&args[0])
            .fetch_all(&s.pool)
            .await
        {
            (StatusCode::OK, Json(Some(res)))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
    // remoteStorage.set(key, val)
    else {
        if let Ok(_) = sqlx::query(r"insert into data select ?, ?, ?")
            .bind(url)
            .bind(&args[0])
            .bind(&args[1])
            .execute(&s.pool)
            .await
        {
            (StatusCode::OK, Json(None))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let listen_on = var("LISTEN_ON").unwrap_or("0.0.0.0:80".to_string());
    let sock_addr = listen_on.parse().expect("failed to parse ADDR:PORT");

    let tls_cert = var("TLS_CERT").expect("TLS_KEY not found in `.env`");
    let tls_key = var("TLS_KEY").expect("TLS_KEY not found in `.env`");

    let clearing_site_data_allowed =
        var("CLEARING_SITE_DATA_ALLOWED").unwrap_or("false".to_string()) == "true";

    let mut allowed_hosts = r"^https?:\/\/(?:localhost|127\.0\.0\.1".to_string();
    if let Ok(s) = var("ALLOWED_HOSTS") {
        allowed_hosts.push_str("|");
        allowed_hosts.push_str(&s);
    };

    // specified port number
    allowed_hosts.push_str(r")(?::\d{1,5})?$");

    let allow_list =
        Regex::new(&allowed_hosts).expect("unable to compile regex from ALLOWED_HOSTS");

    let tls_conf = RustlsConfig::from_pem_file(tls_cert, tls_key)
        .await
        .expect("TLS config failure");

    let db_uri = var("DB_URI").unwrap_or("fallback.db".to_string());

    let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_uri))
        .await
        .expect("unable to connect to DB");

    sqlx::query(&std::fs::read_to_string("schema.sql").expect("unable to read schema.sql"))
        .execute(&pool)
        .await
        .expect("unable to execute schema.sql");

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let routes = axum::Router::new()
        .route(
            "/storage",
            post(
                |s: St, headers: HeaderMap, Json(args): Json<Vec<String>>| async move {
                    if let Some(url) = get_url(headers.get("Origin")) {
                        logic(s, url, args).await
                    } else {
                        (StatusCode::BAD_REQUEST, Json(None))
                    }
                },
            ),
        )
        .route("/meaning/of/life", axum::routing::get(Json(42)))
        .nest_service("/", ServeDir::new("static"))
        .with_state(AppState {
            pool,
            allow_list,
            clearing_site_data_allowed,
        })
        .layer(cors_layer);

    axum_server::bind_rustls(sock_addr, tls_conf)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
