use crate::entities::{
    dive_site::structure::DiveSite, diver::structure::Diver,
    gas_definitions::structure::GasDefinitions, generator::structure::Generator,
    maker::structure::Maker, profile_data::structure::ProfileData,
};

use super::structure::Uddf;

const MISSING_UDDF_VERSION: &str = "UDDF version is required";
const MISSING_UDDF_XMLNS: &str = "UDDF xmlns is required";
const MISSING_UDDF_GENERATOR: &str = "UDDF generator is required";
const MISSING_UDDF_MAKER: &str = "UDDF maker is required";
const MISSING_UDDF_DIVER: &str = "UDDF diver is required";
const MISSING_UDDF_DIVE_SITE: &str = "UDDF dive site is required";
const MISSING_UDDF_GAS_DEFINITIONS: &str = "UDDF gas definitions are required";
const MISSING_UDDF_PROFILE_DATA: &str = "UDDF profile data is required";

/// Builder for creating `Uddf` instances.
pub struct UddfBuilder {
    version: Option<String>,
    xmlns: Option<String>,
    generator: Option<Generator>,
    maker: Option<Maker>,
    diver: Option<Diver>,
    dive_site: Option<DiveSite>,
    gas_definitions: Option<GasDefinitions>,
    profile_data: Option<ProfileData>,
}

impl UddfBuilder {
    /// Create a new `UddfBuilder` instance.
    pub fn new() -> Self {
        UddfBuilder {
            version: None,
            xmlns: None,
            generator: None,
            maker: None,
            diver: None,
            dive_site: None,
            gas_definitions: None,
            profile_data: None,
        }
    }

    /// Set the version.
    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the XMLNS.
    pub fn xmlns(mut self, xmlns: String) -> Self {
        self.xmlns = Some(xmlns);
        self
    }

    /// Set the generator.
    pub fn generator(mut self, generator: Generator) -> Self {
        self.generator = Some(generator);
        self
    }

    /// Set the maker.
    pub fn maker(mut self, maker: Maker) -> Self {
        self.maker = Some(maker);
        self
    }

    /// Set the diver.
    pub fn diver(mut self, diver: Diver) -> Self {
        self.diver = Some(diver);
        self
    }

    /// Set the dive site.
    pub fn dive_site(mut self, dive_site: DiveSite) -> Self {
        self.dive_site = Some(dive_site);
        self
    }

    /// Set the gas definitions.
    pub fn gas_definitions(mut self, gas_definitions: GasDefinitions) -> Self {
        self.gas_definitions = Some(gas_definitions);
        self
    }

    /// Set the profile data.
    pub fn profile_data(mut self, profile_data: ProfileData) -> Self {
        self.profile_data = Some(profile_data);
        self
    }

    /// Build the final `Uddf` object.
    pub fn build(self) -> Result<Uddf, &'static str> {
        Ok(Uddf {
            version: self.version.ok_or(MISSING_UDDF_VERSION)?,
            xmlns: self.xmlns.ok_or(MISSING_UDDF_XMLNS)?,
            generator: self.generator.ok_or(MISSING_UDDF_GENERATOR)?,
            maker: self.maker.ok_or(MISSING_UDDF_MAKER)?,
            diver: self.diver.ok_or(MISSING_UDDF_DIVER)?,
            dive_site: self.dive_site.ok_or(MISSING_UDDF_DIVE_SITE)?,
            gas_definitions: self.gas_definitions.ok_or(MISSING_UDDF_GAS_DEFINITIONS)?,
            profile_data: self.profile_data.ok_or(MISSING_UDDF_PROFILE_DATA)?,
        })
    }
}

impl Default for UddfBuilder {
    fn default() -> Self {
        Self::new()
    }
}
