# zero-auth zero-security async storage similar to window.localStorage

I use this as a text bucket where my pages' visitors simply can append text.

## About

Remote and `async` version of `localStorage`, 1 __GIGANTIC SECURITY HOLE__. Based on the [Origin](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin) of the HTTP request, it gets rejected with a `HTTP 403: FORBIDDEN` or gets processed. The stored data is then readable and *appendable* by **ALL** visitors of your website (Origin).

## Example

At this point the DB holds data related to 2 websites:
- https://tomjtoth.github.io
- https://oracle-dev.tomjtoth.h4ck.me:55589

Extract of the human-readable `data` view:

```
sqlite> select * from data;
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq
https://tomjtoth.github.io|omena|qqq2
https://tomjtoth.github.io|omena|qqqy
https://tomjtoth.github.io|omena|qqqyyyy
https://tomjtoth.github.io|omena|qqqyyyíyy
https://tomjtoth.github.io|omena|qqqyyyíyy
https://tomjtoth.github.io|omena|qqqyyyíyy
https://tomjtoth.github.io|omena|qqqyyyíyy
https://oracle-dev.tomjtoth.h4ck.me:55589|koira|lalala
https://oracle-dev.tomjtoth.h4ck.me:55589|omena|lalala
https://oracle-dev.tomjtoth.h4ck.me:55589|omena|lalala223
https://oracle-dev.tomjtoth.h4ck.me:55589|omena|qqq
https://oracle-dev.tomjtoth.h4ck.me:55589|omena|qqq2
https://oracle-dev.tomjtoth.h4ck.me:55589|koira|qq2
```

...that actually is stored in table `junction`:

```
sqlite> select * from junction;
1|2|3
1|2|3
1|2|3
1|2|3
1|2|3
1|2|3
1|2|3
1|2|4
1|2|5
1|2|6
1|2|7
1|2|7
1|2|7
1|2|7
10|11|12
10|2|12
10|2|13
10|2|3
10|2|4
10|11|9
```

...which only references uniqe texts saved to table `strings`:

```
sqlite> select * from strings;
1|https://tomjtoth.github.io
2|omena
3|qqq
4|qqq2
5|qqqy
6|qqqyyyy
7|qqqyyyíyy
9|qq2
10|https://oracle-dev.tomjtoth.h4ck.me:55589
11|koira
12|lalala
13|lalala223
```

If `CLEARING_SITE_DATA_ALLOWED`, then *any* client of the above mentioned 2 websites could erase ALL data related to that website, meaning `remoteStorage.clear()` initiated from `tomjtoth.github.io` would delete _row nro. 1_ from table `strings`, which in turn deletes _all rows_ from table junction _where the 1st column is `1`_. For this reason it defaults to `false`.


## Build

Simply run `cargo build --release` or if you need to cross-compile, like me, try something like:
- ~~`rustup target add aarch64-unknown-linux-gnu`~~
- install `docker` and `cross`
    - run `sudo systemctl start docker`
- run `cross build --release --target=aarch64-unknown-linux-gnu`
- copy over your binary to the server
    - also copy `.env` and `schema.sql` which is read on every startup

## Server-side

Get a TLS certificate, I used [Let's Encrypt](https://letsencrypt.org/) via installing `certbot` and running it with the `--standalone` and `certonly` flags. Running an HTTP server and making requests to it from an Origin that's using HTTPS would result in [Mixed content](https://developer.mozilla.org/en-US/docs/Web/Security/Mixed_content).

Set `.env` variables:
- path to your database file:
    - `DB_URI=path/to/your.db`
    - default is `DB_URI=fallback.db`
- SocketAddress to listen on:
    - `LISTEN_ON=0.0.0.0:55589`
    - default is `0.0.0.0:80` which requires root priviliges
- `ALLOWED_HOSTS='your-username\.github\.io'`
    - it is merged into a larger **`regex pattern`**
    - if the `Origin` of a request is different, `HTTP 403` is returned
- set `TLS_CERT` and `TLS_KEY`:
    - e.g. `TLS_CERT=/etc/letsencrypt/live/your-website.xyz/fullchain.pem`
    - e.g. `TLS_KEY=/etc/letsencrypt/live/your-website.xyz/privkey.pem`
- set `CLEARING_SITE_DATA_ALLOWED`:
    - `true` to to enable
    -  or `false` to disable 

Automate via `systemd`. The included [service](./remote_storage.service)
- starts the server on system start 
- and restarts it every 10 seconds upon failure. 
- I also enabled [user lingering](https://serverfault.com/questions/846441/loginctl-enable-linger-disable-linger-but-reading-linger-status)
- and the working directory is located under `~/remote_storage/`
    - you'll have to adjust the service file if you deviate

## Client-side

Read instructions from <a href="https://oracle.ttj.hu:55589">here</a>.

## Manual test

Tested from 2 different host:
- populated 7+2 entries in table `strings`
- then cleared the smaller portion
- as expected:
    - only the hostname from `strings` got deleted
    - only rows belonging to said `host` got deleted from table `junction`
