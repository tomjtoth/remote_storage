# About

Remote version of `localStorage`, 1 GIGANTIC security hole.

## Server-side

Edit the `.env` file to contain at least `ALLOWED_HOSTS='username\.github\.io'` it is merged into a larger regex pattern.

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
const remote_storage = new RemoteStorage('http://localhost:44480');
```
