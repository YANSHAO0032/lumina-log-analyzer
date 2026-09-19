use chrono::{DateTime, Utc};

/// 日志级别枚举
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// 请求方法枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// 日志时间戳
    pub timestamp: DateTime<Utc>,
    /// 日志级别
    pub level: LogLevel,
    /// 请求ip
    pub client_ip: String,
    /// 请求方法
    pub method: HttpMethod,
    /// 请求路径
    pub path: String,
    /// 响应码
    pub status_code: u16,
    /// 耗时
    pub latency_ms: u64,
}
