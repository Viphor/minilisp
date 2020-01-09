use super::super::stdlib::error::EvalError;

#[derive(Debug)]
pub struct VMError {
    message: String,
}

impl VMError {
    pub fn error<T>(message: T) -> VMError
    where
        T: Into<String>,
    {
        VMError {
            message: message.into(),
        }
    }

    pub fn no_stack_frame() -> VMError {
        VMError::error("No stack frame exists")
    }

    pub fn not_a_function() -> VMError {
        VMError::error("The first item in the register is not a function")
    }

    pub fn eval_error(error: EvalError) -> VMError {
        VMError::error(format!(
            "Error occured while calling function. Reason: {}",
            error.message
        ))
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
