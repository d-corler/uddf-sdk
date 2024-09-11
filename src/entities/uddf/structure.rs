use serde::Serialize;

use crate::entities::{
    dive_site::structure::DiveSite, diver::structure::Diver,
    gas_definitions::structure::GasDefinitions, generator::structure::Generator,
    maker::structure::Maker, profile_data::structure::ProfileData,
};

/// Represents an UDDF.
///
/// https://www.streit.cc/extern/uddf_v321/en/uddf.html
#[derive(Debug, Serialize)]
pub struct Uddf {
    #[serde(rename = "@version")]
    /// The version of the UDDF specification.
    pub version: String,
    #[serde(rename = "@xmlns")]
    /// The XML namespace of the UDDF specification.
    pub xmlns: String,
    /// The generator of the file.
    pub generator: Generator,
    /// The maker set.
    pub maker: Maker,
    /// The diver.
    pub diver: Diver,
    /// The dive site.
    #[serde(rename = "divesite")]
    pub dive_site: DiveSite,
    /// The gaz definitions.
    #[serde(rename = "gasdefinitions")]
    pub gas_definitions: GasDefinitions,
    /// The profile data.
    #[serde(rename = "profiledata")]
    pub profile_data: ProfileData,
}
