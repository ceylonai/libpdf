# benchmark

![Benchmarking PDF Libraries.png](Benchmarking%20PDF%20Libraries.png)

Based on the benchmarking results, here are the conclusions in point form:

- **Performance Variability Across Libraries**:
    - `libPdf` consistently showed the fastest performance for small and medium PDF files, but its performance significantly decreased with larger and more complex PDFs.
    - `pdf-lib` had the lowest processing time for small and medium files but took considerably longer for large and complex PDFs.
    - `pdf-parse` showed relatively consistent performance but was generally slower compared to `libPdf` and `pdf-lib` for small and medium files.

- **Small PDFs**:
    - `libPdf`: 2.00 ms
    - `pdf-lib`: 6.40 ms
    - `pdf-parse`: 65.56 ms
    - `libPdf` performed the best, followed by `pdf-lib`, with `pdf-parse` being the slowest.

- **Medium PDFs**:
    - `pdf-lib`: 4.35 ms
    - `pdf-parse`: 29.16 ms
    - `libPdf`: 36.42 ms
    - `pdf-lib` was the fastest, with `pdf-parse` being significantly faster than `libPdf`.

- **Large PDFs**:
    - `libPdf`: 145.73 ms
    - `pdf-parse`: 216.32 ms
    - `pdf-lib`: 612.95 ms
    - `libPdf` had the best performance, followed by `pdf-parse`, with `pdf-lib` being the slowest by a large margin.

- **Complex PDFs**:
    - `pdf-parse`: 447.64 ms
    - `pdf-lib`: 548.89 ms
    - `libPdf`: 1848.26 ms
    - `pdf-parse` showed the best performance, followed by `pdf-lib`, with `libPdf` taking the longest time.

- **Overall Observations**:
    - `libPdf` excels with smaller and simpler PDFs but struggles with more complex documents.
    - `pdf-lib` is efficient for smaller files but not suitable for large or complex PDFs.
    - `pdf-parse` offers a balanced performance but is not the fastest for any specific type of PDF.

These conclusions can help guide the selection of a PDF processing library based on the specific requirements of file size and complexity.