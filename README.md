# LibPdf

**LibPdf** is a fast and efficient Node.js library for converting PDF files to text. This open-source project aims to
simplify the process of extracting text content from PDFs, making it easier for developers to work with PDF data in
their applications.

Features

- Fast PDF to text conversion
- Easy-to-use API

# Installing libpdf

To install `libpdf` and its dependencies, ensure you have a supported version of Node installed. You can install the
project with npm. In the project directory, run:

```bash
$ npm install libpdf --save
```

This command fully installs the project, including installing any dependencies.

# Building libpdf

To build `libpdf`, you need to have Rust installed. If you have already installed the project and only want to run the
build, use:

```bash
$ npm run build
```

# Exploring libpdf

After building `libpdf`, you can explore its exports at the Node REPL.

## Using Node Shell

1. Install `libpdf`:
   ```bash
   $ npm install libpdf --save
   ```

2. Open Node REPL:
   ```bash
   $ node
   ```

3. Execute the following commands:
   ```javascript
   > const pdfFile = require("fs").readFileSync("doc.pdf")
   > const doc = require("libpdf").document(pdfFile);
   > console.log(doc);
   ```

## Using a JavaScript file

1. Create a file named `index.ts` with the following content:

   ```javascript
   const pdfFile = require('fs').readFileSync("doc.pdf");
   const doc = require('libpdf').document(pdfFile);
   console.log(doc);
   ```

2. Run the file with Node:

   ```bash
   $ node index.ts
   ```

This setup ensures you can easily install, build, and explore the capabilities of `libpdf`.

# Benchmark Result

![Benchmarking PDF Libraries.png](./benchmark/BenchmarkingV0.2.0.png)

## conclusion

1. **Best for Small and Medium PDFs**: `libPdf` consistently performs the fastest for small and medium PDF files,
   showing significant speed advantages over `pdf-lib` and `pdf-parse`.

2. **Balanced Performance**: `pdf-parse` offers a middle-ground performance across all file sizes but is generally
   slower than `libPdf` for smaller files and `pdf-lib` for medium files.

3. **Inefficiency with Complex PDFs**: `libPdf` shows a notable drop in performance with complex PDFs, taking
   significantly longer compared to `pdf-parse` and `pdf-lib`.

4. **Library Efficiency**: `pdf-lib` excels with small and medium PDFs but struggles significantly with large and
   complex documents, making it less suitable for those cases.

# Future Steps (TODO)

- [x] Run Benchmark
- [ ] Add support for extracting text from specific pages
- [ ] Improve text extraction accuracy for complex PDFs
- [ ] Implement batch processing for multiple PDFs
- [ ] Add CLI support for direct command-line usage
- [ ] Create detailed documentation and examples

# Known issues

- Not supported for Identity-H encoding

# Contributing

We welcome contributions to improve LibPdf! Feel free to submit issues and pull requests on our GitHub repository.

# License

This project is licensed under the Apache-2.0 license.

https://x.com/CeylonAIDev