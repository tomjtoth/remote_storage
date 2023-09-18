const remoteStorage = {

    _db_url: 'https://oracle-dev.tomjtoth.h4ck.me:44480',

    async clear() {
        return await this._rw()
    },

    async get(key) {
        return await this._rw(key);
    },

    async set(key, val) {
        return await this._rw(key, val);
    },

    async _rw(...args) {
        return await fetch(this._db_url + '/storage', {
            method: 'POST',
            mode: 'cors',
            body: args
        }).then(response => response.json())
    }
}
