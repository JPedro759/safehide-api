pub struct Email(String);

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        if email.contains("@") && email.contains(".com") {
            Ok(Self(email))
        } else {
            Err(EmailError::InvalidEmail)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}