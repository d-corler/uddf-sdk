use super::structure::DiveBase;

const MISSING_DIVE_BASE_ID: &str = "Dive base ID is required";

/// Builder for creating `DiveBase` instances.
pub struct DiveBaseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl DiveBaseBuilder {
    /// Create a new `DiveBaseBuilder` instance.
    pub fn new() -> Self {
        DiveBaseBuilder {
            id: None,
            name: None,
        }
    }

    /// Set the ID of the manufacturer.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Build the final `DiveBase` object.
    pub fn build(self) -> Result<DiveBase, &'static str> {
        Ok(DiveBase {
            id: self.id.ok_or(MISSING_DIVE_BASE_ID)?,
            name: self.name,
        })
    }
}

impl Default for DiveBaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
