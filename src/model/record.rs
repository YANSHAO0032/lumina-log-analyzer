use chrono::{ DateTime,Utc};
#[derive(Debug, Clone,PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone,PartialEq)]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Delete,
    Patch,
}

/// 日志格式   2026-09-04T10:00:00.123Z INFO 192.168.1.10 GET /api/v1/orders 200 18ms
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub client_ip: String,
    pub method: HttpMethod,
    pub path: String,
    pub status_code: u16,
    pub latency_ms: u64,
}