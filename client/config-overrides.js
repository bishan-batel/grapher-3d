/**
 * Config-Override, used to edit how project is built & ran
 * The only thing I'm changing from default config is adding the WasmPack plugin
 * to allow loading WASM packages
 */

const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const path = require('path')
module.exports = function override(config, env) {
  // Make file-loader ignore WASM files

  // noinspection JSValidateTypes
  config.module.rules.forEach((rule) => {
    ;(rule.oneOf || []).forEach((oneOf) => {
      // noinspection JSValidateTypes
      if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
        oneOf.exclude.push(/\.wasm$/)
      }
    })
  })

  // Add WasmPack Plugin
  config.plugins.push(
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '../lib'),

      // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
      // the available set of arguments.
      //
      // Optional space delimited arguments to appear before the wasm-pack
      // command. Default arguments are `--verbose`.
      args: '--log-level warn', // Default arguments are `--typescript --target browser --mode normal`.
      extraArgs: '--target web',
      withTypescript: true,

      // Optional array of absolute paths to directories, changes to which
      // will trigger the build.
      // watchDirectories: [
      //   path.resolve(__dirname, "another-crate/src")
      // ],

      // The same as the `--out-dir` option for `wasm-pack`
      outDir: '../client/src/crate-build',

      // The same as the `--out-name` option for `wasm-pack`
      // outName: "index",

      // If defined, `forceWatch` will force activate/deactivate watch mode for
      // `.rs` files.
      //
      // The default (not set) aligns watch mode for `.rs` files to Webpack's
      // watch mode.
      forceWatch: true,

      // If defined, `forceMode` will force the compilation mode for `wasm-pack`
      //
      // Possible values are `development` and `production`.
      //
      // the mode `development` makes `wasm-pack` build in `debug` mode.
      // the mode `production` makes `wasm-pack` build in `release` mode.
      // forceMode: "development",

      // Controls plugin output verbosity, either 'info' or 'error'.
      // Defaults to 'info'.
      // pluginLogLevel: 'info'
    }),
  )

  return config
}
