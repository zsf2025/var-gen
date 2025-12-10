use std::sync::Arc;

use crate::config::Config;
use crate::storage::Storage;
use crate::utils::error::Error;

pub mod llm;
pub mod rule;

use llm::LLMGenerator;
use rule::RuleGenerator;
use crate::cli::NamingStyle;


#[derive(Debug)]
pub struct Generator {
    pub llm_generator: Option<LLMGenerator>,
    pub rule_generator: RuleGenerator,
    pub storage: Arc<Storage>,
    pub config: Arc<Config>,
}

impl Generator {
    pub fn clone_with_storage_config(&self) -> Result<Self, Error> {
        // 使用配置中的映射文件路径创建新的规则生成器
        let mapping_config_path = self.config.mapping_config_path();
        let rule_generator = if let Some(path) = mapping_config_path {
            RuleGenerator::new_with_config(Some(path))?
        } else {
            RuleGenerator::new()?
        };
        
        Ok(Generator {
            llm_generator: self.llm_generator.clone(),
            rule_generator,
            storage: self.storage.clone(),
            config: self.config.clone(),
        })
    }
}

impl Generator {
    pub fn new(config: Arc<Config>, storage: Arc<Storage>) -> Result<Self, Error> {
        // 尝试初始化大模型生成器
        let llm_generator = match LLMGenerator::new(config.clone()) {
            Ok(generator) => Some(generator),
            Err(e) => {
                // 如果大模型生成器初始化失败，记录错误但继续使用规则引擎
                eprintln!("Warning: Failed to initialize LLM generator: {}", e);
                None
            }
        };
        
        // 初始化规则引擎生成器，使用配置中的映射文件路径
        let mapping_config_path = config.mapping_config_path();
        let rule_generator = if let Some(path) = mapping_config_path {
            RuleGenerator::new_with_config(Some(path))?
        } else {
            RuleGenerator::new()?
        };
        
        Ok(Self {
            llm_generator,
            rule_generator,
            storage,
            config,
        })
    }
    
    pub async fn generate(
        &self,
        description: &str,
        style: NamingStyle,
        force_rule: bool,
    ) -> Result<Vec<String>, Error> {
        // 首先检查是否有网络连接
        let has_network = if !force_rule {
            self.check_network().await
        } else {
            false
        };
        
        // 如果有网络连接且未强制使用规则引擎，尝试使用大模型生成器
        if has_network && self.llm_generator.is_some() && !force_rule {
            if let Some(llm_generator) = &self.llm_generator {
                match llm_generator.generate(description, style).await {
                    Ok(variable_names) => {
                        return Ok(variable_names);
                    }
                    Err(e) => {
                        // 如果大模型生成失败，回退到规则引擎
                        eprintln!("Warning: LLM generation failed: {}", e);
                    }
                }
            }
        }
        
        // 使用规则引擎生成变量名
        let variable_names = self.rule_generator.generate(description, style)?;
        
        Ok(variable_names)
    }
    
    async fn check_network(&self) -> bool {
        // 简单的网络连接检查
        match reqwest::get("https://www.baidu.com").await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

// 用于测试的辅助函数
#[cfg(test)]
#[allow(dead_code)]
pub fn mock_generator() -> Generator {
    use crate::config::mock_config;
    use crate::storage::mock_storage;
    
    let config = Arc::new(mock_config());
    let storage = Arc::new(mock_storage().unwrap());
    
    Generator {
        llm_generator: None,
        rule_generator: RuleGenerator::new().unwrap(),
        storage,
        config,
    }
}

use reqwest;