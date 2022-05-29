use hf_ctx::abstract_model::*;
use hf_ctx::arch::*;

fn main() {
    let mut ctx: Context = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    ctx.save_context();
    println!("Hello, world!");
    println!("ctx: {:?}", ctx);
    ctx.apply_context()
}
