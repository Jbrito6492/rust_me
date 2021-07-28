const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const ENTRY_DIR = path.join(__dirname, "frontend");
const DIST_DIR = path.join(__dirname, "frontend", "dist");

const config = {
  mode: "development",
  entry: `${ENTRY_DIR}/index.tsx`,
  devtool: "inline-source-map",
  output: {
    path: DIST_DIR,
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: ["babel-loader"],
      },
      {
        test: /\.(ts|tsx)$/,
        exclude: /node_modules/,
        use: ["ts-loader"],
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".jsx", ".js", ".json"],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "hello world",
      template: path.join(__dirname, "templates", "index.ejs"),
    }),
  ],
};

module.exports = config;
