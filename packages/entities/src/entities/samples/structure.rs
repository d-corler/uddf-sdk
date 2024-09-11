use serde::Serialize;

use crate::entities::waypoint::structure::Waypoint;

/// Represents a collection of samples.
///
/// https://www.streit.cc/extern/uddf_v321/en/samples.html
#[derive(Debug, Serialize)]
pub struct Samples {
    #[serde(rename = "waypoint")]
    pub waypoints: Vec<Waypoint>,
}
