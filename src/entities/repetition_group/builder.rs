use crate::entities::dive::structure::Dive;

use super::structure::RepetitionGroup;

const MISSING_REPETITION_GROUP_ID: &str = "Repetition group ID is required";

/// Builder for creating `RepetitionGroup` instances.
pub struct RepetitionGroupBuilder {
    id: Option<String>,
    dives: Vec<Dive>,
}

impl RepetitionGroupBuilder {
    /// Create a new `RepetitionGroupBuilder` instance.
    pub fn new() -> Self {
        RepetitionGroupBuilder {
            id: None,
            dives: Vec::new(),
        }
    }

    /// Set the ID of the repetition group.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Add a dive.
    pub fn add_dive(mut self, dive: Dive) -> Self {
        self.dives.push(dive);
        self
    }

    /// Build the final `RepetitionGroup` object.
    pub fn build(self) -> Result<RepetitionGroup, &'static str> {
        Ok(RepetitionGroup {
            id: self.id.ok_or(MISSING_REPETITION_GROUP_ID)?,
            dives: self.dives,
        })
    }
}

impl Default for RepetitionGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}
