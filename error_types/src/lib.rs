use chrono::prelude::*;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self { form_values: (field_name.to_string(), field_value.to_string()), date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), err: err.to_string() }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("first_name", self.name.to_string(), "Username is empty"));
        }
        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.to_string() , "Password should be at least 8 characters long"));
        }
        let mut numeric = false;
        let mut symbol = false;
        let mut alpha = false;
        for c in self.password.chars() {
            if c.is_numeric() {
                numeric = true
            } else if c.is_alphabetic() {
                alpha = true
            } else {
                symbol = true
            }

        }
        if !numeric || !symbol || !alpha {
            return Err(FormError::new("password", self.password.to_string() , "Password should be a combination of ASCII numbers, letters and symbols"));
        }
        Ok(())
    }
}