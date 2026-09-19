use crate::error::ParseError;
use crate::model::record::LogRecord;

pub mod simple;

pub trait LogParser {
    fn parse(&self, line: &str) -> Result<LogRecord, ParseError>;
}
