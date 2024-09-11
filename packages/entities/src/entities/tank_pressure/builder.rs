use super::structure::TankPressure;

const MISSING_TANK_PRESSURE_VALUE: &str = "Tank pressure value is required";

/// Builder for creating `TankPressure` instances.
pub struct TankPressureBuilder {
    value: Option<f64>,
}

impl TankPressureBuilder {
    /// Create a new `TankPressureBuilder` instance.
    pub fn new() -> Self {
        TankPressureBuilder { value: None }
    }

    /// Set the value of the tank pressure.
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Build the final `TankPressure` object.
    pub fn build(self) -> Result<TankPressure, &'static str> {
        Ok(TankPressure {
            value: self.value.ok_or(MISSING_TANK_PRESSURE_VALUE)?,
        })
    }
}

impl Default for TankPressureBuilder {
    fn default() -> Self {
        Self::new()
    }
}
