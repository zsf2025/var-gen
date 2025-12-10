/*
 * @Description: 
 * @Author: zhangfu 18072150332@163.com
 * @Date: 2025-12-09 19:01:22
 * @LastEditors: zhangfu 18072150332@163.com
 * @LastEditTime: 2025-12-10 21:08:02
 */
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generator error: {0}")]
    GeneratorError(String),
    
    #[error("LLM error: {0}")]
    LLMError(String),
    
    #[error("Rule engine error: {0}")]
    RuleError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),
    
    #[error("Sled error: {0}")]
    SledError(#[from] sled::Error),
    
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error),
    
    #[error("Config error: {0}")]
    ConfigError(String),
}

// 为其他错误类型提供转换
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::GeneratorError(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::GeneratorError(s)
    }
}