use std::{fs::File, io::Write, path::Path};

use uddf_sdk::{
    converter::{constants::UDDF_XMLNS_MODEL, garmin::import::from_garmin_fit},
    utils::xml::ToXml,
};

const FILE_NAME: &str = "16961399118_ACTIVITY";

fn main() -> Result<(), &'static str> {
    let current_dir = Path::new(file!())
        .parent()
        .ok_or("Error getting current directory")?;

    let fit_file_path = current_dir.join(format!("../assets/garmin/{}.fit", FILE_NAME));
    println!("Reading FIT file: {:?}", fit_file_path);

    let mut fit_file = File::open(fit_file_path).expect("File not found");

    println!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );

    let fit_content = fitparser::from_reader(&mut fit_file).expect("Error parsing FIT file");

    let uddf = from_garmin_fit(fit_content)?;

    let xml_content = uddf.to_xml();

    let xml_declaration = format!(
        "{}\n{}",
        r#"<?xml version="1.0" encoding="utf-8"?>"#,
        format_args!(r#"<?xml-model href="{}"?>"#, UDDF_XMLNS_MODEL)
    );

    let xml = format!("{}\n{}", xml_declaration, xml_content);

    // Save the XML to a file
    let xml_file_path = current_dir.join(format!("../assets/garmin/{}.xml", FILE_NAME));
    let mut file = File::create(xml_file_path).expect("Error creating XML file");
    file.write_all(xml.as_bytes())
        .expect("Error writing to file");

    Ok(())
}
