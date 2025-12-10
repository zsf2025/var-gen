use serde::{Deserialize, Serialize};
use sled::{Config as SledConfig, Db, Tree};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::error::Error;

use super::cli::NamingStyle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub description: String,
    pub style: NamingStyle,
    pub variable_name: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Storage {
    #[allow(dead_code)]
    db: Db,
    history_tree: Tree,
    config_tree: Tree,
}

impl Storage {
    pub fn new() -> Result<Self, Error> {
        // 检查环境变量是否指定了数据库路径
        let db_path = if let Ok(custom_db_path) = std::env::var("DATABASE_URL") {
            std::path::PathBuf::from(custom_db_path)
        } else {
            // 获取用户目录
            let home_dir = dirs::home_dir().ok_or_else(|| Error::StorageError("Failed to get home directory".to_string()))?;
            home_dir.join(".var-gen").join("db")
        };
        
        // 创建目录
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // 初始化数据库
        let config = SledConfig::new()
            .path(db_path)
            .cache_capacity(1024 * 1024 * 10) // 10MB 缓存
            .flush_every_ms(Some(1000)) // 每秒刷新
            .use_compression(true);
        
        let db = config.open()?;
        
        // 打开树
        let history_tree = db.open_tree("history")?;
        let config_tree = db.open_tree("config")?;
        
        Ok(Self {
            db,
            history_tree,
            config_tree,
        })
    }
    
    pub fn save_history(&self, description: &str, style: NamingStyle, variable_name: &str) -> Result<(), Error> {
        // 创建历史记录条目
        let entry = HistoryEntry {
            description: description.to_string(),
            style,
            variable_name: variable_name.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Error::StorageError(format!("Failed to get current time: {}", e)))?
                .as_secs(),
        };
        
        // 序列化
        let entry_bytes = bincode::serialize(&entry)?;
        
        // 生成键（使用时间戳确保唯一性）
        let key = format!("{}:{}:{}", entry.timestamp, style, variable_name);
        
        // 保存到数据库
        self.history_tree.insert(key, entry_bytes)?;
        
        Ok(())
    }
    
    pub fn get_history(&self, limit: usize) -> Result<Vec<HistoryEntry>, Error> {
        let mut entries = Vec::new();
        
        // 按时间戳倒序遍历
        for result in self.history_tree.iter().rev().take(limit) {
            let (_key, value) = result?;
            let entry: HistoryEntry = bincode::deserialize(&value)?;
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    #[allow(dead_code)]
    pub fn check_duplicate(&self, variable_name: &str) -> Result<bool, Error> {
        // 遍历历史记录，检查是否存在相同的变量名
        for result in self.history_tree.iter() {
            let (_key, value) = result?;
            let entry: HistoryEntry = bincode::deserialize(&value)?;
            if entry.variable_name == variable_name {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    #[allow(dead_code)]
    pub fn save_config(&self, key: &str, value: &str) -> Result<(), Error> {
        self.config_tree.insert(key, value.as_bytes())?;
        Ok(())
    }
    
    pub fn get_config(&self, key: &str) -> Result<Option<String>, Error> {
        if let Some(value) = self.config_tree.get(key)? {
            Ok(Some(String::from_utf8(value.to_vec())?))
        } else {
            Ok(None)
        }
    }
    
    pub fn clear_history(&self) -> Result<(), Error> {
        self.history_tree.clear()?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn export_history(&self, path: &Path) -> Result<(), Error> {
        let history = self.get_history(usize::MAX)?;
        
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &history)?;
        
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn import_history(&self, path: &Path) -> Result<(), Error> {
        let file = std::fs::File::open(path)?;
        let history: Vec<HistoryEntry> = serde_json::from_reader(file)?;
        
        for entry in history {
            let key = format!("{}:{}:{}", entry.timestamp, entry.style, entry.variable_name);
            let entry_bytes = bincode::serialize(&entry)?;
            self.history_tree.insert(key, entry_bytes)?;
        }
        
        Ok(())
    }
}

// 用于测试的辅助函数
#[cfg(test)]
#[allow(dead_code)]
pub fn mock_storage() -> Result<Storage, Error> {
    use tempfile::tempdir;
    
    let dir = tempdir()?;
    let db_path = dir.path().join("var-gen-db");
    
    let config = SledConfig::new()
        .path(db_path)
        .temporary(true);
    
    let db = config.open()?;
    let history_tree = db.open_tree("history")?;
    let config_tree = db.open_tree("config")?;
    
    Ok(Storage {
        db,
        history_tree,
        config_tree,
    })
}

