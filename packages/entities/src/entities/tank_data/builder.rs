use crate::entities::{link::structure::Link, tank_pressure::structure::TankPressure};

use super::structure::TankData;

/// Builder for creating `TankData` instances.
pub struct TankDataBuilder {
    link: Vec<Link>,
    tank_volume: Option<f64>,
    tank_pressure_begin: Option<TankPressure>,
    tank_pressure_end: Option<TankPressure>,
}

impl TankDataBuilder {
    /// Create a new `TankDataBuilder` instance.
    pub fn new() -> Self {
        TankDataBuilder {
            link: Vec::new(),
            tank_volume: None,
            tank_pressure_begin: None,
            tank_pressure_end: None,
        }
    }

    /// Add a link.
    pub fn add_link(mut self, link: Link) -> Self {
        self.link.push(link);
        self
    }

    /// Set the tank volume.
    pub fn tank_volume(mut self, tank_volume: f64) -> Self {
        self.tank_volume = Some(tank_volume);
        self
    }

    /// Set the tank pressure at the beginning of the dive.
    pub fn tank_pressure_begin(mut self, tank_pressure_begin: TankPressure) -> Self {
        self.tank_pressure_begin = Some(tank_pressure_begin);
        self
    }

    /// Set the tank pressure at the end of the dive.
    pub fn tank_pressure_end(mut self, tank_pressure_end: TankPressure) -> Self {
        self.tank_pressure_end = Some(tank_pressure_end);
        self
    }

    /// Build the final `TankData` object.
    pub fn build(self) -> Result<TankData, &'static str> {
        Ok(TankData {
            link: self.link,
            tank_volume: self.tank_volume,
            tank_pressure_begin: self.tank_pressure_begin,
            tank_pressure_end: self.tank_pressure_end,
        })
    }
}

impl Default for TankDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
