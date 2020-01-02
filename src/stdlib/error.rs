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
    /// Mismatch in number of arguments
    E0006,
    /// Not implemented yet
    E0007,
    /// Unable to eval function
    E0008,
    /// Not a `Name`
    E0009,
    /// Parameter list unparseable
    E0010,
}
