use super::vm::VMError;

#[cfg(feature = "vm-debug")]
use backtrace::Backtrace;

#[derive(Debug)]
pub struct EvalError {
    pub code: EvalErrorCode,
    pub message: String,
    #[cfg(feature = "vm-debug")]
    pub backtrace: Backtrace,
}

impl EvalError {
    pub fn new<T>(code: EvalErrorCode, message: T) -> EvalError
    where
        T: Into<String>,
    {
        EvalError {
            code,
            message: message.into(),
            #[cfg(feature = "vm-debug")]
            backtrace: Backtrace::new_unresolved(),
        }
    }
}

impl From<VMError> for EvalError {
    fn from(error: VMError) -> EvalError {
        EvalError {
            message: error.message().into(),
            code: EvalErrorCode::E0011,
            #[cfg(feature = "vm-debug")]
            backtrace: error.backtrace_own(),
        }
    }
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
    /// VMError
    E0011,
    /// Incompatible type
    E0012,
}

pub fn mismatch_arguments(method: &str, expected: usize, found: usize) -> EvalError {
    EvalError {
        code: EvalErrorCode::E0006,
        message: format!(
            "Wrong amount of arguments for '{}'. Expexted {}, found {}",
            method, expected, found
        ),
        #[cfg(feature = "vm-debug")]
        backtrace: Backtrace::new_unresolved(),
    }
}

pub fn unparseable_arguments(method: &str) -> EvalError {
    EvalError {
        code: EvalErrorCode::E0010,
        message: format!("Could not parse arguments for '{}'", method),
        #[cfg(feature = "vm-debug")]
        backtrace: Backtrace::new_unresolved(),
    }
}
