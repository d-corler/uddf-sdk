use crate::entities::tank_pressure::structure::TankPressure;

use super::structure::Waypoint;

const MISSING_DEPTH: &str = "Depth is required";
const MISSING_DIVE_TIME: &str = "Dive time is required";

/// Builder for creating `Waypoint` instances.
pub struct WaypointBuilder {
    depth: Option<f64>,
    dive_time: Option<u64>,
    tank_pressure: Option<TankPressure>,
    temperature: Option<f64>,
}

impl WaypointBuilder {
    /// Create a new `WaypointBuilder` instance.
    pub fn new() -> Self {
        WaypointBuilder {
            depth: None,
            dive_time: None,
            tank_pressure: None,
            temperature: None,
        }
    }

    /// Set the depth.
    pub fn depth(mut self, depth: f64) -> Self {
        self.depth = Some(depth);
        self
    }

    /// Set the dive time.
    pub fn dive_time(mut self, dive_time: u64) -> Self {
        self.dive_time = Some(dive_time);
        self
    }

    /// Set the tank pressure.
    pub fn tank_pressure(mut self, tank_pressure: TankPressure) -> Self {
        self.tank_pressure = Some(tank_pressure);
        self
    }

    /// Set the temperature.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Build the final `Waypoint` object.
    pub fn build(self) -> Result<Waypoint, &'static str> {
        Ok(Waypoint {
            depth: self.depth.ok_or(MISSING_DEPTH)?,
            dive_time: self.dive_time.ok_or(MISSING_DIVE_TIME)?,
            tank_pressure: self.tank_pressure,
            temperature: self.temperature,
        })
    }
}

impl Default for WaypointBuilder {
    fn default() -> Self {
        Self::new()
    }
}
