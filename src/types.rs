use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub iso3: String,
    pub numeric: Option<String>,
    pub name: String,
    pub official_name: String,
    pub name_tr: Option<String>,
    pub capital: Option<String>,
    pub region: Option<String>,
    pub subregion: Option<String>,
    pub tld: Option<String>,
    pub flag: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub area_km2: Option<f64>,
    pub independent: bool,
    #[serde(default)]
    pub currencies: Vec<Currency>,
    #[serde(default)]
    pub languages: Vec<Language>,
    pub calling_code: Option<String>,
    pub in_eu: bool,
}

#[derive(Debug, Deserialize)]
pub struct Division {
    pub id: String,
    pub country: String,
    pub code: String,
    pub name: String,
    pub level: i32,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub children: Vec<Division>,
}

#[derive(Debug, Deserialize)]
pub struct IpLocation {
    pub ip: Option<String>,
    pub version: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub country_code: Option<String>,
    pub country_code_iso3: Option<String>,
    pub country_name: Option<String>,
    pub country_name_tr: Option<String>,
    pub country_capital: Option<String>,
    pub country_tld: Option<String>,
    pub country_flag: Option<String>,
    pub continent_code: Option<String>,
    pub in_eu: Option<bool>,
    pub postal: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub utc_offset: Option<String>,
    pub country_calling_code: Option<String>,
    pub currency: Option<String>,
    pub currency_name: Option<String>,
    pub languages: Option<String>,
    #[serde(default)]
    pub language_codes: Vec<String>,
    pub country_area: Option<f64>,
    pub asn: Option<String>,
    pub org: Option<String>,
    pub colo: Option<String>,
    pub metro_code: Option<String>,
    pub browser_language: Option<String>,
    pub source: Option<String>,
    pub attribution: Option<String>,
    pub attribution_url: Option<String>,
    /// Listed in the Tor exit list. `None` when the list is not loaded.
    #[serde(default)]
    pub is_tor: Option<bool>,
    /// Inside a published cloud or hosting range. `None` when the data is not loaded.
    #[serde(default)]
    pub is_hosting: Option<bool>,
    #[serde(default)]
    pub hosting_provider: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallingCode {
    pub country: String,
    pub name: String,
    pub calling_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PhoneValidation {
    pub valid: bool,
    pub possible: bool,
    pub reason: Option<String>,
    pub e164: Option<String>,
    pub national: Option<String>,
    pub international: Option<String>,
    pub country: Option<String>,
    pub calling_code: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlateFormat {
    pub country: String,
    pub description: String,
    pub example: String,
}

#[derive(Debug, Deserialize)]
pub struct PlateValidation {
    pub valid: bool,
    pub reason: Option<String>,
    pub normalized: Option<String>,
    pub country: String,
    pub province_code: Option<String>,
    pub province_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReferenceRate {
    pub code: String,
    pub rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct TcmbRate {
    pub code: String,
    pub name: String,
    pub unit: i32,
    pub source_unit: f64,
    pub forex_buying: Option<f64>,
    pub forex_selling: Option<f64>,
    pub banknote_buying: Option<f64>,
    pub banknote_selling: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "source")]
pub enum Rates {
    #[serde(rename = "ecb")]
    Ecb {
        base: String,
        date: Option<String>,
        fetched_at: String,
        rates: Vec<ReferenceRate>,
    },
    #[serde(rename = "fed")]
    Fed {
        base: String,
        date: Option<String>,
        fetched_at: String,
        rates: Vec<ReferenceRate>,
    },
    #[serde(rename = "tcmb")]
    Tcmb {
        base: String,
        date: Option<String>,
        fetched_at: String,
        rates: Vec<TcmbRate>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Conversion {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub rate: f64,
    pub result: f64,
    pub source: String,
    pub base: String,
    pub date: Option<String>,
    pub tcmb_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Distance {
    pub kilometers: f64,
    pub from: Place,
    pub to: Place,
}

#[derive(Debug, Deserialize)]
pub struct IbanValidation {
    pub valid: bool,
    pub reason: Option<String>,
    pub iban: Option<String>,
    pub formatted: Option<String>,
    pub country: Option<String>,
    pub check_digits: Option<String>,
    pub bban: Option<String>,
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrNumberValidation {
    pub valid: bool,
    pub reason: Option<String>,
    pub r#type: String,
    pub registry_checked: bool,
}

#[derive(Debug, Deserialize)]
pub struct Holiday {
    pub date: String,
    pub name: String,
    pub local_name: String,
    pub half_day: bool,
    pub nationwide: bool,
}

#[derive(Debug, Deserialize)]
pub struct HolidayList {
    pub country: String,
    pub year: i32,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub holidays: Vec<Holiday>,
}

#[derive(Debug, Deserialize)]
pub struct BusinessDays {
    pub country: String,
    pub start: String,
    pub days: Option<i32>,
    pub end: String,
    pub business_days: i32,
    #[serde(default)]
    pub weekend: Vec<i32>,
    #[serde(default)]
    pub holidays_skipped: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Timezone {
    pub timezone: String,
    pub utc_offset: Option<String>,
    pub abbreviation: Option<String>,
    pub is_dst: bool,
    pub local_time: String,
    pub country_code: Option<String>,
    #[serde(default)]
    pub timezones: Vec<String>,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct MxRecord {
    pub priority: i32,
    pub exchange: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailCheck {
    pub email: String,
    pub valid: bool,
    pub reason: Option<String>,
    pub normalized: Option<String>,
    pub local_part: Option<String>,
    pub domain: Option<String>,
    pub syntax_valid: bool,
    pub dns_checked: bool,
    pub domain_exists: Option<bool>,
    pub accepts_mail: Option<bool>,
    #[serde(default)]
    pub mx_records: Vec<MxRecord>,
    pub disposable: bool,
    pub role: bool,
}

#[derive(Debug, Deserialize)]
pub struct VatCheck {
    pub country_code: String,
    pub vat_number: String,
    pub valid: bool,
    pub name: Option<String>,
    pub address: Option<String>,
    pub request_date: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country: Option<String>,
}

impl Point {
    pub fn country(code: impl Into<String>) -> Self {
        Self { latitude: None, longitude: None, country: Some(code.into()) }
    }

    pub fn coordinates(latitude: f64, longitude: f64) -> Self {
        Self { latitude: Some(latitude), longitude: Some(longitude), country: None }
    }
}

#[derive(Debug, Deserialize)]
pub struct IdValidator {
    pub country: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub holder: String,
    pub description: String,
    pub example: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct IdValidation {
    pub valid: bool,
    pub normalized: Option<String>,
    pub country: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub reason: Option<String>,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
    pub registry_checked: bool,
}

#[derive(Debug, Deserialize)]
pub struct GeoNamesAttribution {
    pub source: String,
    pub url: String,
    pub license: String,
}

#[derive(Debug, Deserialize)]
pub struct PostalCodeCountry {
    pub country: String,
    pub count: u64,
    pub partial: bool,
    pub updated: String,
}

#[derive(Debug, Deserialize)]
pub struct PostalCodeCountries {
    pub data: Vec<PostalCodeCountry>,
    pub attribution: GeoNamesAttribution,
}

#[derive(Debug, Deserialize)]
pub struct PostalPlace {
    pub postal_code: String,
    pub place: String,
    pub admin1: Option<String>,
    pub admin1_code: Option<String>,
    pub admin2: Option<String>,
    pub admin2_code: Option<String>,
    pub admin3: Option<String>,
    pub admin3_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PostalCodeLookup {
    pub country: String,
    pub query: String,
    pub postal_code: String,
    #[serde(rename = "match")]
    pub match_kind: String,
    pub partial_data: bool,
    pub coverage_note: Option<String>,
    pub places: Vec<PostalPlace>,
    pub updated: String,
    pub attribution: GeoNamesAttribution,
}

#[derive(Debug, Deserialize)]
pub struct ReverseArea {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReverseGeocode {
    pub latitude: f64,
    pub longitude: f64,
    pub country_code: String,
    pub country_name: Option<String>,
    pub admin1: Option<ReverseArea>,
    pub admin2: Option<ReverseArea>,
    pub precision: String,
    pub attribution: String,
}

#[derive(Debug, Deserialize)]
pub struct NameVersion {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub major: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UaDevice {
    #[serde(rename = "type")]
    pub kind: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UaCpu {
    pub architecture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserAgent {
    pub ua: String,
    pub browser: NameVersion,
    pub engine: NameVersion,
    pub os: NameVersion,
    pub device: UaDevice,
    pub cpu: UaCpu,
    pub is_bot: bool,
    pub bot_match: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BarcodeValidation {
    pub input: String,
    pub normalized: String,
    pub valid: bool,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub reason: Option<String>,
    pub check_digit: Option<String>,
    pub expected_check_digit: Option<String>,
    pub gtin14: Option<String>,
    pub isbn10: Option<String>,
    pub isbn13: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Rates;

    #[test]
    fn decodes_tcmb_rates() {
        let raw = r#"{"source":"tcmb","base":"TRY","date":"2025-12-31","fetched_at":"2025-12-31T15:00:00Z","rates":[{"code":"USD","name":"US Dollar","unit":1,"source_unit":1,"forex_buying":1.1,"forex_selling":null,"banknote_buying":null,"banknote_selling":null}]}"#;
        let rates: Rates = serde_json::from_str(raw).unwrap();
        match rates {
            Rates::Tcmb { rates, .. } => assert_eq!(rates[0].code, "USD"),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn decodes_new_endpoint_bodies() {
        let id: super::IdValidation = serde_json::from_str(r#"{"valid":true,"normalized":"12345678Z","country":"ES","type":"nif","reason":null,"registry_checked":false}"#).unwrap();
        assert_eq!(id.kind, "nif");
        let bc: super::BarcodeValidation = serde_json::from_str(r#"{"input":"4006381333931","normalized":"4006381333931","valid":true,"type":"ean13","reason":null,"check_digit":"1","expected_check_digit":"1","gtin14":"04006381333931","isbn10":null,"isbn13":null}"#).unwrap();
        assert_eq!(bc.kind.as_deref(), Some("ean13"));
        let ua: super::UserAgent = serde_json::from_str(r#"{"ua":"x","browser":{"name":"Chrome","version":"1","major":"1"},"engine":{"name":null,"version":null},"os":{"name":null,"version":null},"device":{"type":"desktop","vendor":null,"model":null},"cpu":{"architecture":null},"is_bot":false,"bot_match":null}"#).unwrap();
        assert_eq!(ua.device.kind, "desktop");
        let rev: super::ReverseGeocode = serde_json::from_str(r#"{"latitude":41.01,"longitude":28.97,"country_code":"TR","country_name":"Turkey","admin1":{"code":"34","name":"Istanbul","type":"province"},"admin2":null,"precision":"admin1","attribution":"geoBoundaries"}"#).unwrap();
        assert_eq!(rev.admin1.unwrap().name, "Istanbul");
        let pc: super::PostalCodeLookup = serde_json::from_str(r#"{"country":"DE","query":"10115","postal_code":"10115","match":"exact","partial_data":false,"coverage_note":null,"places":[{"postal_code":"10115","place":"Berlin","admin1":null,"admin1_code":null,"admin2":null,"admin2_code":null,"admin3":null,"admin3_code":null,"latitude":52.5,"longitude":13.4,"accuracy":4}],"updated":"2026-10-01","attribution":{"source":"GeoNames","url":"https://www.geonames.org","license":"CC BY 4.0"}}"#).unwrap();
        assert_eq!(pc.places[0].place, "Berlin");
    }
}
