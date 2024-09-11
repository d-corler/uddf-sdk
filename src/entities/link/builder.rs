use super::structure::Link;

const MISSING_REFERENCE: &str = "Link reference is required";

/// Builder for creating `Link` instances.
pub struct LinkBuilder {
    reference: Option<String>,
}

impl LinkBuilder {
    /// Create a new `LinkBuilder` instance.
    pub fn new() -> Self {
        LinkBuilder { reference: None }
    }

    /// Set the reference of the link.
    pub fn reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Build the final `Link` object.
    pub fn build(self) -> Result<Link, &'static str> {
        Ok(Link {
            reference: self.reference.ok_or(MISSING_REFERENCE)?,
        })
    }
}

impl Default for LinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}
