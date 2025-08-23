use ip2location::{DB, Record, error};
use std::net::IpAddr;

const IPV6BIN: &str = "ip_db.BIN";

pub struct CountryDetails {
    pub name: String,
    pub code: String,
}

impl Default for CountryDetails {
    fn default() -> Self {
        CountryDetails {
            name: "Unknown".to_string(),
            code: "Unknown".to_string(),
        }
    }
}

pub fn get_country_details(ip_address: IpAddr) -> Result<Option<CountryDetails>, error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup(ip_address)?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    let record = record.unwrap();
    let country = record.country.unwrap();

    // This means that we didn't find a country.
    if country.long_name.to_string() == "-".to_string() {
        return Ok(None);
    }

    Ok(Some(CountryDetails {
        name: country.long_name.to_string(),
        code: country.short_name.to_string(),
    }))
}
