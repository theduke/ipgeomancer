# AI Agent Instructions

The ipgeomancer project is a collection of libraries and tools for IP geolocation,
written in Rust.

## Development

Always be thorough and diligent when making changes to the codebase.
Make sure to handle all edge cases and ensure that the code is robust and well-tested.

* Check the code with `cargo check --offline`
* Test the code with `cargo test --offline`

When fixing bugs, first write a regression test that fails when running `cargo test`.
Then do the fixes and make sure the test passes.

Before finalizing any changes, please ensure the following:

* Ensure that all public APIs have doc comments that are descriptive and clear,
  but not overly verbose.
* Lint the code with `cargo clippy -D warnings --offline`
* Make sure the code is formatted with `cargo fmt`
