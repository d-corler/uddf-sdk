use super::structure::Mix;

const MISSING_MIX_ID: &str = "Mix ID is required";
const MISSING_MIX_NAME: &str = "Mix name is required";

/// Builder for creating `Mix` instances.
pub struct MixBuilder {
    id: Option<String>,
    name: Option<String>,
    oxygen: Option<f64>,
    helium: Option<f64>,
}

impl MixBuilder {
    /// Create a new `MixBuilder` instance.
    pub fn new() -> Self {
        MixBuilder {
            id: None,
            name: None,
            oxygen: None,
            helium: None,
        }
    }

    /// Set the ID of the mix.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the oxygen fraction.
    pub fn oxygen(mut self, oxygen: f64) -> Self {
        self.oxygen = Some(oxygen);
        self
    }

    /// Set the helium fraction.
    pub fn helium(mut self, helium: f64) -> Self {
        self.helium = Some(helium);
        self
    }

    /// Build the final `Mix` object.
    pub fn build(self) -> Result<Mix, &'static str> {
        Ok(Mix {
            id: self.id.ok_or(MISSING_MIX_ID)?,
            name: self.name.ok_or(MISSING_MIX_NAME)?,
            oxygen: self.oxygen,
            helium: self.helium,
        })
    }
}

impl Default for MixBuilder {
    fn default() -> Self {
        Self::new()
    }
}
