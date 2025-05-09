use crate::Object;

/// Parse an RPSL database and return a vector of RPSL objects.
pub fn parse_objects(input: &str) -> Result<Vec<Object>, ParseError> {
    todo!();
}

#[derive(Debug)]
pub enum ParseError {
    // TODO: Add error variants
}
