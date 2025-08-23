use ip2location::{DB, Record, error};

const IPV6BIN: &str = "ip_db.BIN";

pub struct CountryDetails {
    pub name: String,
    pub code: String,
}

pub fn get_country_details() -> Result<CountryDetails, error::Error> {
    let db = DB::from_file(IPV6BIN)?;
    let record = db.ip_lookup("43.224.159.155".parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    let record = record.unwrap();
    let country = record.country.unwrap();

    Ok(CountryDetails {
        name: country.long_name.to_string(),
        code: country.short_name.to_string(),
    })
}
