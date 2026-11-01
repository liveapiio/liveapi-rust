use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use crate::types::{
    BusinessDays, CallingCode, Conversion, Country, Distance, Division, EmailCheck, HolidayList, IbanValidation, IpLocation,
    PhoneValidation, PlateFormat, PlateValidation, Point, Rates, Timezone, TrNumberValidation, VatCheck, IdValidator, IdValidation,
    PostalCodeCountries, PostalCodeLookup, ReverseGeocode, UserAgent, BarcodeValidation,
};

/// A non-2xx response, or a failure to reach LiveAPI.
#[derive(Debug)]
pub struct Error {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.code.is_empty() {
            write!(f, "liveapi: {} ({})", self.message, self.status)
        } else {
            write!(f, "liveapi: {}: {} ({})", self.code, self.message, self.status)
        }
    }
}

impl std::error::Error for Error {}

/// Calls the LiveAPI reference-data endpoints.
/// A token belongs on a server. Do not ship it inside a browser or mobile app.
pub struct Client {
    base_url: String,
    token: String,
    agent: ureq::Agent,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_base_url(token, "https://liveapi.io")
    }

    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            agent: ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).build(),
        }
    }

    pub fn caller_ip(&self) -> Result<IpLocation, Error> {
        self.json("GET", "/v1/ip", &[], None)
    }

    pub fn lookup_ip(&self, ip: &str) -> Result<IpLocation, Error> {
        self.json("GET", &format!("/v1/ip/{}", encode(ip)), &[], None)
    }

    pub fn caller_field(&self, field: &str) -> Result<String, Error> {
        self.text(&format!("/v1/ip/{}", encode(field)))
    }

    pub fn lookup_field(&self, ip: &str, field: &str) -> Result<String, Error> {
        self.text(&format!("/v1/ip/{}/{}", encode(ip), encode(field)))
    }

    pub fn countries(&self, query: Option<&str>) -> Result<Vec<Country>, Error> {
        let mut pairs = Vec::new();
        push(&mut pairs, "q", query);
        self.json("GET", "/v1/countries", &pairs, None)
    }

    pub fn country(&self, code: &str) -> Result<Country, Error> {
        self.json("GET", &format!("/v1/countries/{}", encode(code)), &[], None)
    }

    pub fn divisions(&self, country: &str) -> Result<Vec<Division>, Error> {
        self.json("GET", &format!("/v1/countries/{}/divisions", encode(country)), &[], None)
    }

    pub fn division_by_code(&self, country: &str, code: &str) -> Result<Division, Error> {
        self.json("GET", &format!("/v1/countries/{}/divisions/{}", encode(country), encode(code)), &[], None)
    }

    pub fn division_children_by_code(&self, country: &str, code: &str) -> Result<Vec<Division>, Error> {
        let path = format!("/v1/countries/{}/divisions/{}/children", encode(country), encode(code));
        self.json("GET", &path, &[], None)
    }

    pub fn division(&self, id: &str) -> Result<Division, Error> {
        self.json("GET", &format!("/v1/divisions/{}", encode(id)), &[], None)
    }

    pub fn division_children(&self, id: &str) -> Result<Vec<Division>, Error> {
        self.json("GET", &format!("/v1/divisions/{}/children", encode(id)), &[], None)
    }

    pub fn currencies(&self) -> Result<Vec<crate::types::Currency>, Error> {
        self.json("GET", "/v1/currencies", &[], None)
    }

    pub fn calling_codes(&self, country: Option<&str>) -> Result<Vec<CallingCode>, Error> {
        let mut pairs = Vec::new();
        push(&mut pairs, "country", country);
        self.json("GET", "/v1/phones/codes", &pairs, None)
    }

    pub fn validate_phone(&self, number: &str, country: Option<&str>) -> Result<PhoneValidation, Error> {
        let mut body = json!({ "number": number });
        if let Some(country) = country {
            body["country"] = json!(country);
        }
        self.json("POST", "/v1/phones/validate", &[], Some(body))
    }

    pub fn plate_formats(&self) -> Result<Vec<PlateFormat>, Error> {
        self.json("GET", "/v1/plates/formats", &[], None)
    }

    pub fn validate_plate(&self, plate: &str, country: &str) -> Result<PlateValidation, Error> {
        self.json("POST", "/v1/plates/validate", &[], Some(json!({ "plate": plate, "country": country })))
    }

    pub fn rates(&self, source: Option<&str>, date: Option<&str>) -> Result<Rates, Error> {
        let mut pairs = Vec::new();
        push(&mut pairs, "source", source);
        push(&mut pairs, "date", date);
        self.json("GET", "/v1/rates", &pairs, None)
    }

    pub fn convert(
        &self,
        from: &str,
        to: &str,
        amount: Option<f64>,
        source: Option<&str>,
        date: Option<&str>,
        tcmb_rate: Option<&str>,
    ) -> Result<Conversion, Error> {
        let amount_text = amount.map(|value| value.to_string());
        let mut pairs = vec![("from", from), ("to", to)];
        if let Some(amount) = amount_text.as_deref() {
            pairs.push(("amount", amount));
        }
        push(&mut pairs, "source", source);
        push(&mut pairs, "date", date);
        push(&mut pairs, "tcmb_rate", tcmb_rate);
        self.json("GET", "/v1/rates/convert", &pairs, None)
    }

    pub fn distance(&self, from: &Point, to: &Point) -> Result<Distance, Error> {
        self.json("POST", "/v1/geo/distance", &[], Some(json!({ "from": point_json(from), "to": point_json(to) })))
    }

    pub fn validate_iban(&self, iban: &str) -> Result<IbanValidation, Error> {
        self.json("POST", "/v1/iban/validate", &[], Some(json!({ "iban": iban })))
    }

    pub fn validate_identity(&self, number: &str) -> Result<TrNumberValidation, Error> {
        self.json("POST", "/v1/tr/identity/validate", &[], Some(json!({ "number": number })))
    }

    pub fn validate_tax_number(&self, number: &str) -> Result<TrNumberValidation, Error> {
        self.json("POST", "/v1/tr/tax-number/validate", &[], Some(json!({ "number": number })))
    }

    pub fn holidays(&self, country: &str, year: Option<i32>) -> Result<HolidayList, Error> {
        let year_text = year.map(|value| value.to_string());
        let mut pairs = vec![("country", country)];
        if let Some(year) = year_text.as_deref() {
            pairs.push(("year", year));
        }
        self.json("GET", "/v1/holidays", &pairs, None)
    }

    pub fn business_days(&self, body: &impl Serialize) -> Result<BusinessDays, Error> {
        self.json("POST", "/v1/business-days", &[], Some(serde_json::to_value(body).map_err(decode_setup)?))
    }

    pub fn timezone(&self, latitude: Option<f64>, longitude: Option<f64>, country: Option<&str>) -> Result<Timezone, Error> {
        let latitude_text = latitude.map(|value| value.to_string());
        let longitude_text = longitude.map(|value| value.to_string());
        let mut pairs = Vec::new();
        if let Some(latitude) = latitude_text.as_deref() {
            pairs.push(("lat", latitude));
        }
        if let Some(longitude) = longitude_text.as_deref() {
            pairs.push(("lon", longitude));
        }
        push(&mut pairs, "country", country);
        self.json("GET", "/v1/timezone", &pairs, None)
    }

    pub fn validate_email(&self, email: &str, dns: Option<bool>) -> Result<EmailCheck, Error> {
        let mut body = json!({ "email": email });
        if let Some(dns) = dns {
            body["dns"] = json!(dns);
        }
        self.json("POST", "/v1/email/validate", &[], Some(body))
    }

    pub fn validate_vat(&self, vat_number: &str, country: Option<&str>) -> Result<VatCheck, Error> {
        let mut body = json!({ "vat_number": vat_number });
        if let Some(country) = country {
            body["country"] = json!(country);
        }
        self.json("POST", "/v1/vat/validate", &[], Some(body))
    }

    /// Lists the national ID, tax and company number checks.
    pub fn id_validators(&self) -> Result<Vec<IdValidator>, Error> {
        #[derive(serde::Deserialize)]
        struct List {
            data: Vec<IdValidator>,
        }
        let list: List = self.json("GET", "/v1/validate", &[], None)?;
        Ok(list.data)
    }

    /// Checks a national number offline, for example `("ES", "nif", "12345678Z")`.
    pub fn validate_id(&self, country: &str, kind: &str, value: &str) -> Result<IdValidation, Error> {
        self.json("POST", &format!("/v1/validate/{}/{}", encode(country), encode(kind)), &[], Some(json!({ "value": value })))
    }

    pub fn postal_code_countries(&self) -> Result<PostalCodeCountries, Error> {
        self.json("GET", "/v1/postal-codes", &[], None)
    }

    pub fn postal_code(&self, country: &str, code: &str) -> Result<PostalCodeLookup, Error> {
        self.json("GET", &format!("/v1/postal-codes/{}/{}", encode(country), encode(code)), &[], None)
    }

    /// Country and administrative area for a point.
    pub fn reverse_geocode(&self, latitude: f64, longitude: f64) -> Result<ReverseGeocode, Error> {
        let (lat, lon) = (latitude.to_string(), longitude.to_string());
        self.json("GET", "/v1/reverse", &[("lat", lat.as_str()), ("lon", lon.as_str())], None)
    }

    /// Parses a User-Agent string, for example one your server received.
    pub fn parse_user_agent(&self, ua: &str) -> Result<UserAgent, Error> {
        self.json("POST", "/v1/user-agent", &[], Some(json!({ "ua": ua })))
    }

    /// Checks an EAN-8, UPC-A, EAN-13, GTIN-14 or ISBN check digit.
    pub fn validate_barcode(&self, code: &str) -> Result<BarcodeValidation, Error> {
        self.json("GET", "/v1/barcodes/validate", &[("code", code)], None)
    }

    fn json<T: DeserializeOwned>(&self, method: &str, path: &str, query: &[(&str, &str)], body: Option<serde_json::Value>) -> Result<T, Error> {
        let raw = self.send(method, path, query, body, "application/json")?;
        serde_json::from_str(&raw).map_err(|err| Error { status: 200, code: "invalid_response".into(), message: err.to_string() })
    }

    fn text(&self, path: &str) -> Result<String, Error> {
        self.send("GET", path, &[], None, "text/plain")
    }

    fn send(&self, method: &str, path: &str, query: &[(&str, &str)], body: Option<serde_json::Value>, accept: &str) -> Result<String, Error> {
        if self.token.trim().is_empty() {
            return Err(Error { status: 401, code: "unauthorized".into(), message: "Bearer token is missing.".into() });
        }
        let mut url = format!("{}{}", self.base_url, path);
        if !query.is_empty() {
            let encoded = query.iter().map(|(key, value)| format!("{}={}", encode(key), encode(value))).collect::<Vec<_>>().join("&");
            url.push('?');
            url.push_str(&encoded);
        }
        let request = self
            .agent
            .request(method, &url)
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Accept", accept)
            .set("User-Agent", "liveapi-rust/0.1.0");
        let result = if let Some(body) = body { request.send_json(body) } else { request.call() };
        match result {
            Ok(response) => response.into_string().map_err(|err| Error { status: 0, code: "transport".into(), message: err.to_string() }),
            Err(ureq::Error::Status(status, response)) => Err(api_error(status, response.into_string().unwrap_or_default())),
            Err(err) => Err(Error { status: 0, code: "transport".into(), message: err.to_string() }),
        }
    }
}

fn api_error(status: u16, raw: String) -> Error {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
        if let Some(message) = value.get("error").and_then(|error| error.get("message")).and_then(|message| message.as_str()) {
            let code = value.get("error").and_then(|error| error.get("code")).and_then(|code| code.as_str()).unwrap_or("");
            return Error { status, code: code.to_string(), message: message.to_string() };
        }
    }
    let message = raw.trim();
    Error { status, code: String::new(), message: if message.is_empty() { "LiveAPI request failed.".into() } else { message.into() } }
}

fn push<'a>(query: &mut Vec<(&'a str, &'a str)>, key: &'a str, value: Option<&'a str>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        query.push((key, value));
    }
}

fn point_json(point: &Point) -> serde_json::Value {
    let mut body = serde_json::Map::new();
    if let Some(latitude) = point.latitude {
        body.insert("latitude".into(), json!(latitude));
    }
    if let Some(longitude) = point.longitude {
        body.insert("longitude".into(), json!(longitude));
    }
    if let Some(country) = &point.country {
        body.insert("country".into(), json!(country));
    }
    serde_json::Value::Object(body)
}

fn encode(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn decode_setup(err: serde_json::Error) -> Error {
    Error { status: 0, code: "invalid_request".into(), message: err.to_string() }
}
