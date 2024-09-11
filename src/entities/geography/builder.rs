use super::structure::Geography;

/// Builder for creating `Geography` instances.
pub struct GeographyBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    location: Option<String>,
}

impl GeographyBuilder {
    /// Create a new `GeographyBuilder` instance.
    pub fn new() -> Self {
        GeographyBuilder {
            latitude: None,
            longitude: None,
            location: None,
        }
    }

    /// Set the latitude.
    pub fn latitude(mut self, latitude: f64) -> Self {
        self.latitude = Some(latitude);
        self
    }

    /// Set the longitude.
    pub fn longitude(mut self, longitude: f64) -> Self {
        self.longitude = Some(longitude);
        self
    }

    /// Set the location.
    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    /// Build the final `Geography` object.
    pub fn build(self) -> Result<Geography, &'static str> {
        // If location is not provided, use the latitude and longitude to create a location string
        let location = self.location.unwrap_or_else(|| {
            format!(
                "{}, {}",
                self.latitude.ok_or("Latitude is required").unwrap_or(0.0),
                self.longitude.ok_or("Longitude is required").unwrap_or(0.0)
            )
        });

        Ok(Geography {
            latitude: self.latitude,
            longitude: self.longitude,
            location: Some(location),
        })
    }
}

impl Default for GeographyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
