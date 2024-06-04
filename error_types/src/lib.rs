pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors = Vec::new();
        let mut valid = Vec::new();
        let mut first_name_vid = false;
        if self.first_name.is_empty() {
            first_name_vid = true;
            errors.push("No user name");
        } else {
            valid.push("Valid first name");
        }
        if self.password.len() < 8 {
            errors.push("At least 8 characters");
        }
        if !self.password.chars().any(|c| c.is_digit(10))
            || !self.password.chars().any(|c| c.is_alphabetic())
            || !self.password.chars().any(|c| !c.is_alphanumeric())
        {
            errors.push("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)");
        }
        if errors.is_empty() {
            valid.push("Valid password");
            Ok(valid)
        } else {
            if first_name_vid {
                return Err(FormError::new(
                    "first_name".to_string(),
                    self.first_name.clone(),
                    "No user name".to_string(),
                ));
            }
            Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                errors[0].to_string(),
            ))
        }
    }
}

pub fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}
