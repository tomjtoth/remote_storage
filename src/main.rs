use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::post,
    Json,
};
use dotenv::var;
use regex::Regex;

#[derive(Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
    allow_list: Regex,
}

type St = axum::extract::State<AppState>;

fn get_url(url: Option<&HeaderValue>) -> Option<String> {
    if let Some(inner) = url {
        return Some(inner.to_str().unwrap().to_string());
    }

    None
}

async fn logic(
    s: St,
    h: HeaderMap,
    key: String,
    val: Option<String>,
) -> (StatusCode, Json<Option<Vec<String>>>) {
    if let Some(url) = get_url(h.get("Origin")) {
        if !s.allow_list.is_match(&url) {
            println!("FORBIDDEN: {}", url);
            return (StatusCode::FORBIDDEN, Json(None));
        }
        if val.is_some() {
            if let Ok(_) = sqlx::query(r"insert into data select ?, ?, ?")
                .bind(url)
                .bind(key)
                .bind(val)
                .execute(&s.pool)
                .await
            {
                (StatusCode::OK, Json(None))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        } else {
            if let Ok(res) = sqlx::query_scalar(r"select val from data where url == ? and key == ?")
                .bind(url)
                .bind(key)
                .fetch_all(&s.pool)
                .await
            {
                (StatusCode::OK, Json(Some(res)))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, Json(None))
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let listen_on = var("LISTEN_ON").unwrap_or("0.0.0.0:80".to_string());
    let sock_addr = &listen_on.parse().expect("failed to parse ADDR:PORT");
    let mut allowed_hosts = r"^https?:\/\/(?:localhost|127\.0\.0\.1".to_string();
    if let Ok(s) = var("ALLOWED_HOSTS") {
        allowed_hosts.push_str("|");
        allowed_hosts.push_str(&s);
    };

    // specified port number
    allowed_hosts.push_str(r")(?::\d{1,5})?$");

    let allow_list =
        Regex::new(&allowed_hosts).expect("unable to compile regex from ALLOWED_HOSTS");

    let db_uri = var("DB_URI").unwrap_or("fallback.db".to_string());

    let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_uri))
        .await
        .expect("unable to connect to DB");

    sqlx::query(&std::fs::read_to_string("schema.sql").expect("unable to read schema.sql"))
        .execute(&pool)
        .await
        .expect("unable to execute schema.sql");

    let routes = axum::Router::new()
        .route(
            "/storage/:key",
            post(
                |s: St, Path(key): Path<String>, headers: HeaderMap| async move {
                    logic(s, headers, key, None).await
                },
            ),
        )
        .route(
            "/storage/:key/:val",
            post(
                |s: St, Path((key, val)): Path<(String, String)>, headers: HeaderMap| async move {
                    logic(s, headers, key, Some(val)).await
                },
            ),
        )
        .with_state(AppState { pool, allow_list });

    axum::Server::bind(sock_addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
