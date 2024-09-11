use crate::entities::geography::structure::Geography;

use super::structure::Site;

const MISSING_SITE_ID: &str = "Site ID is required";

/// Builder for creating `Site` instances.
pub struct SiteBuilder {
    pub id: Option<String>,
    pub name: Option<String>,
    pub geography: Option<Geography>,
}

impl SiteBuilder {
    /// Create a new `SiteBuilder` instance.
    pub fn new() -> Self {
        SiteBuilder {
            id: None,
            name: None,
            geography: None,
        }
    }

    /// Set the ID of the site.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the geography.
    pub fn geography(mut self, geography: Geography) -> Self {
        self.geography = Some(geography);
        self
    }

    /// Build the final `Site` object.
    pub fn build(self) -> Result<Site, &'static str> {
        Ok(Site {
            id: self.id.ok_or(MISSING_SITE_ID)?,
            name: self.name,
            geography: self.geography,
        })
    }
}

impl Default for SiteBuilder {
    fn default() -> Self {
        Self::new()
    }
}
