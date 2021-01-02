const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: true,
      port: 8000,
      historyApiFallback: true,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "jinya.js",
      webassemblyModuleFilename: "jinya.wasm"
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
        watchDirectories: [
          path.resolve(__dirname, "../jinya-ui/src")
        ],
        // forceMode: 'production',
      })
    ],
    watch: argv.mode !== 'production'
  };
};
