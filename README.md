# About

Remote and `async` version of `localStorage`, 1 GIGANTIC security hole.

## Build

Simply run `cargo build --release` or if you need to cross-compile, like me, try something like:
- ~~`rustup target add aarch64-unknown-linux-gnu`~~
- install `docker` and `cross`
    - run `sudo systemctl start docker`
- run `cross build --release --target=aarch64-unknown-linux-gnu`
- copy over your binary to the server
    - also copy `.env` and `schema.sql` which is read on every startup

## Server-side

Set `.env` variables:
- path to your database file:
    - `DB_URI=path/to/your.db`
    - default is `DB_URI=fallback.db`
- SocketAddress to listen on:
    - `LISTEN_ON=0.0.0.0:55589`
    - default is `0.0.0.0:80` which requires root priviliges
- `ALLOWED_HOSTS='your-username\.github\.io'`
    - it is merged into a larger **`regex pattern`**
    - if the `Origin` of a request is different, it `HTTP 403` is returned
- Get a TLS certificate via `certbot`
    - `TLS_CERT` and `TLS_KEY` accordingly
    - e.g. `TLS_CERT=/etc/letsencrypt/live/your-website.xyz/fullchain.pem`
    - e.g. `TLS_KEY=/etc/letsencrypt/live/your-website.xyz/privkey.pem`
- `CLEARING_SITE_DATA_ALLOWED`:
    - `true` to to enable
    -  or `false` to disable 

## Client-side

Store and retrieve data via fetch. Copy the below `<script>` tag into your static webpage.

```html
<script src="https://oracle-dev.tomjtoth.h4ck.me:55589/remoteStorage.js">
```

Usage is as below:
- `await remoteStorage.set(key, val)` to store data
- `let value = await remoteStorage.get(key)` to retrieve data
- `await remoteStorage.clear()` to clear the site's data
    - provided that `.env` contains `CLEARING_SITE_DATA_ALLOWED`

## Manual test

Tested from 2 different host:
- populated 7+2 entries in table `strings`
- then cleared the smaller portion
- as expected:
    - only the hostname from `strings` got deleted
    - only rows belonging to said `host` got deleted from table `junction`
