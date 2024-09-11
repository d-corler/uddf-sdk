use crate::entities::waypoint::structure::Waypoint;

use super::structure::Samples;

/// Builder for creating `Samples` instances.
pub struct SamplesBuilder {
    waypoints: Vec<Waypoint>,
}

impl SamplesBuilder {
    /// Create a new `SamplesBuilder` instance.
    pub fn new() -> Self {
        SamplesBuilder {
            waypoints: Vec::new(),
        }
    }

    /// Add a waypoint.
    pub fn add_waypoint(mut self, waypoint: Waypoint) -> Self {
        self.waypoints.push(waypoint);
        self
    }

    /// Build the final `Samples` object.
    pub fn build(self) -> Result<Samples, &'static str> {
        Ok(Samples {
            waypoints: self.waypoints,
        })
    }
}

impl Default for SamplesBuilder {
    fn default() -> Self {
        Self::new()
    }
}
