use chrono::{DateTime, Utc};
use fitparser::FitDataRecord;
use uddf_sdk_entities::entities::{
    contact::builder::ContactBuilder,
    dive::builder::DiveBuilder,
    dive_base::builder::DiveBaseBuilder,
    dive_computer::{builder::DiveComputerBuilder, structure::DiveComputer},
    dive_site::{builder::DiveSiteBuilder, structure::DiveSite},
    diver::{builder::DiverBuilder, structure::Diver},
    equipment::{builder::EquipmentBuilder, structure::Equipment_},
    first_name::builder::FirstNameBuilder,
    gas_definitions::{builder::GasDefinitionsBuilder, structure::GasDefinitions},
    generator::{
        builder::GeneratorBuilder,
        structure::{Generator, GeneratorType},
    },
    geography::builder::GeographyBuilder,
    information_after_dive::builder::InformationAfterDiveBuilder,
    information_before_dive::builder::InformationBeforeDiveBuilder,
    last_name::builder::LastNameBuilder,
    link::builder::LinkBuilder,
    maker::{builder::MakerBuilder, structure::Maker},
    manufacturer::builder::ManufacturerBuilder,
    mix::builder::MixBuilder,
    owner::builder::OwnerBuilder,
    personal::builder::PersonalBuilder,
    profile_data::{builder::ProfileDataBuilder, structure::ProfileData},
    repetition_group::builder::RepetitionGroupBuilder,
    samples::builder::SamplesBuilder,
    site::builder::SiteBuilder,
    tank_data::builder::TankDataBuilder,
    tank_pressure::builder::TankPressureBuilder,
    uddf::{builder::UddfBuilder, structure::Uddf},
    waypoint::{builder::WaypointBuilder, structure::Waypoint},
};
use uddf_sdk_utils::math::{
    convert_bar_to_pascal, convert_celsius_to_kelvin, semicircles_to_degrees,
};

use crate::constants::{UDDF_VERSION, UDDF_XMLNS};

use super::constants::{
    AVG_DEPTH_TAG, BOTTOM_TIME_TAG, DEPTH_TAG, DIVER_OWNER_ID, DIVE_BASE_ID,
    DIVE_COMPUTER_MANUFACTURER_ID, DIVE_COMPUTER_MANUFACTURER_NAME, DIVE_NUMBER_TAG,
    END_PRESSURE_TAG, GARMIN_PRODUCT_TAG, GENERATOR_MANUFACTURER_HOMEPAGE,
    GENERATOR_MANUFACTURER_ID, GENERATOR_MANUFACTURER_NAME, GENERATOR_NAME, HELIUM_CONTENT_TAG,
    MAX_DEPTH_TAG, OXYGEN_CONTENT_TAG, PRESSURE_TAG, SERIAL_NUMBER_TAG, START_POSITION_LAT_TAG,
    START_POSITION_LONG_TAG, START_PRESSURE_TAG, TEMPERATURE_TAG, TIMESTAMP_TAG,
};

/// Converts Garmin FIT data to UDDF.
pub fn from_garmin(fit_data_record_vec: Vec<FitDataRecord>) -> Result<Uddf, &'static str> {
    let file_id = find_by_kind_in_fit_data_record_vec(
        &fit_data_record_vec,
        fitparser::profile::MesgNum::FileId,
    )?;
    let session = find_by_kind_in_fit_data_record_vec(
        &fit_data_record_vec,
        fitparser::profile::MesgNum::Session,
    )?;
    let dive_gas = find_by_kind_in_fit_data_record_vec(
        &fit_data_record_vec,
        fitparser::profile::MesgNum::DiveGas,
    )?;
    let dive_summary = find_by_kind_in_fit_data_record_vec(
        &fit_data_record_vec,
        fitparser::profile::MesgNum::DiveSummary,
    )?;
    let tank_summary = find_by_kind_in_fit_data_record_vec(
        &fit_data_record_vec,
        fitparser::profile::MesgNum::TankSummary,
    )
    .ok();

    let generator = create_generator()?;

    let maker = create_maker()?;

    let diver = create_diver(&file_id)?;

    let dive_site = create_dive_site(&session)?;

    let gas_definitions = create_gas_definitions(&dive_gas)?;

    let profile_data = create_profile_data(
        &file_id,
        &fit_data_record_vec,
        &dive_summary,
        &dive_site,
        &gas_definitions,
        &tank_summary,
    )?;

    UddfBuilder::new()
        .version(UDDF_VERSION.to_owned())
        .xmlns(UDDF_XMLNS.to_owned())
        .generator(generator)
        .maker(maker)
        .diver(diver)
        .dive_site(dive_site)
        .gas_definitions(gas_definitions)
        .profile_data(profile_data)
        .build()
}

/// Find a FIT data record with a specific kind.
pub fn find_by_kind_in_fit_data_record_vec(
    fit_data_record_vec: &[FitDataRecord],
    mesg_num: fitparser::profile::MesgNum,
) -> Result<FitDataRecord, &'static str> {
    fit_data_record_vec
        .iter()
        .find(|record| record.kind() == mesg_num)
        .cloned()
        .ok_or("Expected kind not found")
}

/// Filter the FIT data record vector by kind.
pub fn filter_fit_data_record_vec(
    fit_data_record_vec: &[FitDataRecord],
    mesg_num: fitparser::profile::MesgNum,
) -> Vec<FitDataRecord> {
    fit_data_record_vec
        .iter()
        .filter(|record| record.kind() == mesg_num)
        .cloned()
        .collect()
}

/// Find a value in a FIT data record with a custom extractor.
pub fn find_value_by_kind_in_fit_data_record<T, F>(
    fit_data_record: &FitDataRecord,
    name: &str,
    extract_value: F,
) -> Result<T, &'static str>
where
    F: Fn(&fitparser::Value) -> Result<T, &'static str>,
{
    fit_data_record
        .fields()
        .iter()
        .find(|field| field.name() == name)
        .map(|field| extract_value(field.value()))
        .transpose()
        .and_then(|opt| opt.ok_or("Failed to find value"))
}

/// Create a generator instance.
pub fn create_generator() -> Result<Generator, &'static str> {
    let manufacturer_link = LinkBuilder::new()
        .reference(GENERATOR_MANUFACTURER_ID.to_owned())
        .build()?;

    GeneratorBuilder::new()
        .name(GENERATOR_NAME.to_owned())
        .generator_type(GeneratorType::Converter)
        .link(manufacturer_link)
        .date_time(Utc::now())
        .build()
}

pub fn create_maker() -> Result<Maker, &'static str> {
    let corler_manufacturer_contact = ContactBuilder::new()
        .homepage(GENERATOR_MANUFACTURER_HOMEPAGE.to_owned())
        .build()?;

    let corler_manufacturer = ManufacturerBuilder::new()
        .id(GENERATOR_MANUFACTURER_ID.to_owned())
        .name(GENERATOR_MANUFACTURER_NAME.to_owned())
        .contact(corler_manufacturer_contact)
        .build()?;

    let garmin_manufacturer = ManufacturerBuilder::new()
        .id(DIVE_COMPUTER_MANUFACTURER_ID.to_owned())
        .name(DIVE_COMPUTER_MANUFACTURER_NAME.to_owned())
        .build()?;

    MakerBuilder::new()
        .add_manufacturer(corler_manufacturer)
        .add_manufacturer(garmin_manufacturer)
        .build()
}

/// Create a dive computer instance.
pub fn create_dive_computer(fit_data_record: &FitDataRecord) -> Result<DiveComputer, &'static str> {
    let dive_computer_serial_number =
        find_value_by_kind_in_fit_data_record(fit_data_record, SERIAL_NUMBER_TAG, |value| {
            if let fitparser::Value::UInt32z(serial_number) = value {
                Ok(serial_number.to_string())
            } else {
                Err("Invalid serial number value")
            }
        })?;

    let dive_computer_model =
        find_value_by_kind_in_fit_data_record(fit_data_record, GARMIN_PRODUCT_TAG, |value| {
            if let fitparser::Value::String(serial_number) = value {
                Ok(serial_number.to_owned())
            } else {
                Err("Invalid serial number value")
            }
        })?;

    let manufacturer_link = LinkBuilder::new()
        .reference(DIVE_COMPUTER_MANUFACTURER_ID.to_owned())
        .build()?;

    let dive_computer_id = format!("dive-computer-{}", dive_computer_serial_number);

    DiveComputerBuilder::new()
        .id(dive_computer_id)
        .model(dive_computer_model.clone())
        .name(dive_computer_model.clone())
        .link(manufacturer_link)
        .serial_number(dive_computer_serial_number)
        .build()
}

/// Create a diver instance.
pub fn create_diver(fit_data_record_file_id: &FitDataRecord) -> Result<Diver, &'static str> {
    // Assert kind of record
    if fit_data_record_file_id.kind() != fitparser::profile::MesgNum::FileId {
        return Err("Invalid record kind");
    }

    // TODO: add a way to configure it externally
    let first_name = FirstNameBuilder::new().build()?;
    // TODO: add a way to configure it externally
    let last_name = LastNameBuilder::new().build()?;

    let personal = PersonalBuilder::new()
        .first_name(first_name)
        .last_name(last_name)
        .build()?;

    let mut equipment = EquipmentBuilder::new();

    let dive_computer = create_dive_computer(fit_data_record_file_id)?;

    equipment = equipment.add_equipment(Equipment_::DiveComputer(dive_computer));

    let equipment = equipment.build()?;

    let owner = OwnerBuilder::new()
        .id(DIVER_OWNER_ID.to_owned())
        .personal(personal)
        .equipment(equipment)
        .build()?;

    DiverBuilder::new().owner(owner).build()
}

/// Create a dive site instance.
pub fn create_dive_site(fit_data_record_session: &FitDataRecord) -> Result<DiveSite, &'static str> {
    // Assert kind of record
    if fit_data_record_session.kind() != fitparser::profile::MesgNum::Session {
        return Err("Invalid record kind");
    }

    let start_position_lat = find_value_by_kind_in_fit_data_record(
        fit_data_record_session,
        START_POSITION_LAT_TAG,
        |value| {
            if let fitparser::Value::SInt32(start_position_lat) = value {
                Ok(semicircles_to_degrees(start_position_lat.to_owned(), 6))
            } else {
                Err("Invalid start position latitude value")
            }
        },
    )?;

    let start_position_long = find_value_by_kind_in_fit_data_record(
        fit_data_record_session,
        START_POSITION_LONG_TAG,
        |value| {
            if let fitparser::Value::SInt32(start_position_long) = value {
                Ok(semicircles_to_degrees(start_position_long.to_owned(), 6))
            } else {
                Err("Invalid start position longitude value")
            }
        },
    )?;

    /* let end_position_lat =
        find_value_by_kind_in_fit_data_record(fit_data_record_session, END_POSITION_LAT, |value| {
            if let fitparser::Value::SInt32(end_position_lat) = value {
                Ok(semicircles_to_degrees(end_position_lat.to_owned(), 6))
            } else {
                Err("Invalid end position latitude value")
            }
        })?;

    let end_position_long =
        find_value_by_kind_in_fit_data_record(fit_data_record_session, END_POSITION_LONG, |value| {
            if let fitparser::Value::SInt32(end_position_long) = value {
                Ok(semicircles_to_degrees(end_position_long.to_owned(), 6))
            } else {
                Err("Invalid end position longitude value")
            }
        })?; */

    let geography = GeographyBuilder::new()
        .latitude(start_position_lat)
        .longitude(start_position_long)
        .build()?;

    let location = geography
        .location
        .clone()
        .unwrap_or("Unknown location".to_owned());

    let dive_base = DiveBaseBuilder::new()
        .id(DIVE_BASE_ID.to_owned())
        .name("TBD".to_owned())
        .build()?;

    // Hash of the geography longitude and latitude
    let geography_hash = format!(
        "dive-site-{:.6}{:.6}",
        start_position_lat, start_position_long
    );

    let site = SiteBuilder::new()
        .id(geography_hash)
        .geography(geography)
        .name(location)
        .build()?;

    DiveSiteBuilder::new()
        .dive_base(dive_base)
        .site(site)
        .build()
}

/// Create gas definitions instance.
pub fn create_gas_definitions(
    fit_data_record_dive_gas: &FitDataRecord,
) -> Result<GasDefinitions, &'static str> {
    // Assert kind of record
    if fit_data_record_dive_gas.kind() != fitparser::profile::MesgNum::DiveGas {
        return Err("Invalid record kind");
    }

    let mix_oxygen_content = find_value_by_kind_in_fit_data_record(
        fit_data_record_dive_gas,
        OXYGEN_CONTENT_TAG,
        |value| {
            if let fitparser::Value::UInt8(oxygen_content) = value {
                Ok(oxygen_content.to_owned() as f64 / 100.0)
            } else {
                Err("Invalid oxygen content value")
            }
        },
    )?;

    let mix_helium_content = find_value_by_kind_in_fit_data_record(
        fit_data_record_dive_gas,
        HELIUM_CONTENT_TAG,
        |value| {
            if let fitparser::Value::UInt8(helium_content) = value {
                Ok(helium_content.to_owned() as f64 / 100.0)
            } else {
                Err("Invalid helium content value")
            }
        },
    )?;

    let mix_id = format!("mix-O2_{}-HE_{}", mix_oxygen_content, mix_helium_content);

    let mix = MixBuilder::new()
        .id(mix_id.clone())
        .name(mix_id)
        .oxygen(mix_oxygen_content)
        .helium(mix_helium_content)
        .build()?;

    GasDefinitionsBuilder::new().add_mix(mix).build()
}

/// Create a profile data instance.
pub fn create_profile_data(
    fit_data_record_file_id: &FitDataRecord,
    fit_data_record_vec: &[FitDataRecord],
    dive_summary: &FitDataRecord,
    dive_site: &DiveSite,
    gas_definitions: &GasDefinitions,
    tank_summary: &Option<FitDataRecord>,
) -> Result<ProfileData, &'static str> {
    // Assert kind of record
    if fit_data_record_file_id.kind() != fitparser::profile::MesgNum::FileId {
        return Err("Invalid record kind");
    }

    let time_created = fit_data_record_file_id
        .fields()
        .iter()
        .find(|field| field.name() == "time_created")
        .and_then(|field| match field.value() {
            fitparser::Value::Timestamp(timestamp) => Some(timestamp),
            _ => None,
        })
        .ok_or("No time_created found")?;

    // Convert local to UTC
    let time_created = DateTime::<Utc>::from_naive_utc_and_offset(time_created.naive_utc(), Utc);

    let mut records = fit_data_record_vec
        .iter()
        .filter(|record| record.kind() == fitparser::profile::MesgNum::Record);

    // Extract tank updates, it will be used in waypoint closure
    let tank_updates: Vec<(f64, DateTime<Utc>)> =
        filter_fit_data_record_vec(fit_data_record_vec, fitparser::profile::MesgNum::TankUpdate)
            .iter()
            .try_fold(Vec::new(), |mut accumulator, tank_update| {
                let tank_pressure =
                    find_value_by_kind_in_fit_data_record(tank_update, PRESSURE_TAG, |value| {
                        if let fitparser::Value::Float64(pressure) = value {
                            Ok(convert_bar_to_pascal(pressure.to_owned(), 2))
                        } else {
                            Err("Invalid pressure value")
                        }
                    })?;

                let timestamp =
                    find_value_by_kind_in_fit_data_record(tank_update, TIMESTAMP_TAG, |value| {
                        if let fitparser::Value::Timestamp(timestamp) = value {
                            Ok(DateTime::<Utc>::from_naive_utc_and_offset(
                                timestamp.naive_utc(),
                                Utc,
                            ))
                        } else {
                            Err("Invalid timestamp value")
                        }
                    })?;

                accumulator.push((tank_pressure, timestamp));
                Ok(accumulator)
            })?;

    let waypoints = records.try_fold(Vec::<Waypoint>::new(), |mut accumulator, waypoint| {
        let mut last_temperature = None;

        if let Some(waypoint) = accumulator
            .iter()
            .rev()
            .find_map(|waypoint| waypoint.temperature)
        {
            last_temperature = Some(waypoint);
        }

        let waypoint = create_waypoint(
            waypoint,
            &time_created,
            // Find tank update with the same timestamp as the waypoint
            |timestamp| {
                tank_updates
                    .iter()
                    .find_map(|(tank_pressure, tank_timestamp)| {
                        if tank_timestamp == timestamp {
                            Some(tank_pressure.to_owned())
                        } else {
                            None
                        }
                    })
            },
            last_temperature,
        )?;

        accumulator.push(waypoint);
        Ok(accumulator)
    })?;

    let samples = SamplesBuilder::new();

    let samples = waypoints
        .into_iter()
        .fold(samples, |samples, waypoint| samples.add_waypoint(waypoint));

    let samples = samples.build()?;

    let dive_number =
        find_value_by_kind_in_fit_data_record(dive_summary, DIVE_NUMBER_TAG, |value| {
            if let fitparser::Value::UInt32(dive_number) = value {
                Ok(dive_number.to_owned())
            } else {
                Err("Invalid dive number value")
            }
        })?;

    let site = dive_site.site.as_ref().unwrap();

    let dive_site_link = LinkBuilder::new()
        // TODO: unsafe unwrap, fix that
        .reference(site.id.clone())
        .build()?;

    let information_before_dive = InformationBeforeDiveBuilder::new()
        .dive_number(dive_number as u64)
        .date_time(time_created)
        .add_link(dive_site_link)
        .build()?;

    let greatest_depth =
        find_value_by_kind_in_fit_data_record(dive_summary, MAX_DEPTH_TAG, |value| {
            if let fitparser::Value::Float64(max_depth) = value {
                Ok(max_depth.to_owned())
            } else {
                Err("Invalid max depth value")
            }
        })?;

    let average_depth =
        find_value_by_kind_in_fit_data_record(dive_summary, AVG_DEPTH_TAG, |value| {
            if let fitparser::Value::Float64(max_depth) = value {
                Ok(max_depth.to_owned())
            } else {
                Err("Invalid avg depth value")
            }
        })?;

    let dive_duration =
        find_value_by_kind_in_fit_data_record(dive_summary, BOTTOM_TIME_TAG, |value| {
            if let fitparser::Value::Float64(bottom_time) = value {
                Ok(bottom_time.to_owned())
            } else {
                Err("Invalid bottom time value")
            }
        })?;

    let information_after_dive = InformationAfterDiveBuilder::new()
        .set_greatest_depth(greatest_depth)
        .set_average_depth(average_depth)
        .set_dive_duration(dive_duration)
        .build()?;

    // TODO: unsafe unwrap, fix that
    let mix_link = LinkBuilder::new()
        .reference(gas_definitions.mixes.first().unwrap().id.clone())
        .build()?;

    let mut tank_data = TankDataBuilder::new().add_link(mix_link);

    if let Some(tank_summary) = tank_summary {
        let start_pressure =
            find_value_by_kind_in_fit_data_record(tank_summary, START_PRESSURE_TAG, |value| {
                if let fitparser::Value::Float64(start_pressure) = value {
                    TankPressureBuilder::new()
                        .value(convert_bar_to_pascal(start_pressure.to_owned(), 2))
                        .build()
                } else {
                    Err("Invalid start pressure value")
                }
            })?;

        let end_pressure =
            find_value_by_kind_in_fit_data_record(tank_summary, END_PRESSURE_TAG, |value| {
                if let fitparser::Value::Float64(end_pressure) = value {
                    TankPressureBuilder::new()
                        .value(convert_bar_to_pascal(end_pressure.to_owned(), 2))
                        .build()
                } else {
                    Err("Invalid end pressure value")
                }
            })?;

        tank_data = tank_data
            .tank_pressure_begin(start_pressure)
            .tank_pressure_end(end_pressure);
    }

    let tank_data = tank_data.build()?;

    let dive = DiveBuilder::new()
        .id("d1".to_owned())
        .information_before_dive(information_before_dive)
        .add_sample(samples)
        .information_after_dive(information_after_dive)
        .tank_data(tank_data)
        .build()?;

    let repetition_group = RepetitionGroupBuilder::new()
        .id("r1".to_owned())
        .add_dive(dive)
        .build()?;

    ProfileDataBuilder::new()
        .add_repetition_group(repetition_group)
        .build()
}

/// Create a waypoint instance.
pub fn create_waypoint(
    fit_data_record_record: &FitDataRecord,
    time_created: &DateTime<Utc>,
    get_tank_pressure: impl Fn(&DateTime<Utc>) -> Option<f64>,
    // For optimization purposes
    last_temperature: Option<f64>,
) -> Result<Waypoint, &'static str> {
    // Assert kind of record
    if fit_data_record_record.kind() != fitparser::profile::MesgNum::Record {
        return Err("Invalid record kind");
    }

    let timestamp =
        find_value_by_kind_in_fit_data_record(fit_data_record_record, TIMESTAMP_TAG, |value| {
            if let fitparser::Value::Timestamp(timestamp) = value {
                Ok(DateTime::<Utc>::from_naive_utc_and_offset(
                    timestamp.naive_utc(),
                    Utc,
                ))
            } else {
                Err("Invalid timestamp value")
            }
        })?;

    // Get tank pressure, if available at the same timestamp, with the help of the closure
    let tank_pressure = get_tank_pressure(&timestamp)
        .map(|pressure| {
            // Create tank pressure instance
            TankPressureBuilder::new().value(pressure).build()
        })
        .transpose()?;

    // Get difference in seconds between the current and time created
    let dive_time = timestamp.signed_duration_since(time_created).num_seconds() as u64;

    let depth =
        find_value_by_kind_in_fit_data_record(fit_data_record_record, DEPTH_TAG, |value| {
            if let fitparser::Value::Float64(depth) = value {
                Ok(depth.to_owned())
            } else {
                Err("Invalid depth value")
            }
        })?;

    let temperature =
        find_value_by_kind_in_fit_data_record(fit_data_record_record, TEMPERATURE_TAG, |value| {
            if let fitparser::Value::SInt8(temperature) = value {
                Ok(convert_celsius_to_kelvin(*temperature as f64))
            } else {
                // TODO: useless error message, to improve
                Err("Invalid temperature value")
            }
        })
        .ok();

    let mut waypoint = WaypointBuilder::new()
        .dive_time(dive_time)
        .depth(depth.to_owned());

    // Optimization: only add optional fields if they are different from the previous waypoint
    if let Some(temperature) = temperature {
        let mut should_add_temperature = true;

        if let Some(last_temperature) = last_temperature
            && last_temperature == temperature
        {
            // Skip adding temperature
            should_add_temperature = false;
        }

        if should_add_temperature {
            waypoint = waypoint.temperature(temperature.to_owned());
        }
    }

    if tank_pressure.is_some() {
        waypoint = waypoint.tank_pressure(tank_pressure.unwrap());
    }

    waypoint.build()
}
