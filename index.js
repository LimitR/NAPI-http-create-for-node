const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'server_rust_node' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `server_rust_node.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `server_rust_node-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'app', 'server_rust_node')
