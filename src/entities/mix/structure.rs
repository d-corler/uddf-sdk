use serde::Serialize;

/// Represents a mix.
///
/// https://www.streit.cc/extern/uddf_v321/en/mix.html
#[derive(Debug, Serialize)]
pub struct Mix {
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    // Fraction of oxygen in the mix, in the range of 0.0 to 1.0.
    #[serde(rename = "o2", skip_serializing_if = "Option::is_none")]
    pub oxygen: Option<f64>,
    // Fraction of helium in the mix, in the range of 0.0 to 1.0.
    #[serde(rename = "he", skip_serializing_if = "Option::is_none")]
    pub helium: Option<f64>,
}
