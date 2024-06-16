use neon::prelude::{Context, FunctionContext, JsNumber, JsResult, ModuleContext, NeonResult};

fn get(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(10f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get)?;
    Ok(())
}