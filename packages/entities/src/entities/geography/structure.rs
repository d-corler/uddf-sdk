use serde::Serialize;

/// Represents a geography.
///
/// https://www.streit.cc/extern/uddf_v321/en/geography.html
#[derive(Debug, Serialize)]
pub struct Geography {
    /// Latitude in degrees.
    pub latitude: Option<f64>,
    /// Longitude in degrees.
    pub longitude: Option<f64>,
    /// Concatenation of the latitude and longitude by default.
    pub location: Option<String>,
}
