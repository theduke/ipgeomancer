mod object;
mod parser;

pub use self::{
    object::Object,
    parser::{ParseError, parse_objects},
};
