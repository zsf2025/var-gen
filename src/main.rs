/*
 * @Description: 
 * @Author: zhangfu 18072150332@163.com
 * @Date: 2025-12-09 19:01:22
 * @LastEditors: zhangfu 18072150332@163.com
 * @LastEditTime: 2025-12-10 20:34:29
 */
use clap::Parser;
use std::process;
use tokio::runtime::Runtime;

mod cli;
mod config;
mod generator;
mod storage;
mod utils;

fn main() {
    // 初始化日志（如果需要）
    // env_logger::init();

    // 解析命令行参数
    let args = cli::Args::parse();

    // 创建 tokio 运行时
    let rt = Runtime::new().expect("Failed to create runtime");

    // 运行主逻辑
    if let Err(e) = rt.block_on(cli::run(args)) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}