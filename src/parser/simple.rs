use crate::error::ParseError;
use crate::model::record::{HttpMethod, LogLevel, LogRecord};
use crate::parser::LogParser;
use chrono::{DateTime, Utc};

/// 日志字段数量
const FIELD_COUNT: usize = 7;

#[derive(Debug, Default)]
pub struct SimpleLogParser;

impl SimpleLogParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_level(s: &str) -> Result<LogLevel, ParseError> {
        match s {
            "INFO" => Ok(LogLevel::Info),
            "ERROR" => Ok(LogLevel::Error),
            "WARN" => Ok(LogLevel::Warn),
            "DEBUG" => Ok(LogLevel::Debug),
            _ => Err(ParseError::InvalidLevel),
        }
    }

    pub fn parse_method(s: &str) -> Result<HttpMethod, ParseError> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(ParseError::InvalidMethod),
        }
    }

    /// "18ms" -> 18
    pub fn parse_latency(s: &str) -> Result<u64, ParseError> {
        let num = s.strip_suffix("ms").ok_or(ParseError::InvalidLatency)?;
        num.parse::<u64>().map_err(|_| ParseError::InvalidLatency)
    }
}

impl LogParser for SimpleLogParser {
    /// 日志格式   2026-09-04T10:00:00.123Z INFO 192.168.1.10 GET /api/v1/orders 200 18ms
    fn parse(&self, line: &str) -> Result<LogRecord, ParseError> {
        let mut parts = line.split_whitespace();

        let mut fields = [""; FIELD_COUNT];

        for field in &mut fields {
            *field = parts.next().ok_or(ParseError::InvalidFileCount)?;
        }

        // 如果超过7个值还能获取到值，说明日志格式非法
        if parts.next().is_some() {
            return Err(ParseError::InvalidFileCount);
        }

        let timestamp = DateTime::parse_from_rfc3339(fields[0])
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| ParseError::InvalidTimestamp)?;
        let level = Self::parse_level(fields[1])?;
        let ip = fields[2];
        let method = Self::parse_method(fields[3])?;
        let path = fields[4];
        let status_code = fields[5]
            .parse::<u16>()
            .map_err(|_| ParseError::InvalidStatusCode)?;
        let latency = Self::parse_latency(fields[6])?;

        Ok(LogRecord {
            timestamp,
            level,
            client_ip: ip.to_string(),
            method,
            path: path.to_string(),
            status_code,
            latency_ms: latency,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_line() {
        let line = "2026-09-04T10:00:00.123Z INFO 192.168.1.10 GET /api/v1/orders 200 18ms";
        let parser = SimpleLogParser::new();
        let record = parser.parse(line).expect("parse failed");
        assert_eq!(record.client_ip, "192.168.1.10");
        assert_eq!(record.level, LogLevel::Info);
        assert_eq!(record.method, HttpMethod::Get);
    }
}
