use super::structure::Contact;

/// Builder for creating `Contact` instances.
pub struct ContactBuilder {
    homepage: Option<String>,
}

impl ContactBuilder {
    /// Create a new `ContactBuilder` instance.
    pub fn new() -> Self {
        ContactBuilder { homepage: None }
    }

    /// Set the homepage.
    pub fn homepage(mut self, homepage: String) -> Self {
        self.homepage = Some(homepage);
        self
    }
    /// Build the final `Contact` object.
    pub fn build(self) -> Result<Contact, &'static str> {
        Ok(Contact {
            homepage: self.homepage,
        })
    }
}

impl Default for ContactBuilder {
    fn default() -> Self {
        Self::new()
    }
}
