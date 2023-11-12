use crate::traits::handler::Handler;

pub struct ErrorHandler {
    reason: String,
}

impl ErrorHandler {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl Handler for ErrorHandler {
    fn execute(&self) {
        eprintln!("{}", self.reason);
    }
}
