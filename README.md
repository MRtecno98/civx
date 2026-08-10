[![CI](https://github.com/MRtecno98/civx/actions/workflows/ci.yml/badge.svg)](https://github.com/MRtecno98/civx/actions/workflows/ci.yml)
![Packaging Status](https://img.shields.io/crates/v/civx)
![License: MIT](https://img.shields.io/badge/license-MIT-34d058.svg)
# CivX
Asynchronous Rust client for the new [CivitAI site api](https://developer.civitai.com/site/).

## Description
This crate provides Rust structs for most API calls and responses, authentication is provided through a bearer token.

```rust
// If you don't need auth you can use CivitAI::new()
let client = civx::CivitAI::new_auth("==TOKEN==")?;

// Fetches metadata for the latest 20 published models
let models = client.list_models()
    .pagination(Some(20), None, None)
    .sort(SortKind::Newest)
    .send().await?;

// Some requests only need a single argument
let illustrious = client.get_model(795765).await?;
```

For more information about available methods and authentication check out the [official documentation](https://developer.civitai.com/site/reference/).

### Feature flags
-  `enums`: Downloads and generates code for all available enums (such as base models, model types, file types, etc.)
    from the API itself at compile time and generates Rust wrappers to use them in requests. *Requires a network connection at compile time.*
- `network-tests`: Enables execution of API tests against the actual servers, requires a network connection.

## Development
To build and run tests
```
cargo test
```

To run online tests (against the real servers)

```
cargo test --features network-tests
```

To run authenticated tests, first create a `test_token` containing the token an nothing 
else (check for newlines) and put it in the `tests` folder, then run

```
cargo test --features network-tests -- --include-ignored
```

In all the above commands you can enable the `enums` feature to also download the latests enums from CivitAI (see above)
