

pub trait ContextInfo {
    fn get_context_size() -> usize;

    fn get_stack_ptr() -> usize;
}

pub trait ContextOp {
    fn save_context(&mut self);

    fn apply_context(&mut self);

    fn return_from_trap(&mut self);
}

pub trait PartialContextOp {
    extern "C" fn save_par_ctx(&mut self);
    extern "C" fn apply_par_ctx(&mut self);
}