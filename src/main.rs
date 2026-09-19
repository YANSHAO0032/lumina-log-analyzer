use crate::error::AppError;

pub mod error;
pub mod model;
pub mod parser;

fn main() -> Result<(), AppError> {
    Ok(())
}
