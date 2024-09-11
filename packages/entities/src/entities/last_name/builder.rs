use super::structure::LastName;

/// Builder for creating `LastName` instances.
pub struct LastNameBuilder {
    value: Option<String>,
}

impl LastNameBuilder {
    /// Create a new `LastNameBuilder` instance.
    pub fn new() -> Self {
        LastNameBuilder { value: None }
    }

    /// Set the value.
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Build the final `LastName` object.
    pub fn build(self) -> Result<LastName, &'static str> {
        Ok(LastName { value: self.value })
    }
}

impl Default for LastNameBuilder {
    fn default() -> Self {
        Self::new()
    }
}
