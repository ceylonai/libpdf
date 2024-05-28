# docreader

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing docreader

Installing docreader requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building docreader

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

## Exploring docreader

After building docreader, you can explore its exports at the Node REPL:

```sh
$ npm install
$ node
> const pdfFile = require("fs").readFileSync("Dewmal Resume.pdf")
> require("./libpdf.node").document(pdfFile)
"<PDF Content Will Print Here>"
```

