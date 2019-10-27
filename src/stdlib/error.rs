#[derive(Debug)]
pub struct EvalError {
    pub code: EvalErrorCode,
    pub message: String,
}

#[derive(Debug)]
pub enum EvalErrorCode {
    /// Data is not a function
    E0001,
    /// Name is not bound
    E0002,
    /// Environment does not exist
    E0003,
    /// Name does not resolve to a function
    E0004,
    /// Cannot unwrap `Cons` field
    E0005,
    /// Missmatch in number of arguments
    E0006,
}
