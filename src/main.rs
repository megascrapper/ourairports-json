//! ourairports
//! Converts data from OurAirports to JSON format.

/**
 * TODO
 * use serde deserialize (see sandbox copy for implementation)
 * other data types
 * tests
 */
extern crate anyhow;
extern crate clap;
extern crate csv;
extern crate human_panic;
extern crate serde;
extern crate serde_json;
use anyhow::{anyhow, Context, Result};
use clap::Clap;
use human_panic::setup_panic;
use std::fs;

/// Number of fields in airport data
const AIRPORT_FIELDS: usize = 18;

/// Airport data URL
const AIRPORT_URL: &str = "https://ourairports.com/data/airports.csv";

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
}

/// Request data type
enum RequestType {
    Airport,
}

/// Reads the csv data from a local file or the internet
#[tokio::main]
async fn read_text(file_path: &Option<std::path::PathBuf>, request_type: RequestType) -> Result<String> {
    if let Some(path) = file_path {
        println!("Reading file {}", path.to_string_lossy());
        let content = fs::read_to_string(&path).with_context(|| format!("Could not open file: {}", path.to_string_lossy()))?;
        Ok(content)
    } else {
        match request_type {
            RequestType::Airport => {
                println!("Downloading from {}", AIRPORT_URL);
                let resp = reqwest::get(AIRPORT_URL)
                .await
                .with_context(|| format!("Could not open page: {}",AIRPORT_URL))?
                .text()
                .await?;
                Ok(resp)
            },
        }
    }
}

/// Converts airport data to JSON
fn convert_airport_data(file_path: &Option<std::path::PathBuf>, pretty_print: bool) -> Result<String> {
    // read original file as csv
    let data = read_text(&file_path, RequestType::Airport)?;
    println!("Converting data");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    // plane list
    let mut airport_list: Vec<Airport> = Vec::new();

    // deserialize each record to a struct and add to list
    for line in rdr.records() {
        let record = line?;
        // Return error if the number of fields do not match
        if record.len() != AIRPORT_FIELDS {
            return Err(anyhow!(
                "Invalid number of columns: expected {}, found {}",
                AIRPORT_FIELDS,
                record.len()
            ));
        }

        let id = record[0].to_string();
        let ident = record[1].to_string();
        let airport_type = record[2].to_string();
        let name = record[3].to_string();
        let latitude_deg = record[4].parse()?;
        let longitude_deg = record[5].parse()?;
        let elevation_ft = parse_option_i32(record[6].to_string());
        let continent = record[7].to_string();
        let iso_country = record[8].to_string();
        let iso_region = record[9].to_string();
        let municipality = record[10].to_string();
        let scheduled_service = record[11].to_string();
        let gps = record[12].to_string();
        let iata_code = record[13].to_string();
        let local_code = record[14].to_string();
        let home_link = record[15].to_string();
        let wikiepdia_link = record[16].to_string();
        let keywords = record[17].to_string();

        airport_list.push(Airport::new(
            id,
            ident,
            airport_type,
            name,
            latitude_deg,
            longitude_deg,
            elevation_ft,
            continent,
            iso_country,
            iso_region,
            municipality,
            scheduled_service,
            gps,
            iata_code,
            local_code,
            home_link,
            wikiepdia_link,
            keywords,
        ));
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
    }

    Ok(())
}
