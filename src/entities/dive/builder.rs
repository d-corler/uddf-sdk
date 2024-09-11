use crate::entities::{
    information_after_dive::structure::InformationAfterDive,
    information_before_dive::structure::InformationBeforeDive, samples::structure::Samples,
    tank_data::structure::TankData,
};

use super::structure::Dive;

const MISSING_DIVE_ID: &str = "Dive ID is required";
const MISSING_DIVE_INFORMATION_BEFORE_DIVE: &str = "Dive information before dive is required";
const MISSING_DIVE_INFORMATION_AFTER_DIVE: &str = "Dive information after dive is required";

/// Builder for creating `Dive` instances.
pub struct DiveBuilder {
    id: Option<String>,
    information_before_dive: Option<InformationBeforeDive>,
    tank_data: Option<TankData>,
    samples: Vec<Samples>,
    information_after_dive: Option<InformationAfterDive>,
}

impl DiveBuilder {
    /// Create a new `DiveBuilder` instance.
    pub fn new() -> Self {
        DiveBuilder {
            id: None,
            information_before_dive: None,
            tank_data: None,
            samples: Vec::new(),
            information_after_dive: None,
        }
    }

    /// Set the ID of the dive.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the information before the dive.
    pub fn information_before_dive(
        mut self,
        information_before_dive: InformationBeforeDive,
    ) -> Self {
        self.information_before_dive = Some(information_before_dive);
        self
    }

    /// Set the tank data.
    pub fn tank_data(mut self, tank_data: TankData) -> Self {
        self.tank_data = Some(tank_data);
        self
    }

    /// Add a sample.
    pub fn add_sample(mut self, sample: Samples) -> Self {
        self.samples.push(sample);
        self
    }

    /// Set the information after the dive.
    pub fn information_after_dive(mut self, information_after_dive: InformationAfterDive) -> Self {
        self.information_after_dive = Some(information_after_dive);
        self
    }

    /// Build the final `Dive` object.
    pub fn build(self) -> Result<Dive, &'static str> {
        Ok(Dive {
            id: self.id.ok_or(MISSING_DIVE_ID)?,
            information_before_dive: self
                .information_before_dive
                .ok_or(MISSING_DIVE_INFORMATION_BEFORE_DIVE)?,
            tank_data: self.tank_data,
            samples: self.samples,
            information_after_dive: self
                .information_after_dive
                .ok_or(MISSING_DIVE_INFORMATION_AFTER_DIVE)?,
        })
    }
}

impl Default for DiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
