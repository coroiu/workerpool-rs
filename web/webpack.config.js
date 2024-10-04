const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/index.ts',  // Your entry file
  output: {
    filename: '[name].js',
    assetModuleFilename: (pathData) => {
      if (pathData.filename.match(/worker\.ts$/)) {
        return '[name].js';
      } else {
        return 'assets/[name][ext]';
      }
    },
    // path: path.resolve(__dirname, 'dist'),
  },
  resolve: {
    extensions: ['.ts', '.js', '.wasm'],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.wasm$/,
        type: "asset/resource", // Use this to bundle wasm as a static file
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './src/index.html',
    }),
    new CopyWebpackPlugin({
      patterns: [
        { from: 'node_modules/@sdk/wasm', to: 'wasm' },
      ]
    })
  ],
  devServer: {
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp"
    },
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 3000,
  },
  experiments: {
    asyncWebAssembly: true,
  },
  mode: 'development',  // Set to 'production' for a production build
};
