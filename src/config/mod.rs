use std::sync::Arc;
use std::time::Duration;

use crate::storage::Storage;
use crate::utils::error::Error;

pub mod mapping;

#[derive(Debug)]
pub struct Config {
    storage: Arc<Storage>,
    default_style: String,
    api_key: Option<String>,
    model: String,
    cache_enabled: bool,
    cache_ttl: Duration,
    mapping_config_path: Option<String>,
}

impl Config {
    pub fn new(storage: Arc<Storage>) -> Result<Self, Error> {
        // 从存储中加载配置
        let default_style = storage
            .get_config("default_style")?
            .unwrap_or_else(|| "snake".to_string());
            
        let api_key = storage.get_config("api_key")?;
        
        let model = storage
            .get_config("model")?
            .unwrap_or_else(|| "qwen-tiny".to_string());
            
        let cache_enabled = storage
            .get_config("cache_enabled")?
            .map(|s| s.parse().unwrap_or(true))
            .unwrap_or(true);
            
        let cache_ttl = storage
            .get_config("cache_ttl")?
            .map(|s| Duration::from_secs(s.parse().unwrap_or(86400)))
            .unwrap_or(Duration::from_secs(86400)); // 默认 24 小时
            
        let mapping_config_path = storage.get_config("mapping_config_path")?;
            
        Ok(Self {
            storage,
            default_style,
            api_key,
            model,
            cache_enabled,
            cache_ttl,
            mapping_config_path,
        })
    }
    
    // 配置访问方法
    #[allow(dead_code)]
    pub fn default_style(&self) -> &str {
        &self.default_style
    }
    
    pub fn mapping_config_path(&self) -> Option<&str> {
        self.mapping_config_path.as_deref()
    }
    
    #[allow(dead_code)]
    pub fn set_mapping_config_path(&mut self, path: &str) -> Result<(), Error> {
        self.mapping_config_path = Some(path.to_string());
        self.storage.save_config("mapping_config_path", path)
    }
    
    #[allow(dead_code)]
    pub fn clear_mapping_config_path(&mut self) -> Result<(), Error> {
        self.mapping_config_path = None;
        self.storage.save_config("mapping_config_path", "")
    }
    
    #[allow(dead_code)]
    pub fn set_default_style(&mut self, style: &str) -> Result<(), Error> {
        self.default_style = style.to_string();
        self.storage.save_config("default_style", style)
    }
    
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }
    
    #[allow(dead_code)]
    pub fn set_api_key(&mut self, api_key: &str) -> Result<(), Error> {
        self.api_key = Some(api_key.to_string());
        self.storage.save_config("api_key", api_key)
    }
    
    #[allow(dead_code)]
    pub fn clear_api_key(&mut self) -> Result<(), Error> {
        self.api_key = None;
        self.storage.save_config("api_key", "")
    }
    
    pub fn model(&self) -> &str {
        &self.model
    }
    
    #[allow(dead_code)]
    pub fn set_model(&mut self, model: &str) -> Result<(), Error> {
        self.model = model.to_string();
        self.storage.save_config("model", model)
    }
    
    #[allow(dead_code)]
    pub fn cache_enabled(&self) -> bool {
        self.cache_enabled
    }
    
    #[allow(dead_code)]
    pub fn set_cache_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        self.cache_enabled = enabled;
        self.storage.save_config("cache_enabled", &enabled.to_string())
    }
    
    #[allow(dead_code)]
    pub fn cache_ttl(&self) -> Duration {
        self.cache_ttl
    }
    
    #[allow(dead_code)]
    pub fn set_cache_ttl(&mut self, ttl: Duration) -> Result<(), Error> {
        self.cache_ttl = ttl;
        self.storage.save_config("cache_ttl", &ttl.as_secs().to_string())
    }
    
    // 重置所有配置到默认值
    #[allow(dead_code)]
    pub fn reset_to_defaults(&mut self) -> Result<(), Error> {
        self.default_style = "snake".to_string();
        self.api_key = None;
        self.model = "qwen-tiny".to_string();
        self.cache_enabled = true;
        self.cache_ttl = Duration::from_secs(86400);
        self.mapping_config_path = None;
        
        self.storage.save_config("default_style", "snake")?;
        self.storage.save_config("api_key", "")?;
        self.storage.save_config("model", "qwen-tiny")?;
        self.storage.save_config("cache_enabled", "true")?;
        self.storage.save_config("cache_ttl", "86400")?;
        self.storage.save_config("mapping_config_path", "")?;
        
        Ok(())
    }
}

// 用于测试的辅助函数
#[cfg(test)]
#[allow(dead_code)]
pub fn mock_config() -> Config {
    use crate::storage::mock_storage;
    
    let storage = Arc::new(mock_storage().unwrap());
    
    Config {
        storage,
        default_style: "snake".to_string(),
        api_key: None,
        model: "qwen-tiny".to_string(),
        cache_enabled: true,
        cache_ttl: Duration::from_secs(86400),
        mapping_config_path: None,
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub fn mock_config_with_api_key(api_key: &str) -> Config {
    let mut config = mock_config();
    config.api_key = Some(api_key.to_string());
    config
}

#[cfg(test)]
#[allow(dead_code)]
pub fn mock_config_with_mapping_path(path: &str) -> Config {
    let mut config = mock_config();
    config.mapping_config_path = Some(path.to_string());
    config
}