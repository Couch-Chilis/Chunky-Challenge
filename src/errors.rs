use std::error::Error;

#[derive(Debug)]
pub struct UnknownDirection;

impl std::fmt::Display for UnknownDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown direction")
    }
}

impl Error for UnknownDirection {}

#[derive(Debug)]
pub struct UnknownObjectType;

impl std::fmt::Display for UnknownObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown object type")
    }
}

impl Error for UnknownObjectType {}
