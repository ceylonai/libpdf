use neon::prelude::*;
use neon::types::buffer::TypedArray;

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("0.1.6"))
}


fn document(mut cx: FunctionContext) -> JsResult<JsString> {
    // Get the buffer argument
    let buf = cx.argument::<JsBuffer>(0)?;

    // Convert buffer to a slice
    let file_buffer = buf.as_slice(&cx);

    // Extract text from the PDF
    match pdf_extract::extract_text_from_mem(file_buffer) {
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
