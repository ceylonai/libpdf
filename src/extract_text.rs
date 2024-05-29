use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use clap::Parser;
use lopdf::{Document, Object};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json;
#[cfg(feature = "async")]
use tokio::runtime::Builder;

static IGNORE: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Resources",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "Font",
    "MediaBox",
    "Annot",
];

#[derive(Debug, Deserialize, Serialize)]
struct PdfText {
    text: BTreeMap<u32, Vec<String>>, // Key is page number
    errors: Vec<String>,
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Extract TOC and write to file.",
    arg_required_else_help = true
)]
pub struct Args {
    pub pdf_path: PathBuf,

    /// Optional output directory. If omitted the directory of the PDF file will be used.
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Optional pretty print output.
    #[clap(short, long)]
    pub pretty: bool,

    /// Optional password for encrypted PDFs
    #[clap(long, default_value_t = String::from(""))]
    pub password: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}

fn filter_func(object_id: (u32, u16), object: &mut Object) -> Option<((u32, u16), Object)> {
    if IGNORE.contains(&object.type_name().unwrap_or_default()) {
        return None;
    }
    if let Ok(d) = object.as_dict_mut() {
        d.remove(b"Font");
        d.remove(b"Resources");
        d.remove(b"Producer");
        d.remove(b"ModDate");
        d.remove(b"Creator");
        d.remove(b"ProcSet");
        d.remove(b"XObject");
        d.remove(b"MediaBox");
        d.remove(b"Annots");
        if d.is_empty() {
            return None;
        }
    }
    Some((object_id, object.to_owned()))
}

#[cfg(not(feature = "async"))]
fn load_pdf<P: AsRef<Path>>(path: P) -> Result<Document, Error> {
    Document::load_filtered(path, filter_func).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}

#[cfg(feature = "async")]
fn load_pdf<P: AsRef<Path>>(path: P) -> Result<Document, Error> {
    Ok(
        Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async move {
                Document::load_filtered(path, filter_func)
                    .await
                    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
            })?
    )
}


fn get_pdf_text(doc: &Document) -> Result<PdfText, Error> {
    let mut pdf_text: PdfText = PdfText {
        text: BTreeMap::new(),
        errors: Vec::new(),
    };
    let pages: Vec<Result<(u32, Vec<String>), Error>> = doc
        .get_pages()
        .into_par_iter()
        .map(
            |(page_num, page_id): (u32, (u32, u16))| -> Result<(u32, Vec<String>), Error> {
                let text = doc.extract_text(&[page_num]).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to extract text from page {page_num} id={page_id:?}: {e:?}"),
                    )
                })?;
                Ok((
                    page_num,
                    text.split('\n')
                        .map(|s| s.trim_end().to_string())
                        .collect::<Vec<String>>(),
                ))
            },
        )
        .collect();
    for page in pages {
        match page {
            Ok((page_num, lines)) => {
                pdf_text.text.insert(page_num, lines);
            }
            Err(e) => {
                pdf_text.errors.push(e.to_string());
            }
        }
    }
    Ok(pdf_text)
}

pub fn pdf2text(mut doc: Document, pretty: bool, password: &str) -> Result<String, Error> {
    if doc.is_encrypted() {
        doc.decrypt(password)
            .map_err(|_err|
                Error::new(ErrorKind::InvalidInput, "Failed to decrypt")
            )?;
    }
    let pdf_text = get_pdf_text(&doc)?;
    if !pdf_text.errors.is_empty() {
        eprintln!("Document has {} errors:", pdf_text.errors.len());
        for error in &pdf_text.errors[..10] {
            eprintln!("{error:?}");
        }
    }

    let mut text_value = pdf_text
        .text.clone()
        .into_iter()
        .map(|(page_num, lines)| {
            format!("Page {page_num}:\n{}", lines.join("\n"))
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    let data = match pretty {
        true => text_value,
        false => serde_json::to_string(&pdf_text).unwrap(),
    };
    Ok(data)
}