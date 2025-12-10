use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use colored::Colorize;


use crate::config::Config;
use crate::generator::Generator;
use crate::storage::Storage;
use crate::utils::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 变量描述文本
    #[arg(short, long)]
    description: Option<String>,
    
    /// 命名规范风格
    #[arg(short, long, default_value = "snake")]
    style: NamingStyle,
    
    /// 强制使用交互式模式
    #[arg(long)]
    interactive: bool,
    
    /// 批量处理文件路径
    #[arg(long)]
    file: Option<String>,
    
    /// 输出文件路径
    #[arg(long)]
    output: Option<String>,
    
    /// 显示所有支持的命名规范
    #[arg(long)]
    all_styles: bool,
    
    /// 显示历史记录
    #[arg(long)]
    history: bool,
    
    /// 清除历史记录
    #[arg(long)]
    clear_history: bool,
    
    /// 强制使用规则引擎（不调用大模型）
    #[arg(long)]
    force_rule: bool,
    
    /// 设置API密钥
    #[arg(long)]
    set_api_key: Option<String>,
    
    /// 清除API密钥
    #[arg(long)]
    clear_api_key: bool,
    
    /// 词汇映射配置文件路径
    #[arg(long)]
    mapping_config: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum NamingStyle {
    #[clap(name = "camel")]
    Camel,
    #[clap(name = "pascal")]
    Pascal,
    #[clap(name = "snake")]
    Snake,
    #[clap(name = "kebab")]
    Kebab,
    #[clap(name = "upper_snake")]
    UpperSnake,
    #[clap(name = "lower_camel")]
    LowerCamel,
}

impl std::fmt::Display for NamingStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingStyle::Camel => write!(f, "驼峰命名法 (camelCase)"),
            NamingStyle::Pascal => write!(f, "帕斯卡命名法 (PascalCase)"),
            NamingStyle::Snake => write!(f, "下划线命名法 (snake_case)"),
            NamingStyle::Kebab => write!(f, "短横线命名法 (kebab-case)"),
            NamingStyle::UpperSnake => write!(f, "大写下划线命名法 (UPPER_SNAKE_CASE)"),
            NamingStyle::LowerCamel => write!(f, "小驼峰命名法 (lowerCamelCase)"),
        }
    }
}

pub async fn run(args: Args) -> Result<(), Error> {
    // 初始化存储
    let storage = Arc::new(Storage::new()?);
    
    // 初始化配置
    let mut config = Config::new(storage.clone())?;
    
    // 如果指定了映射配置文件路径，更新配置
    if let Some(mapping_path) = &args.mapping_config {
        config.set_mapping_config_path(mapping_path)?;
        println!("使用自定义词汇映射配置文件: {}", mapping_path);
    }
    
    let config = Arc::new(config);
    
    // 初始化生成器
    let generator = Generator::new(config.clone(), storage.clone())?;
    
    // 处理各种命令行参数
    if args.all_styles {
        print_all_styles();
        return Ok(());
    }
    
    if args.history {
        print_history(storage.clone())?;
        return Ok(());
    }
    
    if args.clear_history {
        clear_history(storage.clone())?;
        return Ok(());
    }
    
    // 处理API密钥设置
    if let Some(api_key) = args.set_api_key {
        let mut config = Config::new(storage.clone())?;
        config.set_api_key(&api_key)?;
        println!("API密钥已设置");
        return Ok(());
    }
    
    if args.clear_api_key {
        let mut config = Config::new(storage.clone())?;
        config.clear_api_key()?;
        println!("API密钥已清除");
        return Ok(());
    }
    
    // 处理批量文件
    if let Some(file_path) = args.file {
        process_file(&file_path, &args.output, &generator, args.style, args.force_rule).await?;
        return Ok(());
    }
    
    // 处理交互式模式或单变量生成
    if args.interactive || args.description.is_none() {
        run_interactive(storage.clone(), config.clone(), Arc::new(generator.clone_with_storage_config()?)).await?;
    } else {
        let description = args.description.unwrap();
        generate_single(&description, args.style, &generator, storage.clone(), args.force_rule).await?;
    }
    
    Ok(())
}

fn print_all_styles() {
    println!("支持的命名规范：");
    for style in NamingStyle::value_variants() {
        println!("  - {}", style);
    }
}

fn print_history(storage: Arc<Storage>) -> Result<(), Error> {
    let history = storage.get_history(20)?;
    if history.is_empty() {
        println!("暂无历史记录");
        return Ok(());
    }
    
    println!("最近生成的变量名：");
    for entry in history {
        println!(
            "  - {} (描述: \"{}\", 风格: {})",
            entry.variable_name, entry.description, entry.style
        );
    }
    
    Ok(())
}

fn clear_history(storage: Arc<Storage>) -> Result<(), Error> {
    let confirm = dialoguer::Confirm::new()
        .with_prompt("确定要清除所有历史记录吗？")
        .default(false)
        .interact()?;
    
    if confirm {
        storage.clear_history()?;
        println!("历史记录已清除");
    }
    
    Ok(())
}

async fn process_file(
    file_path: &str,
    output_path: &Option<String>,
    generator: &Generator,
    style: NamingStyle,
    force_rule: bool,
) -> Result<(), Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut results = Vec::new();
    
    for line in reader.lines() {
        let description = line?;
        if description.trim().is_empty() {
            continue;
        }
        
        let variable_names: Vec<String> = generator.generate(&description, style, force_rule).await?;
        results.push((description, variable_names));
    }
    
    match output_path {
        Some(path) => {
            let mut file = File::create(path)?;
            for (description, variable_names) in &results {
                writeln!(file, "描述: {}", description)?;
                writeln!(file, "变量名: {}", variable_names.join(", "))?;
                writeln!(file)?;
            }
            println!("结果已保存到 {}", path);
        },
        None => {
            for (description, variable_names) in &results {
                println!("描述: {}", description);
                println!("变量名: {}", variable_names.join(", "));
                println!();
            }
        },
    }
    
    Ok(())
}

async fn run_interactive(
    storage: Arc<Storage>,
    _config: Arc<Config>,
    generator: Arc<Generator>,
) -> Result<(), Error> {
    println!("=== var-gen 交互模式 ===");
    println!("提示：输入变量描述生成变量名，输入空行退出。");
    println!("支持的命名规范：snake, camel, pascal, kebab, upper_snake, lower_camel");
    println!();
    
    let stdin = std::io::stdin();
    let mut input_buffer = String::new();
    
    loop {
        // 清空缓冲区
        input_buffer.clear();
        
        // 显示提示并刷新输出
        print!("请输入变量描述（输入空行退出）：");
        std::io::stdout().flush()?;
        
        // 读取输入
        match stdin.read_line(&mut input_buffer) {
            Ok(0) => {
                // EOF，正常退出
                println!("\n检测到输入结束，退出交互模式。");
                break;
            }
            Ok(_) => {
                let description = input_buffer.trim();
                if description.is_empty() {
                    println!("输入为空，退出交互模式。");
                    break;
                }
                
                println!("您输入的描述是：\"{}\"", description);
                
                // 使用箭头选择命名规范
                let styles = vec![
                    ("snake_case", "下划线命名法 (snake_case)"),
                    ("camelCase", "驼峰命名法 (camelCase)"),
                    ("PascalCase", "帕斯卡命名法 (PascalCase)"),
                    ("kebab-case", "短横线命名法 (kebab-case)"),
                    ("UPPER_SNAKE_CASE", "大写下划线命名法 (UPPER_SNAKE_CASE)"),
                    ("lowerCamelCase", "小驼峰命名法 (lowerCamelCase)"),
                ];
                
                let selection = dialoguer::Select::new()
                    .with_prompt("请选择命名规范")
                    .items(&styles.iter().map(|(name, desc)| format!("{} - {}", name, desc)).collect::<Vec<_>>())
                    .default(0)
                    .interact()?;
                
                let style = match selection {
                    0 => NamingStyle::Snake,
                    1 => NamingStyle::Camel,
                    2 => NamingStyle::Pascal,
                    3 => NamingStyle::Kebab,
                    4 => NamingStyle::UpperSnake,
                    5 => NamingStyle::LowerCamel,
                    _ => NamingStyle::Snake, // 默认情况
                };
                
                println!("正在生成变量名...");
                
                // 生成变量名
                match generator.generate(&description, style, false).await {
                    Ok(variable_names) => {
                        // 显示结果
                        println!("\n生成的变量名：");
                        for (i, name) in variable_names.iter().enumerate() {
                            println!("  {}. {}", i + 1, name.green());
                        }
                        
                        // 询问是否保存到历史记录
                        let save_to_history = dialoguer::Confirm::new()
                            .with_prompt("是否保存到历史记录？")
                            .default(false)
                            .interact()?;
                        
                        if save_to_history && !variable_names.is_empty() {
                            match storage.save_history(&description, style, &variable_names[0]) {
                                Ok(_) => println!("已保存到历史记录。"),
                                Err(e) => eprintln!("保存历史记录失败：{}", e),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("生成变量名失败：{}", e);
                    }
                }
                
                println!("\n---");
            }
            Err(e) => {
                eprintln!("读取输入失败：{}，退出交互模式。", e);
                break;
            }
        }
    }
    
    println!("退出交互模式。");
    Ok(())
}

async fn generate_single(
    description: &str,
    style: NamingStyle,
    generator: &Generator,
    storage: Arc<Storage>,
    force_rule: bool,
) -> Result<(), Error> {
    let variable_names: Vec<String> = generator.generate(description, style, force_rule).await?;
    
    println!("生成的变量名：");
    for (i, name) in variable_names.iter().enumerate() {
        println!("  {}. {}", i + 1, name.green());
    }
    
    // 自动保存到历史记录
    if !variable_names.is_empty() {
        storage.save_history(description, style, &variable_names[0])?;
    }
    
    Ok(())
}

use std::sync::Arc;