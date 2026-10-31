# liveapi-rust

Rust client for the [LiveAPI](https://liveapi.io) reference-data API.

The token spends calls on your account. Keep it in an environment variable on your server. Do not put it in a browser page or a mobile app.

```rust
let client = liveapi::Client::new(std::env::var("LIVEAPI_TOKEN").unwrap());
let phone = client.validate_phone("0555 111 22 33", Some("TR"))?;
```

```toml
liveapi = { git = "https://github.com/liveapiio/liveapi-rust" }
```

Docs: https://liveapi.io/docs

IP lookups include `attribution` and `attribution_url`. Keep those fields when you store or show a DB-IP result. A validation result with `valid: false` is still HTTP 200 and uses one call. Only a non-2xx response is free.
