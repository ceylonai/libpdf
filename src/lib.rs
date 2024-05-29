mod extract_text;

use lopdf::Document;
use neon::prelude::*;
use neon::types::buffer::TypedArray;

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("0.2.0"))
}


fn document(mut cx: FunctionContext) -> JsResult<JsString> {
    // Get the buffer argument
    let buf = cx.argument::<JsBuffer>(0)?;

    // Convert buffer to a slice
    let file_buffer = buf.as_slice(&cx);
    let doc = Document::load_mem(file_buffer).expect("Failed to load PDF document");


    // Extract text from the PDF
    match extract_text::pdf2text(doc, true, "") {
        Ok(text) => Ok(cx.string(text)),
        Err(err) => cx.throw_error(format!("Failed to extract text: {}", err)),
    }
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("version", version)?;
    cx.export_function("document", document)?;
    Ok(())
}
