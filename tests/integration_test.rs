/*
 * @Description: 
 * @Author: zhangfu 18072150332@163.com
 * @Date: 2025-12-09 19:01:22
 * @LastEditors: zhangfu 18072150332@163.com
 * @LastEditTime: 2025-12-10 22:38:52
 */
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_basic_generation() -> Result<(), Box<dyn std::error::Error>> {

    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("var-gen")?;
    
    cmd.arg("--description").arg("user name").arg("--style").arg("snake")
       .env("DATABASE_URL", "test_basic_generation.db");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("user_name"));
    
    Ok(())
}

#[test]
fn test_chinese_generation() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("var-gen")?;
    
    cmd.arg("--description").arg("用户名").arg("--style").arg("camel")
       .env("DATABASE_URL", "test_chinese_generation.db");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("username"));
    
    Ok(())
}

#[test]
fn test_batch_processing() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    
    // 创建输入文件
    fs::write(&input_path, "user name\npassword\ndatabase connection")?;
    
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("var-gen")?;
    cmd.arg("--file").arg(input_path.to_str().unwrap())
       .arg("--output").arg(output_path.to_str().unwrap())
       .arg("--style").arg("snake")
       .env("DATABASE_URL", "test_batch_processing.db");
    
    cmd.assert().success();
    
    // 检查输出文件
    let output = fs::read_to_string(&output_path)?;
    assert!(output.contains("user_name"));
    assert!(output.contains("password"));
    assert!(output.contains("database_connection"));
    
    Ok(())
}

#[test]
fn test_all_styles() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(deprecated)]
    let mut cmd = Command::cargo_bin("var-gen")?;
    
    cmd.arg("--all-styles")
       .env("DATABASE_URL", "test_all_styles.db");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("驼峰命名法"))
        .stdout(predicate::str::contains("帕斯卡命名法"))
        .stdout(predicate::str::contains("下划线命名法"))
        .stdout(predicate::str::contains("短横线命名法"))
        .stdout(predicate::str::contains("大写下划线命名法"))
        .stdout(predicate::str::contains("小驼峰命名法"));
    
    Ok(())
}