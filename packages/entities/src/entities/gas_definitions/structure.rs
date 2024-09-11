use serde::Serialize;

use crate::entities::mix::structure::Mix;

/// Represents gas definitions.
///
/// https://www.streit.cc/extern/uddf_v321/en/gasdefinitions.html
#[derive(Debug, Serialize)]
pub struct GasDefinitions {
    #[serde(rename = "mix")]
    pub mixes: Vec<Mix>,
}
