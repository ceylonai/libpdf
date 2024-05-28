use neon::prelude::*;
use neon::types::buffer::TypedArray;

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("0.0.1"))
}


fn document(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut buf = cx.argument::<JsBuffer>(0)?;
    let file_buffer = buf.as_mut_slice(&mut cx) as &[u8];
    let text = pdf_extract::extract_text_from_mem(file_buffer).unwrap();
    Ok(cx.string(text))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("version", version)?;
    cx.export_function("document", document)?;
    Ok(())
}
