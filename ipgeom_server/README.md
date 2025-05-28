# ipgeom_server

`ipgeom_server` implements the web server for the
[ipgeomancer](https://github.com/theduke/ipgeomancer) project, exposing
REST APIs and a web UI.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/ipgeom_server.svg
[crates-url]: https://crates.io/crates/ipgeom_server
[docs-badge]: https://docs.rs/ipgeom_server/badge.svg
[docs-url]: https://docs.rs/ipgeom_server

## Description

This crate provides the axum based HTTP server that powers the ipgeomancer web
interface and API. It integrates the individual crates into a cohesive service.

## Logging

[`tower_http::trace`](https://docs.rs/tower-http/latest/tower_http/trace)
is used for logging of requests.

The default log filter will enable request and response logging with the
`tower_http::trace=debug` directive.
Remove logs by setting the `RUST_LOG` environment variable to a desired value.


## License

This project is licensed under the terms of the MIT License. See [LICENSE](../LICENSE) for details.
