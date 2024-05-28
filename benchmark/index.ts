const fs = require('fs');
const {performance} = require('perf_hooks');
const pdfParse = require('pdf-parse');
const pdfLib = require('pdf-lib');
const PDFParser = require('pdf2json');
const libPdf = require('../libpdf.node'); // Replace with your library's import
const createCsvWriter = require('csv-writer').createObjectCsvWriter;

const files = ['./docs/small.pdf', './docs/medium.pdf', './docs/large.pdf', './docs/complex.pdf'];
const csvWriter = createCsvWriter({
    path: 'benchmark_results.csv',
    header: [
        { id: 'file', title: 'File' },
        { id: 'library', title: 'Library' },
        { id: 'time', title: 'Time (ms)' },
    ],
    append: true, // Ensure results are appended
});

async function benchmarkPdfParse(file: string) {
    const data = fs.readFileSync(file);
    const start = performance.now();
    await pdfParse(data);
    const end = performance.now();
    return end - start;
}

async function benchmarkPdfLib(file: string) {
    const data = fs.readFileSync(`${file}`);
    const start = performance.now();
    await pdfLib.PDFDocument.load(data, {
        ignoreEncryption: true
    });
    const end = performance.now();
    return end - start;
}

async function benchmarkPdf2Json(file: string) {
    const pdfParser = new PDFParser();
    const data = fs.readFileSync(file);
    const start = performance.now();
    pdfParser.parseBuffer(data);
    const end = performance.now();
    return new Promise((resolve) => {
        pdfParser.on('pdfParser_dataReady', () => {
            resolve(end - start);
        });
    });
}

async function benchmarkLibPdf(file: string) {
    const data = fs.readFileSync(file);
    const start = performance.now();
    await libPdf.document(data);
    const end = performance.now();
    return end - start;
}

async function runBenchmarks() {
    const results = [];
    for (const file of files) {
        console.log(`Benchmarking ${file}...`);
        const timePdfParse = await benchmarkPdfParse(file);
        const timePdfLib = await benchmarkPdfLib(file);
        const timeLibPdf = await benchmarkLibPdf(file);

        // console.log(`${file} - pdf-parse: ${timePdfParse} ms`);
        // console.log(`${file} - pdf-lib: ${timePdfLib} ms`);
        // console.log(`${file} - libPdf: ${timeLibPdf} ms`);
        results.push({file, library: 'pdf-parse', time: timePdfParse});
        results.push({file, library: 'pdf-lib', time: timePdfLib});
        results.push({file, library: 'libPdf', time: timeLibPdf});

    }
    await csvWriter.writeRecords(results);
    console.log('Results appended to benchmark_results.csv');
}

runBenchmarks().catch(console.error);
