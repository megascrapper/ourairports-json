use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Airport {
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
    scheduled_service: bool,
    gps: String,
    iata_code: String,
    local_code: String,
    home_link: String,
    wikiepdia_link: String,
    keywords: Vec<String>,
}

impl Airport {
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
