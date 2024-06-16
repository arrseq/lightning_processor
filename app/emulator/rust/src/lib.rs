use neon::prelude::{Context, ModuleContext, NeonResult};

pub mod instruction;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    Ok(())
}