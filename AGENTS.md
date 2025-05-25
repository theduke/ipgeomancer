# AI Agent Instructions

The ipgeomancer project is a collection of libraries and tools for IP geolocation,
written in Rust.

## Development

NEVER BE LAZY OR SLOPPY WHEN DEVELOPING!
IMPLEMENT TO THE BEST OF YOUR ABILITY, considering aspects not explicitly mentioned
in the instructions, such as performance, security, and maintainability, Rust best practices,
... OR ELSE YOU MIGHT BE REPLACED BY A BETTER AGENT!

Always be thorough and diligent when making changes to the codebase.
Make sure to handle all edge cases and ensure that the code is robust and well-tested.

* Check the code with `cargo check --offline`
* Test the code with `cargo test --offline`

NOTE: cargo compilation can take a noticeable amount of time, so be patient and
don't rush to abort check/test commands.

When fixing bugs, first write a regression test that fails when running `cargo test`.
Then do the fixes and make sure the test passes.

Before finalizing any changes, please ensure the following:

* Ensure that all public APIs have doc comments that are descriptive and clear,
  but not overly verbose.
* Lint the code with `cargo clippy --offline -- -D warnings`
* Make sure the code is formatted with `cargo fmt`
