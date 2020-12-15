const path = require('path');
const htmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: path.resolve(__dirname, "src/bootstrap.tsx"),
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js"
  },
  module: {
    rules: [{
      test: /\.tsx?$/,
      loader: "ts-loader"
    },
    {
      test: /\.css$/,
      loader: ['style-loader', 'css-loader']
      },
    {
      // 追記
      test: /\.(jpg|png)$/,
      loaders: 'url-loader'
    }, ]
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js"]
  },
  devServer: {
    contentBase: path.resolve(__dirname, "public")
  },
  plugins: [
    new htmlWebpackPlugin({
      template: './src/index.html'
    }),
  ]
};
