# LibPdf

**LibPdf** is a fast and efficient Node.js library for converting PDF files to text. This open-source project aims to simplify the process of extracting text content from PDFs, making it easier for developers to work with PDF data in their applications.

Features
- Fast PDF to text conversion
- Easy-to-use API

## Installing libpdf

Installing docreader requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building libpdf

If you have already installed the project and only want to run the build, run:


## Exploring libpdf

After building libpdf, you can explore its exports at the Node REPL:
### Using Node Shell
```sh
$ npm install
$ node
> const pdfFile = require("fs").readFileSync("Dewmal Resume.pdf")
> require("./libpdf.node").document(pdfFile)
"<PDF Content Will Print Here>"
```
### Using javascript file
create file `index.js` with bellow content.
```javascript
const file = require('fs').readFileSync("Dewmal Resume.pdf");
const doc = require('libpdf').document(file);
console.log(doc);
```
then run
```shell
node index.js
```

# Future Steps (TODO)
- Add support for extracting text from specific pages
- Improve text extraction accuracy for complex PDFs
- Implement batch processing for multiple PDFs
- Add CLI support for direct command-line usage
- Create detailed documentation and examples

# Contributing
We welcome contributions to improve LibPdf! Feel free to submit issues and pull requests on our GitHub repository.

# License
This project is licensed under the Apache-2.0 license.