use crate::entities::{first_name::structure::FirstName, last_name::structure::LastName};

use super::structure::Personal;

/// Builder for creating `Personal` instances.
pub struct PersonalBuilder {
    first_name: Option<FirstName>,
    last_name: Option<LastName>,
}

impl PersonalBuilder {
    /// Create a new `PersonalBuilder` instance.
    pub fn new() -> Self {
        PersonalBuilder {
            first_name: None,
            last_name: None,
        }
    }

    /// Set the first name.
    pub fn first_name(mut self, first_name: FirstName) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name.
    pub fn last_name(mut self, last_name: LastName) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Build the final `Personal` object.
    pub fn build(self) -> Result<Personal, &'static str> {
        Ok(Personal {
            first_name: self.first_name,
            last_name: self.last_name,
        })
    }
}

impl Default for PersonalBuilder {
    fn default() -> Self {
        Self::new()
    }
}
