use jieba_rs::Jieba;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use crate::utils::error::Error;
use crate::cli::NamingStyle;
use crate::config::mapping::{MappingConfig, MappingConfigManager};

pub struct RuleGenerator {
    chinese_tokenizer: Jieba,
    english_stemmer: Stemmer,
    stop_words: HashSet<String>,
    chinese_to_english: HashMap<String, String>,
    mapping_config: Option<MappingConfig>,
}

impl std::fmt::Debug for RuleGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleGenerator")
            .field("chinese_tokenizer", &"Jieba")
            .field("english_stemmer", &"Stemmer")
            .field("stop_words", &format!("HashSet with {} elements", self.stop_words.len()))
            .finish()
    }
}

impl RuleGenerator {
    pub fn new() -> Result<Self, Error> {
        Self::new_with_config(None)
    }
    
    pub fn new_with_config(mapping_config_path: Option<&str>) -> Result<Self, Error> {
        // 初始化中文分词器
        let chinese_tokenizer = Jieba::new();
        
        // 初始化英文词根提取器
        let english_stemmer = Stemmer::create(Algorithm::English);
        
        // 加载配置
        let (mapping_config, stop_words, chinese_to_english) = 
            Self::load_mapping_config(mapping_config_path)?;
        
        Ok(Self {
            chinese_tokenizer,
            english_stemmer,
            stop_words,
            chinese_to_english,
            mapping_config,
        })
    }
    
    fn load_mapping_config(mapping_config_path: Option<&str>) -> Result<(Option<MappingConfig>, HashSet<String>, HashMap<String, String>), Error> {
        if let Some(config_path) = mapping_config_path {
            // 使用自定义配置文件
            let manager = MappingConfigManager::new(config_path)?;
            let config = manager.config().clone();
            
            // 从配置构建映射表和停用词
            let mut chinese_to_english = HashMap::new();
            let mut stop_words = HashSet::new();
            
            // 加载词汇映射
            for (chinese, english) in &config.mappings {
                chinese_to_english.insert(chinese.clone(), english.clone());
            }
            
            // 加载停用词
            for word in &config.stop_words {
                stop_words.insert(word.clone());
            }
            
            Ok((Some(config), stop_words, chinese_to_english))
        } else {
            // 使用默认配置
            let stop_words = load_default_stop_words()?;
            let chinese_to_english = load_default_chinese_to_english_map()?;
            Ok((None, stop_words, chinese_to_english))
        }
    }
    
    pub fn generate(&self, description: &str, style: NamingStyle) -> Result<Vec<String>, Error> {
        // 文本预处理
        let processed_text = self.preprocess_text(description);
        
        // 分词
        let tokens = self.tokenize(&processed_text)?;
        
        // 过滤停用词
        let filtered_tokens = self.filter_stop_words(&tokens);
        
        // 根据命名规范转换
        let variable_name = self.convert_to_style(&filtered_tokens, style)?;
        
        Ok(vec![variable_name])
    }
    
    fn preprocess_text(&self, text: &str) -> String {
        // 移除特殊字符，保留字母、数字、中文和空格
        let re = regex::Regex::new(r"[^\p{L}\p{N}\p{Han}\s]").unwrap();
        let cleaned = re.replace_all(text, "");
        
        // 转换为小写
        cleaned.to_lowercase()
    }
    
    fn tokenize(&self, text: &str) -> Result<Vec<String>, Error> {
        let mut tokens = Vec::new();
        
        // 按空格分割文本
        let parts: Vec<&str> = text.split_whitespace().collect();
        
        for part in parts {
            // 检查是否包含中文字符
            if part.chars().any(|c| is_chinese_char(c)) {
                // 中文分词
                let chinese_tokens = self.chinese_tokenizer.cut(part, false);
                
                // 将中文词汇转换为英文
                for chinese_token in chinese_tokens {
                    // 优先使用配置文件的映射
                    let translation = if let Some(config) = &self.mapping_config {
                        config.get_mapping(chinese_token)
                            .or_else(|| self.chinese_to_english.get(chinese_token))
                            .cloned()
                    } else {
                        self.chinese_to_english.get(chinese_token).cloned()
                    };
                    
                    if let Some(english_translation) = translation {
                        // 如果找到英文翻译，使用翻译结果
                        tokens.push(english_translation);
                    } else {
                        // 如果没有找到翻译，尝试将每个中文字符单独翻译
                        let mut translated_parts = Vec::new();
                        for ch in chinese_token.chars() {
                            if is_chinese_char(ch) {
                                let ch_str = ch.to_string();
                                
                                // 优先使用配置文件的映射
                                let char_translation = if let Some(config) = &self.mapping_config {
                                    config.get_mapping(&ch_str)
                                        .or_else(|| self.chinese_to_english.get(&ch_str))
                                        .cloned()
                                } else {
                                    self.chinese_to_english.get(&ch_str).cloned()
                                };
                                
                                if let Some(english) = char_translation {
                                    translated_parts.push(english);
                                } else {
                                    // 如果单个字符也没有翻译，保留原始字符
                                    translated_parts.push(ch_str);
                                }
                            } else {
                                translated_parts.push(ch.to_string());
                            }
                        }
                        // 将翻译的部分合并
                        tokens.push(translated_parts.join(""));
                    }
                }
            } else {
                // 英文处理
                // 检查是否包含连字符
                if part.contains('-') {
                    let subparts: Vec<&str> = part.split('-').collect();
                    tokens.extend(subparts.iter().map(|s| s.to_string()));
                } else {
                    // 检查是否包含驼峰命名
                    let camel_tokens = self.split_camel_case(part);
                    tokens.extend(camel_tokens);
                }
            }
        }
        
        Ok(tokens)
    }
    
    fn split_camel_case(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        // 使用正则表达式拆分驼峰命名
        let re = regex::Regex::new(r"([a-z])([A-Z])").unwrap();
        let converted = re.replace_all(text, "$1 $2");
        
        // 转换为小写并按空格分割
        let lowercased = converted.to_lowercase();
        let parts: Vec<&str> = lowercased.split_whitespace().collect();
        
        for part in parts {
            // 只对短单词进行词干提取，保留长单词的完整性
            if part.len() <= 6 {
                let stem = self.english_stemmer.stem(part);
                result.push(stem.to_string());
            } else {
                result.push(part.to_string());
            }
        }
        
        result
    }
    
    fn filter_stop_words(&self, tokens: &[String]) -> Vec<String> {
        if tokens.is_empty() {
            return Vec::new();
        }
        
        let filtered: Vec<String> = tokens
            .iter()
            .filter(|token| {
                // 如果token包含中文字符，不过滤（保留中文词汇）
                if token.chars().any(|c| is_chinese_char(c)) {
                    return true;
                }
                
                // 优先使用配置文件中的停用词检查
                if let Some(config) = &self.mapping_config {
                    !config.is_stop_word(token)
                } else {
                    // 回退到内置停用词
                    !self.stop_words.contains(*token)
                }
            })
            .cloned()
            .collect();
        
        // 如果过滤后没有剩余词汇，但原始有中文词汇，保留第一个中文词汇
        if filtered.is_empty() {
            for token in tokens {
                if token.chars().any(|c| is_chinese_char(c)) {
                    return vec![token.clone()];
                }
            }
        }
        
        filtered
    }
    
    fn convert_to_style(&self, tokens: &[String], style: NamingStyle) -> Result<String, Error> {
        if tokens.is_empty() {
            return Err(Error::RuleError("No valid tokens after processing".to_string()));
        }
        
        match style {
            NamingStyle::Camel => self.to_camel_case(tokens),
            NamingStyle::Pascal => self.to_pascal_case(tokens),
            NamingStyle::Snake => self.to_snake_case(tokens),
            NamingStyle::Kebab => self.to_kebab_case(tokens),
            NamingStyle::UpperSnake => self.to_upper_snake_case(tokens),
            NamingStyle::LowerCamel => self.to_camel_case(tokens), // 与 camel 相同
        }
    }
    
    fn to_camel_case(&self, tokens: &[String]) -> Result<String, Error> {
        let mut result = String::new();
        
        for (i, token) in tokens.iter().enumerate() {
            if i == 0 {
                // 第一个词保持小写
                result.push_str(token);
            } else {
                // 后续词首字母大写
                let capitalized = token.chars()
                    .enumerate()
                    .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                    .collect::<String>();
                result.push_str(&capitalized);
            }
        }
        
        Ok(result)
    }
    
    fn to_pascal_case(&self, tokens: &[String]) -> Result<String, Error> {
        let mut result = String::new();
        
        for token in tokens {
            // 每个词首字母大写
            let capitalized = token.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>();
            result.push_str(&capitalized);
        }
        
        Ok(result)
    }
    
    fn to_snake_case(&self, tokens: &[String]) -> Result<String, Error> {
        Ok(tokens.join("_"))
    }
    
    fn to_kebab_case(&self, tokens: &[String]) -> Result<String, Error> {
        Ok(tokens.join("-"))
    }
    
    fn to_upper_snake_case(&self, tokens: &[String]) -> Result<String, Error> {
        Ok(tokens.join("_").to_uppercase())
    }
}

/// 判断字符是否为中文字符
fn is_chinese_char(c: char) -> bool {
    matches!(c, '\u{4e00}'..='\u{9fff}' | '\u{3400}'..='\u{4dbf}' | '\u{20000}'..='\u{2a6df}' | '\u{2a700}'..='\u{2b73f}' | '\u{2b740}'..='\u{2b81f}' | '\u{2b820}'..='\u{2ceaf}' | '\u{2ceb0}'..='\u{2ebef}')
}

fn load_default_chinese_to_english_map() -> Result<HashMap<String, String>, Error> {
    static CHINESE_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
    
    Ok(CHINESE_MAP.get_or_init(|| {
        // 常用中文词汇到英文的映射
        let mut map = HashMap::new();
        
        // 常用开发词汇
        map.insert("获取".to_string(), "get".to_string());
        map.insert("得到".to_string(), "get".to_string());
        map.insert("取得".to_string(), "get".to_string());
        map.insert("设置".to_string(), "set".to_string());
        map.insert("更新".to_string(), "update".to_string());
        map.insert("修改".to_string(), "modify".to_string());
        map.insert("删除".to_string(), "delete".to_string());
        map.insert("移除".to_string(), "remove".to_string());
        map.insert("添加".to_string(), "add".to_string());
        map.insert("增加".to_string(), "add".to_string());
        map.insert("创建".to_string(), "create".to_string());
        map.insert("生成".to_string(), "generate".to_string());
        map.insert("计算".to_string(), "calculate".to_string());
        map.insert("处理".to_string(), "process".to_string());
        map.insert("执行".to_string(), "execute".to_string());
        map.insert("调用".to_string(), "call".to_string());
        
        // 用户相关
        map.insert("用户".to_string(), "user".to_string());
        map.insert("用户名".to_string(), "username".to_string());
        map.insert("密码".to_string(), "password".to_string());
        map.insert("邮箱".to_string(), "email".to_string());
        map.insert("信息".to_string(), "info".to_string());
        map.insert("资料".to_string(), "profile".to_string());
        map.insert("账户".to_string(), "account".to_string());
        map.insert("权限".to_string(), "permission".to_string());
        
        // 数据相关
        map.insert("数据".to_string(), "data".to_string());
        map.insert("数据库".to_string(), "database".to_string());
        map.insert("表".to_string(), "table".to_string());
        map.insert("字段".to_string(), "field".to_string());
        map.insert("记录".to_string(), "record".to_string());
        map.insert("文件".to_string(), "file".to_string());
        map.insert("配置".to_string(), "config".to_string());
        map.insert("设置".to_string(), "settings".to_string());
        
        // 系统相关
        map.insert("系统".to_string(), "system".to_string());
        map.insert("服务".to_string(), "service".to_string());
        map.insert("接口".to_string(), "api".to_string());
        map.insert("请求".to_string(), "request".to_string());
        map.insert("响应".to_string(), "response".to_string());
        map.insert("状态".to_string(), "status".to_string());
        map.insert("错误".to_string(), "error".to_string());
        map.insert("日志".to_string(), "log".to_string());
        
        // 连接相关
        map.insert("连接".to_string(), "connection".to_string());
        map.insert("链接".to_string(), "link".to_string());
        map.insert("网络".to_string(), "network".to_string());
        map.insert("地址".to_string(), "address".to_string());
        map.insert("端口".to_string(), "port".to_string());
        
        // 常用单字
        map.insert("名".to_string(), "name".to_string());
        map.insert("姓".to_string(), "surname".to_string());
        map.insert("年".to_string(), "year".to_string());
        map.insert("月".to_string(), "month".to_string());
        map.insert("日".to_string(), "day".to_string());
        map.insert("时".to_string(), "hour".to_string());
        map.insert("分".to_string(), "minute".to_string());
        map.insert("秒".to_string(), "second".to_string());
        map.insert("数".to_string(), "number".to_string());
        map.insert("量".to_string(), "quantity".to_string());
        map.insert("价".to_string(), "price".to_string());
        map.insert("值".to_string(), "value".to_string());
        map.insert("类".to_string(), "type".to_string());
        map.insert("型".to_string(), "type".to_string());
        map.insert("类".to_string(), "class".to_string());
        map.insert("组".to_string(), "group".to_string());
        map.insert("列表".to_string(), "list".to_string());
        map.insert("数组".to_string(), "array".to_string());
        
        // 财务相关
        map.insert("余额".to_string(), "balance".to_string());
        map.insert("金额".to_string(), "amount".to_string());
        map.insert("费用".to_string(), "fee".to_string());
        map.insert("成本".to_string(), "cost".to_string());
        map.insert("收入".to_string(), "income".to_string());
        map.insert("支出".to_string(), "expense".to_string());
        
        map
    }).clone())
}

fn load_default_stop_words() -> Result<HashSet<String>, Error> {
    static STOP_WORDS: OnceLock<HashSet<String>> = OnceLock::new();
    
    Ok(STOP_WORDS.get_or_init(|| {
        // 内置的基本停用词列表
        let default_stop_words = [
            "的", "了", "和", "是", "就", "都", "而", "及", "与", "着", "或", "一个", "没有", "我们", "你们", "他们",
            "this", "that", "these", "those", "is", "are", "was", "were", "be", "been", "being", "the", "a", "an",
            "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "from", "as", "into", "like",
            "through", "after", "over", "between", "out", "against", "during", "before", "because", "if", "when",
            "than", "so", "such", "both", "each", "every", "some", "any", "few", "more", "most", "other", "own",
            "same", "all", "none", "nor", "not", "only", "very", "s", "t", "can", "will", "don", "should", "now"
        ];
        
        let mut stop_words = HashSet::new();
        for word in default_stop_words {
            stop_words.insert(word.to_string());
        }
        
        stop_words
    }).clone())
}

// 用于测试的辅助函数
#[cfg(test)]
#[allow(dead_code)]
pub fn mock_rule_generator() -> RuleGenerator {
    RuleGenerator::new().unwrap()
}

use regex;