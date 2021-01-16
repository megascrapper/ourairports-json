use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};

/// Contains a record of a single airport.
#[derive(Deserialize, Serialize)]
pub struct Airport {
    /// Internal OurAirports integer identifier for the airport.
    /// This will stay persistent, even if the airport code changes.
    id: String,
    /// The text identifier used in the OurAirports URL.
    /// This will be the ICAO code if available. Otherwise, it will be a local airport code (if no conflict), or if nothing else is available, an internally-generated code starting with the ISO2 country code, followed by a dash and a four-digit number.
    ident: String,
    /// The type of the airport.
    /// Allowed values are "closed_airport", "heliport", "large_airport", "medium_airport", "seaplane_base", and "small_airport".
    #[serde(rename = "type")]
    airport_type: String,
    /// The official airport name, including "Airport", "Airstrip", etc.
    name: String,
    /// The airport latitude in decimal degrees (positive for north).
    latitude_deg: f64,
    /// The airport longitude in decimal degrees (positive for east).
    longitude_deg: f64,
    /// The airport elevation MSL in feet (*not* metres). None if unavailable.
    elevation_ft: Option<i32>,
    /// The code for the continent where the airport is (primarily) located.
    /// Allowed values are "AF" (Africa), "AN" (Antarctica), "AS" (Asia), "EU" (Europe), "NA" (North America), "OC" (Oceania), or "SA" (South America).
    continent: String,
    /// The two-character ISO 3166:1-alpha2 code for the country where the airport is (primarily) located.
    /// A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo.
    iso_country: String,
    /// An alphanumeric code for the high-level administrative subdivision of a country where the airport is primarily located (e.g. province, governorate), prefixed by the ISO2 country code and a hyphen.
    /// OurAirports uses ISO 3166:2 codes whenever possible, preferring higher administrative levels, but also includes some custom codes.
    iso_region: String,
    /// The primary municipality that the airport serves (when available).
    /// Note that this is *not* necessarily the municipality where the airport is physically located.
    municipality: String,
    /// true if the airport currently has scheduled airline service; false otherwise.
    #[serde(deserialize_with = "bool_from_str")]
    scheduled_service: bool,
    /// The code that an aviation GPS database (such as Jeppesen's or Garmin's) would normally use for the airport. This will always be the ICAO code if one exists.
    /// Note that, unlike the `ident` column, this is *not* guaranteed to be globally unique.
    gps_code: String,
    /// The three-letter IATA code for the airport (if it has one).
    iata_code: String,
    /// The local country code for the airport, if different from the `gps_code` and `iata_code` fields (used mainly for US airports).
    local_code: String,
    /// URL of the airport's official home page on the web, if one exists.
    home_link: String,
    /// URL of the airport's page on Wikipedia, if one exists.
    wikipedia_link: String,
    /// Extra keywords/phrases to assist with search, as a Vec.
    /// May include former names for the airport, alternate codes, names in other languages, nearby tourist destinations, etc.
    #[serde(deserialize_with = "vec_string_from_string")]
    keywords: Vec<String>,
}

/// Contains information about a single airport radio frequency
/// for voice communication (radio navigation aids appear in struct Navaids)
#[derive(Deserialize, Serialize)]
pub struct AirportFrequency {
    /// Internal OurAirports integer identifier for the frequency.
    /// This will stay persistent, even if the radio frequency or description changes.
    id: String,
    /// Internal integer foreign key matching the `id` column for the associated airport in Airports struct.
    /// (`airport_ident` is a better alternative.)
    airport_ref: String,
    /// Externally-visible string foreign key matching the `ident` column for the associated airport in Airports.
    airport_ident: String,
    /// A code for the frequency type.
    /// This isn't (currently) a controlled vocabulary, but probably will be soon.
    /// Some common values are "TWR" (tower), "ATF" or "CTAF" (common traffic frequency), "GND" (ground control), "RMP" (ramp control), "ATIS" (automated weather), "RCO" (remote radio outlet), "ARR" (arrivals), "DEP" (departures), "UNICOM" (monitored ground station), and "RDO" (a flight-service station).
    #[serde(rename = "type")]
    frequency_type: String,
    /// A description of the frequency, typically the way a pilot would open a call on it.
    description: String,
    /// Radio voice frequency in megahertz.
    /// Note that the same frequency may appear multiple times for an airport, serving different functions.
    frequency_mhz: String,
}

/// Contains information about a single landing surface
#[derive(Deserialize, Serialize)]
pub struct Runway {
    /// Internal OurAirports integer identifier for the runway.
    /// This will stay persistent, even if the runway numbering changes.
    id: String,
    /// Internal integer foreign key matching the id column for the associated airport in airports.csv. (`airport_ident` is a better alternative.)
    airport_ref: String,
    /// Externally-visible string foreign key matching the ident column for the associated airport in airports.csv.
    airport_ident: String,
    /// Length of the full runway surface (including displaced thresholds, overrun areas, etc) in feet.
    length_ft: Option<u32>,
    /// Width of the runway surface in feet.
    width_ft: Option<u32>,
    /// Code for the runway surface type.
    /// This is not yet a controlled vocabulary, but probably will be soon.
    /// Some common values include "ASP" (asphalt), "TURF" (turf), "CON" (concrete), "GRS" (grass), "GRE" (gravel), "WATER" (water), and "UNK" (unknown).
    surface: String,
    /// `true` if the surface is lighted at night. `false` otherwise.
    #[serde(deserialize_with = "bool_from_str")]
    lighted: bool,
    /// `true` if the runway surface is currently closed, `false` otherwise.
    #[serde(deserialize_with = "bool_from_str")]
    closed: bool,
    /// Identifier for the low-numbered end of the runway.
    le_ident: String,
    /// Latitude of the centre of the low-numbered end of the runway, in decimal degrees (positive is north), if available.
    le_latitude_deg: Option<f64>,
    /// Longitude of the centre of the low-numbered end of the runway, in decimal degrees (positive is east), if available.
    le_longitude_deg: Option<f64>,
    /// Elevation above MSL of the low-numbered end of the runway in feet.
    le_elevation_ft: Option<i32>,
    /// Heading of the low-numbered end of the runway in degrees true (*not* magnetic).
    #[serde(rename = "le_heading_degT")]
    le_heading_deg_true: Option<f64>,
    /// Length of the displaced threshold (if any) for the low-numbered end of the runway, in feet.
    le_displaced_threshold_ft: Option<i32>,
    /// Identifier for the high-numbered end of the runway.
    he_ident: String,
    /// Latitude of the centre of the high-numbered end of the runway, in decimal degrees (positive is north), if available.
    he_latitude_deg: Option<f64>,
    /// Longitude of the centre of the high-numbered end of the runway, in decimal degrees (positive is east), if available.
    he_longitude_deg: Option<f64>,
    /// Elevation above MSL of the high-numbered end of the runway in feet.
    he_elevation_ft: Option<i32>,
    #[serde(rename = "he_heading_degT")]
    /// Heading of the high-numbered end of the runway in degrees true (*not* magnetic).
    he_heading_deg_true: Option<f64>,
    /// Length of the displaced threshold (if any) for the high-numbered end of the runway, in feet.
    he_displaced_threshold_ft: Option<i32>,
}

/// Converts a string to a boolean based on "yes" and "no"
fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.to_lowercase().as_str() {
        "yes" | "1"=> Ok(true),
        "no" | "0" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"Value must be yes or no",
        )),
    }
}

/// Transforms a comma-separated string to a vector.
fn vec_string_from_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let keywords = String::deserialize(deserializer)?;
    match keywords.len() {
        0 => Ok(vec![]),
        _ => Ok(keywords.split(',').map(|s| s.to_string()).collect()),
    }
}
