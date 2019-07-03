#[derive(Debug)]
pub struct EvalError {
    pub code: EvalErrorCode,
    pub message: String,
}

#[derive(Debug)]
pub enum EvalErrorCode {
    /// Cannot evaluate function as data
    E0001,
    /// Name is not bound
    E0002,
}
