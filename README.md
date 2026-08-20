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

### Pagination
Both cursor and page navigation is supported, follow CivitAI's [recommendations](https://developer.civitai.com/site/guide/pagination) to know which to choose.

In a nutshell cursors remain consistent across catalogue changes and are best used for automatic iteration of a big section of the catalogue,
while pages support seeking and are better suited for user interfaces.

Note that when using page navigation the requested page number times the request element limit *must be less than 1000*. Consider using
cursors if you require deeper iteration.

```rust
// ===== Using cursors (and streams)
let stream = pin!(models.stream());

while let Some(model) = stream.try_next().await? {
    // automatically requests more cursors
}

// ===== Using pages
let (current_page, page_count) = models.index()
    .ok_or("Request doesn't support page iteration");

// This drains the items from the page, and returns
// owned values for the contents.
// Iterating over a &mut is also equivalent
for model in models.items() {
    // process the contents of the current page
}

// When you need to, you can request a new page.
// This consumes the old one so be sure to drain
// the items by iterating it first!
let new_page = models.seek_page(current_page + 1).await?
    .unwrap_or("No more pages left"); 
```

Using streams lets the library decide which iteration method to use, prioritizing in this order: cursors, `next_page` URLs provided by the api, and falling
back to page iteration if none are present. Most of the time this means using cursors since most requests support it.

### Feature flags
- `enums`: Downloads and generates code for all available enums (such as base models, model types, file types, etc.)
  from the API itself at compile time and generates Rust wrappers to use them in requests. For library consumers this
  guarantees having an up-to-date enum list at the cost of a network call for each compilation.
  *Requires a network connection at compile time.*
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
