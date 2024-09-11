use super::structure::Light;

const MISSING_LIGHT_ID: &str = "Light ID is required";

/// Builder for creating `Light` instances.
pub struct LightBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl LightBuilder {
    /// Create a new `LightBuilder` instance.
    pub fn new() -> Self {
        LightBuilder {
            id: None,
            name: None,
        }
    }

    /// Set the ID of the light.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Build the final `Light` object.
    pub fn build(self) -> Result<Light, &'static str> {
        Ok(Light {
            id: self.id.ok_or(MISSING_LIGHT_ID)?,
            name: self.name,
        })
    }
}

impl Default for LightBuilder {
    fn default() -> Self {
        Self::new()
    }
}
