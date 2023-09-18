# About

Remote version of `localStorage`, 1 GIGANTIC security hole.

## Build

Simply run `cargo build --release` or if you need to cross-compile, like me, try something like:
- ~~`rustup target add aarch64-unknown-linux-gnu`~~
- install `docker` and `cross`
    - run `sudo systemctl start docker`
- run `cross build --release --target=aarch64-unknown-linux-gnu`
- copy over your binary to the server
    - also copy `.env` and `schema.sql` which is read on every startup

## Server-side

Get a TLS certificate via `certbot`. 
Set `.env` variables 
- `TLS_CERT` and `TLS_KEY` accordingly.
- `ALLOWED_HOSTS='your-username\.github\.io'`
    - it is merged into a larger **`regex pattern`**
    - if the `Origin` of a request is different, it `HTTP 403` is returned

## Client-side

Store and retrieve data via fetch. Copy the below `<script>` tag into your static webpage.

```html
<script src="https://oracle-dev.tomjtoth.h4ck.me:44480/static/remoteStorage.js">
```

Usage is as below:
- `remoteStorage.set(key, val)` to store data
- `remoteStorage.get(key)` to retrieve data
- `remoteStorage.clear()` to clear the site's data
