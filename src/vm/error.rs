use super::super::stdlib::error::EvalError;

#[cfg(feature = "vm-debug")]
use backtrace::Backtrace;

#[derive(Debug)]
pub struct VMError {
    message: String,
    #[cfg(feature = "vm-debug")]
    backtrace: Backtrace,
}

impl VMError {
    pub fn error<T>(message: T) -> VMError
    where
        T: Into<String>,
    {
        VMError {
            message: message.into(),
            #[cfg(feature = "vm-debug")]
            backtrace: Backtrace::new_unresolved(),
        }
    }

    #[cfg(feature = "vm-debug")]
    pub fn error_with_backtrace<T>(message: T, backtrace: Backtrace) -> VMError
    where
        T: Into<String>,
    {
        VMError {
            message: message.into(),
            backtrace,
        }
    }

    pub fn no_stack_frame() -> VMError {
        VMError::error("No stack frame exists")
    }

    pub fn not_a_function() -> VMError {
        VMError::error("The first item in the register is not a function")
    }

    #[cfg(feature = "vm-debug")]
    pub fn eval_error(error: EvalError) -> VMError {
        VMError::error_with_backtrace(
            format!(
                "Error occured while calling function. Reason: {}",
                error.message
            ),
            error.backtrace,
        )
    }

    #[cfg(not(feature = "vm-debug"))]
    pub fn eval_error(error: EvalError) -> VMError {
        VMError::error(format!(
            "Error occured while calling function. Reason: {}",
            error.message
        ))
    }

    pub fn wrong_parameter_len(expected: usize, found: usize) -> VMError {
        VMError::error(format!(
            "Wrong amount of arguments. Expexted {}, found {}",
            expected, found
        ))
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    #[cfg(feature = "vm-debug")]
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    #[cfg(feature = "vm-debug")]
    pub fn backtrace_mut(&mut self) -> &mut Backtrace {
        &mut self.backtrace
    }

    #[cfg(feature = "vm-debug")]
    pub fn backtrace_own(self) -> Backtrace {
        self.backtrace
    }
}
