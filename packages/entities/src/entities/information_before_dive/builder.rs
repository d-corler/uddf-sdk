use chrono::{DateTime, Utc};

use crate::entities::link::structure::Link;

use super::structure::InformationBeforeDive;

const MISSING_INFO_BEFORE_DIVE_DIVE_NUMBER: &str = "Dive number is required";
const MISSING_INFO_BEFORE_DIVE_DATE_TIME: &str = "Date time is required";

/// Builder for creating `InformationBeforeDive` instances.
pub struct InformationBeforeDiveBuilder {
    link: Vec<Link>,
    dive_number: Option<u64>,
    date_time: Option<DateTime<Utc>>,
}

impl InformationBeforeDiveBuilder {
    /// Create a new `InformationBeforeDiveBuilder` instance.
    pub fn new() -> Self {
        InformationBeforeDiveBuilder {
            link: Vec::new(),
            dive_number: None,
            date_time: None,
        }
    }

    /// Add a link.
    pub fn add_link(mut self, link: Link) -> Self {
        self.link.push(link);
        self
    }

    /// Set the dive number.
    pub fn dive_number(mut self, dive_number: u64) -> Self {
        self.dive_number = Some(dive_number);
        self
    }

    /// Set the date time.
    pub fn date_time(mut self, date_time: DateTime<Utc>) -> Self {
        self.date_time = Some(date_time);
        self
    }

    /// Build the final `InformationBeforeDive` object.
    pub fn build(self) -> Result<InformationBeforeDive, &'static str> {
        Ok(InformationBeforeDive {
            link: self.link,
            dive_number: self
                .dive_number
                .ok_or(MISSING_INFO_BEFORE_DIVE_DIVE_NUMBER)?,
            date_time: self.date_time.ok_or(MISSING_INFO_BEFORE_DIVE_DATE_TIME)?,
        })
    }
}

impl Default for InformationBeforeDiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
