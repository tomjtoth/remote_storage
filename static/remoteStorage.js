const remoteStorage = {

    _db_url: 'https://oracle-dev.tomjtoth.h4ck.me:55589',

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
        const resp = await fetch(this._db_url + '/storage', {
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'POST',
            body: JSON.stringify(args)
        });

        return resp.json();
    }
}
