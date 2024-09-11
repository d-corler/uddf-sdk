use crate::entities::{dive_base::structure::DiveBase, site::structure::Site};

use super::structure::DiveSite;

const MISSING_DIVE_SITE_DIVE_BASE: &str = "Dive site dive base is required";

/// Builder for creating `DiveSite` instances.
pub struct DiveSiteBuilder {
    dive_base: Option<DiveBase>,
    site: Option<Site>,
}

impl DiveSiteBuilder {
    /// Create a new `DiveSiteBuilder` instance.
    pub fn new() -> Self {
        DiveSiteBuilder {
            dive_base: None,
            site: None,
        }
    }

    /// Set the dive base.
    pub fn dive_base(mut self, dive_base: DiveBase) -> Self {
        self.dive_base = Some(dive_base);
        self
    }

    /// Set the site.
    pub fn site(mut self, site: Site) -> Self {
        self.site = Some(site);
        self
    }

    /// Build the final `DiveSite` object.
    pub fn build(self) -> Result<DiveSite, &'static str> {
        Ok(DiveSite {
            dive_base: self.dive_base.ok_or(MISSING_DIVE_SITE_DIVE_BASE)?,
            site: self.site,
        })
    }
}

impl Default for DiveSiteBuilder {
    fn default() -> Self {
        Self::new()
    }
}
