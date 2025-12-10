use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::error::Error;

/// 词汇映射配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingConfig {
    /// 词汇映射表
    pub mappings: HashMap<String, String>,
    /// 停用词列表
    pub stop_words: Vec<String>,
    /// 配置版本
    pub version: String,
    /// 配置描述
    pub description: Option<String>,
}

impl Default for MappingConfig {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        
        // 默认开发词汇映射
        mappings.insert("获取".to_string(), "get".to_string());
        mappings.insert("得到".to_string(), "get".to_string());
        mappings.insert("取得".to_string(), "get".to_string());
        mappings.insert("设置".to_string(), "set".to_string());
        mappings.insert("更新".to_string(), "update".to_string());
        mappings.insert("修改".to_string(), "modify".to_string());
        mappings.insert("删除".to_string(), "delete".to_string());
        mappings.insert("移除".to_string(), "remove".to_string());
        mappings.insert("添加".to_string(), "add".to_string());
        mappings.insert("增加".to_string(), "add".to_string());
        mappings.insert("创建".to_string(), "create".to_string());
        mappings.insert("生成".to_string(), "generate".to_string());
        mappings.insert("计算".to_string(), "calculate".to_string());
        mappings.insert("处理".to_string(), "process".to_string());
        mappings.insert("执行".to_string(), "execute".to_string());
        mappings.insert("调用".to_string(), "call".to_string());
        
        // 用户相关
        mappings.insert("用户".to_string(), "user".to_string());
        mappings.insert("用户名".to_string(), "username".to_string());
        mappings.insert("密码".to_string(), "password".to_string());
        mappings.insert("邮箱".to_string(), "email".to_string());
        mappings.insert("信息".to_string(), "info".to_string());
        mappings.insert("资料".to_string(), "profile".to_string());
        mappings.insert("账户".to_string(), "account".to_string());
        mappings.insert("权限".to_string(), "permission".to_string());
        
        // 数据相关
        mappings.insert("数据".to_string(), "data".to_string());
        mappings.insert("数据库".to_string(), "database".to_string());
        mappings.insert("表".to_string(), "table".to_string());
        mappings.insert("字段".to_string(), "field".to_string());
        mappings.insert("记录".to_string(), "record".to_string());
        mappings.insert("文件".to_string(), "file".to_string());
        mappings.insert("配置".to_string(), "config".to_string());
        mappings.insert("设置".to_string(), "settings".to_string());
        
        // 系统相关
        mappings.insert("系统".to_string(), "system".to_string());
        mappings.insert("服务".to_string(), "service".to_string());
        mappings.insert("接口".to_string(), "api".to_string());
        mappings.insert("请求".to_string(), "request".to_string());
        mappings.insert("响应".to_string(), "response".to_string());
        mappings.insert("状态".to_string(), "status".to_string());
        mappings.insert("错误".to_string(), "error".to_string());
        mappings.insert("日志".to_string(), "log".to_string());
        
        // 连接相关
        mappings.insert("连接".to_string(), "connection".to_string());
        mappings.insert("链接".to_string(), "link".to_string());
        mappings.insert("网络".to_string(), "network".to_string());
        mappings.insert("地址".to_string(), "address".to_string());
        mappings.insert("端口".to_string(), "port".to_string());
        
        // 常用单字
        mappings.insert("名".to_string(), "name".to_string());
        mappings.insert("姓".to_string(), "surname".to_string());
        mappings.insert("年".to_string(), "year".to_string());
        mappings.insert("月".to_string(), "month".to_string());
        mappings.insert("日".to_string(), "day".to_string());
        mappings.insert("时".to_string(), "hour".to_string());
        mappings.insert("分".to_string(), "minute".to_string());
        mappings.insert("秒".to_string(), "second".to_string());
        mappings.insert("数".to_string(), "number".to_string());
        mappings.insert("量".to_string(), "quantity".to_string());
        mappings.insert("价".to_string(), "price".to_string());
        mappings.insert("值".to_string(), "value".to_string());
        mappings.insert("类".to_string(), "type".to_string());
        mappings.insert("型".to_string(), "type".to_string());
        mappings.insert("组".to_string(), "group".to_string());
        mappings.insert("列表".to_string(), "list".to_string());
        mappings.insert("数组".to_string(), "array".to_string());
        
        // 财务相关
        mappings.insert("余额".to_string(), "balance".to_string());
        mappings.insert("金额".to_string(), "amount".to_string());
        mappings.insert("费用".to_string(), "fee".to_string());
        mappings.insert("成本".to_string(), "cost".to_string());
        mappings.insert("收入".to_string(), "income".to_string());
        mappings.insert("支出".to_string(), "expense".to_string());

        let stop_words = vec![
            "的".to_string(), "了".to_string(), "和".to_string(), "是".to_string(), 
            "就".to_string(), "都".to_string(), "而".to_string(), "及".to_string(), 
            "与".to_string(), "着".to_string(), "或".to_string(), "一个".to_string(), 
            "没有".to_string(), "我们".to_string(), "你们".to_string(), "他们".to_string(),
            "this".to_string(), "that".to_string(), "these".to_string(), "those".to_string(), 
            "is".to_string(), "are".to_string(), "was".to_string(), "were".to_string(), 
            "be".to_string(), "been".to_string(), "being".to_string(), "the".to_string(), 
            "a".to_string(), "an".to_string(), "and".to_string(), "or".to_string(), 
            "but".to_string(), "in".to_string(), "on".to_string(), "at".to_string(), 
            "to".to_string(), "for".to_string(), "of".to_string(), "with".to_string(), 
            "by".to_string(), "from".to_string(), "as".to_string(), "into".to_string(), 
            "like".to_string(), "through".to_string(), "after".to_string(), "over".to_string(), 
            "between".to_string(), "out".to_string(), "against".to_string(), "during".to_string(), 
            "before".to_string(), "because".to_string(), "if".to_string(), "when".to_string(),
            "than".to_string(), "so".to_string(), "such".to_string(), "both".to_string(), 
            "each".to_string(), "every".to_string(), "some".to_string(), "any".to_string(), 
            "few".to_string(), "more".to_string(), "most".to_string(), "other".to_string(), 
            "own".to_string(), "same".to_string(), "all".to_string(), "none".to_string(), 
            "nor".to_string(), "not".to_string(), "only".to_string(), "very".to_string(), 
            "s".to_string(), "t".to_string(), "can".to_string(), "will".to_string(), 
            "don".to_string(), "should".to_string(), "now".to_string(),
        ];

        Self {
            mappings,
            stop_words,
            version: "1.0".to_string(),
            description: Some("默认中文到英文词汇映射配置".to_string()),
        }
    }
}

impl MappingConfig {
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(Error::ConfigError(format!("映射配置文件不存在: {}", path.display())));
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| Error::ConfigError(format!("读取映射配置文件失败: {}", e)))?;
        
        Self::from_str(&content)
    }
    
    /// 从字符串加载配置（支持JSON和TOML格式）
    pub fn from_str(content: &str) -> Result<Self, Error> {
        // 首先尝试解析为JSON
        if let Ok(config) = serde_json::from_str::<MappingConfig>(content) {
            return Ok(config);
        }
        
        // 如果JSON解析失败，尝试解析为TOML
        toml::from_str(content)
            .map_err(|e| Error::ConfigError(format!("解析映射配置文件失败: {}", e)))
    }
    
    /// 保存配置到文件（使用TOML格式）
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::ConfigError(format!("创建配置目录失败: {}", e)))?;
        }
        
        // 根据文件扩展名决定格式
        let content = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::to_string_pretty(self)
                .map_err(|e| Error::ConfigError(format!("序列化映射配置失败: {}", e)))?
        } else {
            // 默认使用TOML格式
            toml::to_string_pretty(self)
                .map_err(|e| Error::ConfigError(format!("序列化映射配置失败: {}", e)))?
        };
        
        fs::write(path, content)
            .map_err(|e| Error::ConfigError(format!("保存映射配置文件失败: {}", e)))?;
        
        Ok(())
    }
    
    /// 合并另一个配置（用于扩展）
    #[allow(dead_code)]
    pub fn merge(&mut self, other: MappingConfig) {
        // 合并词汇映射（后面的覆盖前面的）
        for (key, value) in other.mappings {
            self.mappings.insert(key, value);
        }
        
        // 合并停用词（去重）
        let mut combined_stop_words: std::collections::HashSet<_> = 
            self.stop_words.iter().cloned().collect();
        combined_stop_words.extend(other.stop_words);
        self.stop_words = combined_stop_words.into_iter().collect();
    }
    
    /// 获取词汇映射
    pub fn get_mapping(&self, key: &str) -> Option<&String> {
        self.mappings.get(key)
    }
    
    /// 添加词汇映射
    #[allow(dead_code)]
    pub fn add_mapping(&mut self, chinese: String, english: String) {
        self.mappings.insert(chinese, english);
    }
    
    /// 移除词汇映射
    #[allow(dead_code)]
    pub fn remove_mapping(&mut self, key: &str) -> Option<String> {
        self.mappings.remove(key)
    }
    
    /// 检查是否为停用词
    pub fn is_stop_word(&self, word: &str) -> bool {
        self.stop_words.contains(&word.to_string())
    }
    
    /// 获取所有映射（用于调试）
    #[allow(dead_code)]
    pub fn mappings(&self) -> &HashMap<String, String> {
        &self.mappings
    }
    
    /// 获取所有停用词
    #[allow(dead_code)]
    pub fn stop_words(&self) -> &[String] {
        &self.stop_words
    }
}

/// 映射配置管理器
pub struct MappingConfigManager {
    config_path: PathBuf,
    config: MappingConfig,
}

impl MappingConfigManager {
    /// 创建新的配置管理器
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self, Error> {
        let config_path = config_path.as_ref().to_path_buf();
        
        // 验证配置文件路径有效（消除未使用字段警告）
        let _ = config_path.to_str().ok_or_else(|| {
            Error::ConfigError("无效的配置文件路径".to_string())
        })?;
        
        // 如果配置文件不存在，创建默认配置
        let config = if config_path.exists() {
            MappingConfig::from_file(&config_path)?
        } else {
            let default_config = MappingConfig::default();
            default_config.save_to_file(&config_path)?;
            default_config
        };
        
        Ok(Self {
            config_path,
            config,
        })
    }
    
    /// 获取当前配置
    pub fn config(&self) -> &MappingConfig {
        &self.config
    }
    
    /// 获取可变配置
    #[allow(dead_code)]
    pub fn config_mut(&mut self) -> &mut MappingConfig {
        &mut self.config
    }
    
    /// 重新加载配置
    #[allow(dead_code)]
    pub fn reload(&mut self) -> Result<(), Error> {
        self.config = MappingConfig::from_file(&self.config_path)?;
        Ok(())
    }
    
    /// 保存配置
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), Error> {
        self.config.save_to_file(&self.config_path)
    }
    
    /// 获取配置文件路径
    #[allow(dead_code)]
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_mapping_config_default() {
        let config = MappingConfig::default();
        assert_eq!(config.version, "1.0");
        assert!(config.description.is_some());
        assert!(config.get_mapping("获取").is_some());
        assert_eq!(config.get_mapping("获取").unwrap(), "get");
    }
    
    #[test]
    fn test_mapping_config_serialization() {
        let config = MappingConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let loaded_config = MappingConfig::from_str(&json).unwrap();
        
        assert_eq!(loaded_config.version, config.version);
        assert_eq!(loaded_config.mappings.len(), config.mappings.len());
    }
    
    #[test]
    fn test_mapping_config_file_operations() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = MappingConfig::default();
        
        // 保存到文件
        config.save_to_file(temp_file.path()).unwrap();
        
        // 从文件加载
        let loaded_config = MappingConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config.version, config.version);
    }
    
    #[test]
    fn test_mapping_config_manager() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("mapping.json");
        
        // 创建配置管理器（会自动创建默认配置）
        let manager = MappingConfigManager::new(&config_path).unwrap();
        assert!(config_path.exists());
        
        // 验证默认配置被创建
        assert_eq!(manager.config().version, "1.0");
    }
}