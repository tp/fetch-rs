# fetch

[![Build Status](https://travis-ci.org/tp/fetch-rs.svg?branch=master)](https://travis-ci.org/tp/fetch-rs)

[Documentation](https://docs.rs/fetch)

A small Rust library to fetch web pages.

```rust
extern crate fetch;

fn main() {
    let body = fetch::fetch_body("https://www.rust-lang.org/en-US/"); // fetch the response body of the given URL as String
}
```

## Development

When running on OS X with a homebrew installed openssl version, make sure the correct paths are passed to cargo. See the `scripts` folder for details.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
