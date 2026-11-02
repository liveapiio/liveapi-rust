# liveapi-rust

Rust client for the [LiveAPI](https://liveapi.io) reference-data API.

The token spends calls on your account. Keep it in an environment variable on your server. Do not put it in a browser page or a mobile app.

```rust
let client = liveapi::Client::new(std::env::var("LIVEAPI_TOKEN").unwrap());
let phone = client.validate_phone("0555 111 22 33", Some("TR"))?;
```

Use `client.country_with_geoshape("TR")` to request the optional GeoJSON country boundary and attribution.

```toml
liveapi = { git = "https://github.com/liveapiio/liveapi-rust" }
```

New in this version: `id_validators`, `validate_id`, `postal_code_countries`, `postal_code`, `reverse_geocode`, `parse_user_agent`, `validate_barcode`, and the IP risk fields `is_tor`, `is_hosting`, `hosting_provider`.

```rust
let id = client.validate_id("ES", "nif", "12345678Z")?;
let barcode = client.validate_barcode("4006381333931")?;
let postal = client.postal_code("DE", "10115")?;
let area = client.reverse_geocode(41.01, 28.97)?;
let ua = client.parse_user_agent("Mozilla/5.0 ...")?;
```

Docs: https://liveapi.io/docs

IP lookups include `attribution` and `attribution_url`. Keep those fields when you store or show a DB-IP result. Postal-code results identify LiveAPI's in-house dataset. A validation result with `valid: false` is still HTTP 200 and uses one call. Only a non-2xx response is free.
