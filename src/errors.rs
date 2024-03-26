use pyo3::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[pyclass]
pub enum AuditTokenError {
    TooLong { token: String },
    Disallowed { token: String },
}

impl std::fmt::Display for AuditTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuditTokenError::TooLong { token } => write!(f, "Token '{}' is too long", token),
            AuditTokenError::Disallowed { token } => write!(f, "Token '{}' is not allowed", token),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuditError {
    token_errors: Vec<AuditTokenError>,
}

impl AuditError {
    pub fn new(token_errors: Vec<AuditTokenError>) -> Self {
        Self { token_errors }
    }
}

impl std::convert::From<AuditError> for PyErr {
    fn from(err: AuditError) -> PyErr {
        let message = "Errors found while auditing text";
        pyo3::exceptions::PyException::new_err(format!("{}\n{}", message, err))
    }
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.token_errors {
            write!(f, "- {}\n", error)?;
        }
        Ok(())
    }
}

impl std::error::Error for AuditError {}
