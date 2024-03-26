use pyo3::prelude::*;
use rayon::prelude::*;

use crate::errors::{AuditError, AuditTokenError};

#[derive(Clone)]
#[pyclass]
pub struct Auditor {
    disallowed_strings: Vec<String>,
    max_token_length: usize,
}

#[pymethods]
impl Auditor {
    #[new]
    pub fn new(disallowed_strings: Vec<String>, max_token_length: usize) -> Self {
        Self {
            disallowed_strings: disallowed_strings.iter().map(|s| s.to_string()).collect(),
            max_token_length,
        }
    }

    pub fn audit(&self, text: &str) -> Result<(), AuditError> {
        // let mut errors: Vec<AuditTokenError> = Vec::new();
        let errors = text
            .par_split_whitespace()
            .flat_map(|token| self.audit_token(token))
            .collect::<Vec<AuditTokenError>>();
        if errors.is_empty() {
            return Ok(());
        }
        Err(AuditError::new(errors))
    }

    fn audit_token(&self, token: &str) -> Vec<AuditTokenError> {
        let mut token_errors: Vec<AuditTokenError> = Vec::new();
        if token.len() > self.max_token_length {
            token_errors.push(AuditTokenError::TooLong {
                token: token.to_string(),
            });
        }
        if self.disallowed_strings.contains(&token.to_string()) {
            token_errors.push(AuditTokenError::Disallowed {
                token: token.to_string(),
            });
        }
        token_errors
    }
}
