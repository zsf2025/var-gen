use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::Client;


use crate::config::Config;
use crate::utils::error::Error;

use crate::cli::NamingStyle;

#[derive(Debug, Clone)]
pub struct LLMGenerator {
    client: Client,
    api_key: Option<String>,
    model: String,
    cache: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl LLMGenerator {
    pub fn new(config: Arc<Config>) -> Result<Self, Error> {
        let api_key = config.api_key().map(|s| s.to_string());
        
        // 如果没有配置 API 密钥，返回错误
        if api_key.is_none() {
            return Err(Error::LLMError("API key not configured".to_string()));
        }
        
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        let model = config.model().to_string();
        
        Ok(Self {
            client,
            api_key,
            model,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn generate(&self, description: &str, style: NamingStyle) -> Result<Vec<String>, Error> {
        // 检查缓存
        let cache_key = format!("{}:{}", description, style);
        if let Some(result) = self.cache.read().unwrap().get(&cache_key) {
            return Ok(result.clone());
        }
        
        // 构建提示词
        let prompt: String = self.build_prompt(description, style);
        
        // 调用大模型 API
        let response: String = self.call_api(&prompt).await?;
        
        // 解析结果
        let variable_names: Vec<String> = self.parse_response(&response)?;
        
        // 更新缓存
        self.cache.write().unwrap().insert(cache_key, variable_names.clone());
        
        Ok(variable_names)
    }
    
    fn build_prompt(&self, description: &str, style: NamingStyle) -> String {
        let style_desc = match style {
            NamingStyle::Camel => "camelCase (e.g., userName)",
            NamingStyle::Pascal => "PascalCase (e.g., UserName)",
            NamingStyle::Snake => "snake_case (e.g., user_name)",
            NamingStyle::Kebab => "kebab-case (e.g., user-name)",
            NamingStyle::UpperSnake => "UPPER_SNAKE_CASE (e.g., USER_NAME)",
            NamingStyle::LowerCamel => "lowerCamelCase (e.g., userName)",
        };
        
        format!(
            "请根据以下描述生成符合{}格式的变量名，仅输出变量名，多个候选用逗号分隔，无需额外解释：\n{}",
            style_desc,
            description
        )
    }
    
    async fn call_api(&self, prompt: &str) -> Result<String, Error> {
        // 根据模型选择不同的 API
        match self.model.as_str() {
            "qwen-tiny" => self.call_qwen_api(prompt).await,
            "xinghuo-lite" => self.call_xinghuo_api(prompt).await,
            _ => Err(Error::LLMError(format!("Unsupported model: {}", self.model))),
        }
    }
    
    async fn call_qwen_api(&self, prompt: &str) -> Result<String, Error> {
        let api_key = self.api_key.as_ref().ok_or_else(|| Error::LLMError("API key not configured".to_string()))?;
        
        let url = "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";
        
        let request_body = serde_json::json!({
            "model": "qwen-tiny",
            "input": {
                "prompt": prompt
            },
            "parameters": {
                "result_format": "text",
                "temperature": 0.1
            }
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(Error::LLMError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        let text = response_json["output"]["text"]
            .as_str()
            .ok_or_else(|| Error::LLMError("Invalid response format".to_string()))?;
        
        Ok(text.to_string())
    }
    
    async fn call_xinghuo_api(&self, prompt: &str) -> Result<String, Error> {
        let api_key = self.api_key.as_ref().ok_or_else(|| Error::LLMError("API key not configured".to_string()))?;
        
        let url = "https://spark-api-open.xf-yun.com/v1/chat/completions";
        
        let request_body = serde_json::json!({
            "model": "spark-lite",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.1,
            "max_tokens": 200
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(Error::LLMError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        let text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::LLMError("Invalid response format".to_string()))?;
        
        Ok(text.to_string())
    }
    
    fn parse_response(&self, response: &str) -> Result<Vec<String>, Error> {
        // 清理响应文本
        let cleaned = response.trim();
        
        // 按逗号分隔多个变量名
        let variable_names: Vec<String> = cleaned
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if variable_names.is_empty() {
            return Err(Error::LLMError("No valid variable names found in response".to_string()));
        }
        
        Ok(variable_names)
    }
}

// 用于测试的辅助函数
#[cfg(test)]
#[allow(dead_code)]
pub fn mock_llm_generator() -> LLMGenerator {
    let config = Arc::new(crate::config::mock_config_with_api_key("mock-api-key"));
    
    LLMGenerator::new(config).unwrap()
}