use super::structure::FirstName;

/// Builder for creating `FirstName` instances.
pub struct FirstNameBuilder {
    value: Option<String>,
}

impl FirstNameBuilder {
    /// Create a new `FirstNameBuilder` instance.
    pub fn new() -> Self {
        FirstNameBuilder { value: None }
    }

    /// Set the value.
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Build the final `FirstName` object.
    pub fn build(self) -> Result<FirstName, &'static str> {
        Ok(FirstName { value: self.value })
    }
}

impl Default for FirstNameBuilder {
    fn default() -> Self {
        Self::new()
    }
}
