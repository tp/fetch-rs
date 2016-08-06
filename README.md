# http-fetch

[![Build Status](https://travis-ci.org/tp/http_fetch-rs.svg?branch=master)](https://travis-ci.org/tp/http_fetch-rs)

A small Rust library to fetch web pages.

```rs
let body = fetch_page(url); // fetch the response body of the given URL as String
```

## Development

When running on OS X with a homebrew installed openssl version, make sure to pass on the correct paths to cargo to build:
```sh
env OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include cargo test
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
