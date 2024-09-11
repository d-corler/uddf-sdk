use super::structure::InformationAfterDive;

/// Builder for creating `InformationAfterDive` instances.
pub struct InformationAfterDiveBuilder {
    greatest_depth: Option<f64>,
    average_depth: Option<f64>,
    dive_duration: Option<f64>,
}

impl InformationAfterDiveBuilder {
    /// Create a new `InformationAfterDiveBuilder` instance.
    pub fn new() -> Self {
        InformationAfterDiveBuilder {
            greatest_depth: None,
            average_depth: None,
            dive_duration: None,
        }
    }

    /// Set greatest depth.
    pub fn set_greatest_depth(mut self, greatest_depth: f64) -> Self {
        self.greatest_depth = Some(greatest_depth);
        self
    }

    /// Set average depth.
    pub fn set_average_depth(mut self, average_depth: f64) -> Self {
        self.average_depth = Some(average_depth);
        self
    }

    /// Set dive duration.
    pub fn set_dive_duration(mut self, dive_duration: f64) -> Self {
        self.dive_duration = Some(dive_duration);
        self
    }

    /// Build the final `InformationAfterDive` object.
    pub fn build(self) -> Result<InformationAfterDive, &'static str> {
        Ok(InformationAfterDive {
            greatest_depth: self.greatest_depth,
            average_depth: self.average_depth,
            dive_duration: self.dive_duration,
        })
    }
}

impl Default for InformationAfterDiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
