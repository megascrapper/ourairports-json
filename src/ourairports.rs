use serde::{Serialize, Deserialize};

/// Contains a record of a single airport.
#[derive(Debug, Serialize, Deserialize)]
pub struct Airport {
    /// Internal OurAirports integer identifier for the airport.
    /// This will stay persistent, even if the airport code changes.
    id: String,
    /// The text identifier used in the OurAirports URL.
    /// This will be the ICAO code if available. Otherwise, it will be a local airport code (if no conflict), or if nothing else is available, an internally-generated code starting with the ISO2 country code, followed by a dash and a four-digit number.
    ident: String,
    /// The type of the airport.
    /// Allowed values are "closed_airport", "heliport", "large_airport", "medium_airport", "seaplane_base", and "small_airport".
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
    scheduled_service: bool,
    /// The code that an aviation GPS database (such as Jeppesen's or Garmin's) would normally use for the airport. This will always be the ICAO code if one exists.
    /// Note that, unlike the `ident` column, this is *not* guaranteed to be globally unique.
    gps: String,
    /// The three-letter IATA code for the airport (if it has one).
    iata_code: String,
    /// The local country code for the airport, if different from the `gps_code` and `iata_code` fields (used mainly for US airports).
    local_code: String,
    /// 	URL of the airport's official home page on the web, if one exists.
    home_link: String,
    /// URL of the airport's page on Wikipedia, if one exists.
    wikiepdia_link: String,
    /// Extra keywords/phrases to assist with search, as a Vec.
    /// May include former names for the airport, alternate codes, names in other languages, nearby tourist destinations, etc.
    keywords: Vec<String>,
}

impl Airport {
    /// Creates a new instance of Airport
    ///
    /// The latitude and longitude value must be parsed before ping passed, and the altitude value bust be parsed and encapsulated in an `Option`.
    pub fn new(
        id: String,
        ident: String,
        airport_type: String,
        name: String,
        latitude_deg: f64,
        longitude_deg: f64,
        elevation_ft: Option<i32>,
        continent: String,
        iso_country: String,
        iso_region: String,
        municipality: String,
        scheduled_service: String,
        gps: String,
        iata_code: String,
        local_code: String,
        home_link: String,
        wikiepdia_link: String,
        keywords: String,
    ) -> Airport {
        Airport {
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
            scheduled_service: scheduled_service == "yes",
            gps,
            iata_code,
            local_code,
            home_link,
            wikiepdia_link,
            keywords: to_vec_string(keywords),
        }
    }
}

fn to_vec_string(value: String) -> Vec<String> {
    match value.len() {
        0 => vec![],
        _ => value.split(", ").map(|s| s.to_string()).collect(),
    }
}

pub fn parse_option_i32(value: String) -> Option<i32> {
    let parsed = value.parse();
    match parsed {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
