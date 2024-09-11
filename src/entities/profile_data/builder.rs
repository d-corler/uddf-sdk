use crate::entities::repetition_group::structure::RepetitionGroup;

use super::structure::ProfileData;

/// Builder for creating `ProfileData` instances.
pub struct ProfileDataBuilder {
    repetition_groups: Vec<RepetitionGroup>,
}

impl ProfileDataBuilder {
    /// Create a new `ProfileDataBuilder` instance.
    pub fn new() -> Self {
        ProfileDataBuilder {
            repetition_groups: Vec::new(),
        }
    }

    /// Add a repetition group.
    pub fn add_repetition_group(mut self, repetition_group: RepetitionGroup) -> Self {
        self.repetition_groups.push(repetition_group);
        self
    }

    /// Build the final `ProfileData` object.
    pub fn build(self) -> Result<ProfileData, &'static str> {
        Ok(ProfileData {
            repetition_groups: self.repetition_groups,
        })
    }
}

impl Default for ProfileDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
