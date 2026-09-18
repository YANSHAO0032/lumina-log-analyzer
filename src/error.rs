use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {

    #[error("IO error:{0}")]
    Io(#[from]std::io::Error),

    #[error("Parse error:{0}")]
    Parse(#[from] ParseError),

    #[error("Invalid argument:{0}")]
    InvalidArgument(String),
}

/// 日志格式   2026-09-04T10:00:00.123Z INFO 192.168.1.10 GET /api/v1/orders 200 18ms
#[derive(Debug, Error,PartialEq)]
pub enum ParseError {
    #[error("日志必须包含7个字段")]
    InvalidFileCount,
    #[error("时间格式不合法")]
    InvalidTimestamp,
    #[error("日志级别不合法")]
    InvalidLevel,
    #[error("IP不合法")]
    InvalidIp,
    #[error("请求方法不合法")]
    InvalidMethod,
    #[error("请求路径不合法")]
    InvalidPath,
    #[error("状态码不合法,状态码必须是100到599的整数")]
    InvalidStatusCode,
    #[error("耗时不合法,耗时必须是带ms后缀的非负整数")]
    InvalidLatency,
}