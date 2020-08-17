use crate::SyntaxError;
use serde_json::Error;

impl From<Error> for SyntaxError {
    fn from(value: Error) -> Self {
        SyntaxError::new(value.to_string())
    }
}
