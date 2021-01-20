//! ourairports
//! Converts data from OurAirports to JSON format.

/**
 * TODO
 * tests
 * example code
 * turn this to a library
 */
extern crate anyhow;
extern crate clap;
extern crate csv;
extern crate human_panic;
extern crate serde;
extern crate serde_json;
use anyhow::{Context, Result};
use clap::Clap;
use human_panic::setup_panic;
use std::fs;

/// Airport data URL
const AIRPORT_URL: &str = "https://ourairports.com/data/airports.csv";

/// Airport frequency data URL
const AIRPORT_FREQUENCY_URL: &str = "https://ourairports.com/data/airport-frequencies.csv";

/// Runway data URL
const RUNWAY_URL: &str = "https://ourairports.com/data/runways.csv";

/// navaid data URL
const NAVAID_URL: &str = "https://ourairports.com/data/navaids.csv";

/// country data URL
const COUNTRY_URL: &str = "https://ourairports.com/data/countries.csv";

/// region data URL
const REGION_URL: &str = "https://ourairports.com/data/regions.csv";

// import ourairports module and all structs
/// Contains all of the structs of data types available from OurAirports
/// as well as the methods used to instantiate one.
///
/// Dataset format information is from https://ourairports.com/help/data-dictionary.html
/// with some modifications.
mod ourairports;
use ourairports::*;

/// Converts data from OurAirports to JSON format.
/// You need to download the data on your own from https://ourairports.com/data/
#[derive(Clap)]
enum Cli {
    /// Convert airport data
    Airport {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
    /// Convert airport frequency data
    AirportFrequency {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
    /// Convert runway data
    Runway {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
    /// Convert navaid data
    Navaid {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
    /// Convert country data
    Country {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
    /// Convert region data
    Region {
        #[clap(parse(from_os_str))]
        /// Airport data file from openflights
        input_file: Option<std::path::PathBuf>,
        #[clap(short = 'o', long = "output")]
        /// Output file
        output_file: Option<std::path::PathBuf>,
        /// Pretty print output
        #[clap(short = 'p', long = "pretty-print")]
        pretty_print: bool,
    },
}

/// Request data type
enum RequestType {
    Airport,
    AirportFrequency,
    Runway,
    Navaid,
    Country,
    Region,
}

/// Reads the csv data from a local file or the internet
#[tokio::main]
async fn read_text(
    file_path: &Option<std::path::PathBuf>,
    request_type: RequestType,
) -> Result<String> {
    if let Some(path) = file_path {
        println!("Reading file {}", path.to_string_lossy());
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Could not open file: {}", path.to_string_lossy()))?;
        Ok(content)
    } else {
        let url = match request_type {
            RequestType::Airport => AIRPORT_URL,
            RequestType::AirportFrequency => AIRPORT_FREQUENCY_URL,
            RequestType::Runway => RUNWAY_URL,
            RequestType::Navaid => NAVAID_URL,
            RequestType::Country => COUNTRY_URL,
            RequestType::Region => REGION_URL,
        };
        println!("Downloading from {}", url);
        let resp = reqwest::get(url)
            .await
            .with_context(|| format!("Could not open page: {}", url))?
            .text()
            .await?;
        Ok(resp)
    }
}

/// Converts airport data to JSON
fn convert_airport_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    // read original file as csv
    let data = read_text(&file_path, RequestType::Airport)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    // plane list
    let mut airport_list: Vec<Airport> = Vec::new();

    // deserialize each record to a struct and add to list
    for line in rdr.deserialize() {
        let record: Airport = line?;
        airport_list.push(record);
    }

    // convert to json
    if !pretty_print {
        let json_out = serde_json::to_string(&airport_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&airport_list)?;
        Ok(json_out)
    }
}

/// Converts airport frequency data to JSON
fn convert_airport_frequency_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    let data = read_text(&file_path, RequestType::AirportFrequency)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut airport_frequency_list: Vec<AirportFrequency> = Vec::new();
    for line in rdr.deserialize() {
        let record: AirportFrequency = line?;
        airport_frequency_list.push(record);
    }

    if !pretty_print {
        let json_out = serde_json::to_string(&airport_frequency_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&airport_frequency_list)?;
        Ok(json_out)
    }
}

/// Converts runway data to JSON
fn convert_runway_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    let data = read_text(&file_path, RequestType::Runway)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut runway_list: Vec<Runway> = Vec::new();
    for line in rdr.deserialize() {
        let record: Runway = line?;
        runway_list.push(record);
    }

    if !pretty_print {
        let json_out = serde_json::to_string(&runway_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&runway_list)?;
        Ok(json_out)
    }
}

/// Converts navaid data to JSON
fn convert_navaid_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    let data = read_text(&file_path, RequestType::Navaid)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut navaid_list: Vec<Navaid> = Vec::new();
    for line in rdr.deserialize() {
        let record: Navaid = line?;
        navaid_list.push(record);
    }

    if !pretty_print {
        let json_out = serde_json::to_string(&navaid_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&navaid_list)?;
        Ok(json_out)
    }
}

/// Converts country data to JSON
fn convert_country_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    let data = read_text(&file_path, RequestType::Country)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut country_list: Vec<Country> = Vec::new();
    for line in rdr.deserialize() {
        let record: Country = line?;
        country_list.push(record);
    }

    if !pretty_print {
        let json_out = serde_json::to_string(&country_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&country_list)?;
        Ok(json_out)
    }
}

/// Converts region data to JSON
fn convert_region_data(
    file_path: &Option<std::path::PathBuf>,
    pretty_print: bool,
) -> Result<String> {
    let data = read_text(&file_path, RequestType::Region)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut region_list: Vec<Region> = Vec::new();
    for line in rdr.deserialize() {
        let record: Region = line?;
        region_list.push(record);
    }

    if !pretty_print {
        let json_out = serde_json::to_string(&region_list)?;
        Ok(json_out)
    } else {
        let json_out = serde_json::to_string_pretty(&region_list)?;
        Ok(json_out)
    }
}

fn main() -> Result<()> {
    // setup panic handler
    setup_panic!();

    // match command args
    match Cli::parse() {
        // airports
        Cli::Airport {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(
                    output_path,
                    convert_airport_data(&input_file, pretty_print)?,
                )?;
            } else {
                println!("{}", convert_airport_data(&input_file, pretty_print)?);
            }
        }
        Cli::AirportFrequency {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(
                    output_path,
                    convert_airport_frequency_data(&input_file, pretty_print)?,
                )?;
            } else {
                println!(
                    "{}",
                    convert_airport_frequency_data(&input_file, pretty_print)?
                );
            }
        }
        Cli::Runway {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(output_path, convert_runway_data(&input_file, pretty_print)?)?;
            } else {
                println!("{}", convert_runway_data(&input_file, pretty_print)?);
            }
        },
        Cli::Navaid {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(output_path, convert_navaid_data(&input_file, pretty_print)?)?;
            } else {
                println!("{}", convert_navaid_data(&input_file, pretty_print)?);
            }
        },
        Cli::Country {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(output_path, convert_country_data(&input_file, pretty_print)?)?;
            } else {
                println!("{}", convert_country_data(&input_file, pretty_print)?);
            }
        },
        Cli::Region {
            input_file,
            output_file,
            pretty_print,
        } => {
            if let Some(output_path) = output_file {
                fs::write(output_path, convert_region_data(&input_file, pretty_print)?)?;
            } else {
                println!("{}", convert_region_data(&input_file, pretty_print)?);
            }
        },
    }

    Ok(())
}
