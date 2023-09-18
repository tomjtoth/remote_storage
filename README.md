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

Store and retrieve data via fetch. The below class actually works:

```javascript
class RemoteStorage {

    constructor(db_url) {
        this.db_url = db_url;
    }

    async get(key) {
        return this.set(key);
    };

    async set(key, val = null) {
        return await fetch(this.db_url + encodeURI('/storage/' + (val
            ? key + '/' + val
            : key
        )), {
            method: 'POST',
            mode: 'cors'
        })
    }
}
// please don't abuse my server, thanks <3
const remote_storage = new RemoteStorage('https://oracle-dev.tomjtoth.h4ck.me:44480');
```
