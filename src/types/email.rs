use std::sync::OnceLock;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email(String);

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        let re = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        });

        if re.is_match(&email) {
            Ok(Self(email))
        } else {
            Err(EmailError::InvalidEmail)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}