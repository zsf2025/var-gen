# var-gen - 智能变量命名工具

一个基于大模型驱动的智能变量命名工具，帮助开发者快速生成符合多种编码规范的变量名。

## 🌟 核心亮点

- **🧠 双引擎架构**：LLM智能引擎 + 规则引擎，确保在线/离线都能正常工作
- **🔄 智能降级**：网络异常时自动切换到规则引擎，保证服务可用性
- **⚙️ 高度可配置**：支持自定义词汇映射、API配置、缓存设置等
- **🎯 精准翻译**：内置丰富的中文词汇映射，支持用户自定义扩展
- **📦 多格式支持**：支持6种主流命名规范（snake_case、camelCase、PascalCase等）
- **🚀 批量处理**：支持文件批量处理，提高工作效率
- **💾 历史记录**：自动保存生成历史，支持查看和管理
- **🎨 交互友好**：提供交互式命令行界面，操作直观便捷

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- 网络连接（用于大模型API，可选）

### 安装

#### 从源码构建
```bash
# 克隆项目
git clone https://github.com/yourusername/var-gen.git
cd var-gen

# 编译安装
cargo build --release

# 添加到PATH（可选）
cargo install --path .
```

#### 从预编译二进制文件安装
访问 [GitHub Releases](https://github.com/yourusername/var-gen/releases) 页面下载对应平台的预编译二进制文件。

### 基本使用
```bash
# 生成变量名（交互式模式）
var-gen

# 直接生成
var-gen --description "获取用户信息" --style snake
# 输出：get_user_info

# 批量处理
var-gen --file ./descriptions.txt --style camel --output ./variables.txt
```

### 配置API（可选）
```bash
# 设置API密钥以获得更好的翻译效果
var-gen --set-api-key "your-api-key"
```

## 功能特点

- 🚀 **智能命名生成**：基于大语言模型理解语义，生成贴切的英文变量名
- 🌐 **中英文混合支持**：支持中文描述，智能转换为英文变量名
- 🎨 **多种命名规范**：支持 snake_case、camelCase、PascalCase、kebab-case、UPPER_SNAKE_CASE、lowerCamelCase
- 💡 **交互式体验**：友好的命令行交互界面，支持箭头键选择
- 📚 **规则引擎**：内置中文词汇映射，离线状态也能正常工作
- 🔄 **批量处理**：支持从文件读取多个变量描述进行批量处理
- 💾 **历史记录**：自动保存生成历史，支持查看和复用
- ⚙️ **灵活配置**：支持自定义API密钥、模型选择、缓存设置等
- 🔧 **可扩展映射**：支持自定义词汇映射配置文件
- 🛡️ **错误处理**：完善的错误提示和异常处理机制

### 从预编译二进制文件安装

访问[GitHub Releases](https://github.com/yourusername/var-gen/releases)页面下载对应平台的预编译二进制文件。

## 使用方法

### 命令行模式

```bash
# 基本使用
var-gen --description "用户名称" --style snake

# 输出：
# 生成的变量名：
#   1. user_name

# 指定命名规范
var-gen --description "用户登录时间" --style camel

# 输出：
# 生成的变量名：
#   1. userLoginTime

# 批量处理
var-gen --file ./vars.txt --style snake --output ./results.txt
```

### 交互式模式

```bash
# 启动交互式模式
var-gen

# 或者强制使用交互式模式
var-gen --interactive
```

交互式模式支持箭头键选择功能：
- 使用↑↓箭头键选择命名规范
- 使用←→箭头键或y/n选择是否保存历史记录
- 更直观、便捷的用户体验

### 查看历史记录

```bash
var-gen --history
```

### 清除历史记录

```bash
var-gen --clear-history
```

### 查看支持的命名规范

```bash
var-gen --all-styles
```

## 配置

配置文件位于用户目录下的`.var-gen`文件夹：

```
~/.var-gen/
├── config.toml  # 配置文件
└── db/          # 数据库目录
```

### 配置API密钥

首次使用大模型功能前，需要配置API密钥：

```bash
# 编辑配置文件
# 在config.toml中添加：
# api_key = "your-api-key"
# model = "qwen-tiny"  # 或 "xinghuo-lite"
```

### 配置词汇映射文件

var-gen 支持自定义词汇映射配置文件，允许用户扩展中文到英文的翻译规则：

1. **创建映射配置文件**：
   ```bash
   # 复制示例文件作为模板
   cp mapping-config-example.json my-mapping.json
   ```

2. **编辑配置文件**：
   - 添加或修改词汇映射关系
   - 定义停用词列表
   - 支持版本管理和描述信息

3. **配置使用路径**：
   ```bash
   # 在config.toml中添加映射文件路径：
   mapping_config_path = "/path/to/your/mapping-config.json"
   ```

4. **验证配置**：
   ```bash
   # 测试配置是否生效
   var-gen --description "您的测试描述" --style snake
   ```
```

映射配置文件格式参见项目中的 `mapping-config-example.json` 文件，支持：
- 自定义中文到英文的词汇映射
- 添加或修改停用词规则
- 覆盖默认的映射关系

配置优先级：
1. 配置文件中的映射优先于内置映射
2. 配置文件中的停用词优先于内置停用词

## 支持的命名规范

- **camel (驼峰命名法)**：如 `userName`
- **pascal (帕斯卡命名法)**：如 `UserName`
- **snake (下划线命名法)**：如 `user_name`
- **kebab (短横线命名法)**：如 `user-name`
- **upper_snake (大写下划线命名法)**：如 `USER_NAME`
- **lower_camel (小驼峰命名法)**：如 `userName`（与camel相同）

## 工作原理

### 双引擎架构

var-gen采用双引擎架构，确保在各种环境下都能提供稳定的变量名生成服务：

1. **大模型引擎**（首选）
   - 利用大语言模型的语义理解能力
   - 生成更智能、更符合编程习惯的变量名
   - 需要配置有效的API密钥

2. **规则引擎**（降级方案）
   - 基于预定义规则的本地处理
   - 支持中文到英文的自动翻译
   - 无需网络连接，响应更快

### 中文转英文规则引擎

当使用中文描述且大模型不可用时，规则引擎会自动：

1. **中文分词**：使用Jieba分词器对中文描述进行分词
2. **词汇翻译**：将中文词汇映射到对应的英文单词
3. **命名规范转换**：根据指定的命名规范生成最终变量名

#### 自定义词汇映射

规则引擎支持通过配置文件自定义词汇映射，允许用户扩展翻译规则：

1. **创建映射配置文件**：复制 `mapping-config-example.json` 作为模板
2. **编辑映射关系**：添加或修改中文到英文的词汇映射
3. **配置路径**：在 `config.toml` 中设置 `mapping_config_path`
4. **优先级规则**：配置文件中的映射优先于内置映射

例如，自定义映射配置：
```json
{
  "version": "1.0",
  "description": "自定义映射配置示例",
  "mappings": {
    "客户": "client",
    "订单": "order",
    "支付": "payment"
  },
  "stop_words": ["的", "了", "客户", "订单"]
}
```

#### 翻译示例

内置翻译示例：
- "获取用户信息" → 分词为["获取", "用户", "信息"] → 翻译为["get", "user", "info"] → snake_case → "get_user_info"
- "创建数据库连接" → 分词为["创建", "数据库", "连接"] → 翻译为["create", "database", "connection"] → camelCase → "createDatabaseConnection"

## 示例

### 中文描述示例

```bash
# 中文转英文变量名（规则引擎）
var-gen --description "获取用户信息" --style snake
# 输出：get_user_info

var-gen --description "创建数据库连接" --style camel
# 输出：createDatabaseConnection

var-gen --description "更新用户密码" --style pascal
# 输出：UpdateUserPassword

var-gen --description "删除文件记录" --style kebab
# 输出：delete-file-record

# 中文描述（大模型模式）
var-gen --description "用户登录状态" --style snake
# 输出：user_login_status

var-gen --description "数据库连接池大小" --style camel
# 输出：databaseConnectionPoolSize
```

### 英文描述示例

```bash
var-gen --description "user login time" --style pascal
# 输出：UserLoginTime

var-gen --description "maximum retry count" --style upper_snake
# 输出：MAXIMUM_RETRY_COUNT
```

## 详细功能说明

### 中文转英文编码名功能

当大模型API不可用或未配置时，规则引擎会自动启用，提供中文到英文的转换功能：

**支持的词汇映射：**

#### 开发相关
- 获取/得到/取得 → get
- 设置 → set
- 更新 → update
- 删除/移除 → delete/remove
- 添加/增加 → add
- 创建 → create
- 生成 → generate
- 计算 → calculate
- 处理 → process
- 执行 → execute

#### 用户相关
- 用户 → user
- 用户名 → username
- 密码 → password
- 邮箱 → email
- 信息 → info
- 资料 → profile
- 账户 → account
- 权限 → permission

#### 数据相关
- 数据 → data
- 数据库 → database
- 表 → table
- 字段 → field
- 记录 → record
- 文件 → file
- 配置 → config
- 设置 → settings

#### 系统相关
- 系统 → system
- 服务 → service
- 接口 → api
- 请求 → request
- 响应 → response
- 状态 → status
- 错误 → error
- 日志 → log

### 交互式模式详细说明

交互式模式提供了直观的箭头选择界面：

1. **启动交互式模式**
   ```bash
   var-gen
   # 或
   var-gen --interactive
   ```

2. **操作流程**
   - 输入变量描述（如：获取用户信息）
   - 使用↑↓箭头键选择命名规范
   - 查看生成的变量名结果
   - 使用←→箭头键或y/n选择是否保存历史记录

3. **示例交互过程**
   ```
   === var-gen 交互模式 ===
   提示：输入变量描述生成变量名，输入空行退出。
   
   请输入变量描述（输入空行退出）：获取用户信息
   您输入的描述是："获取用户信息"
   ? 请选择命名规范 ›
   > snake_case - 下划线命名法 (snake_case)
     camelCase - 驼峰命名法 (camelCase)
     PascalCase - 帕斯卡命名法 (PascalCase)
     kebab-case - 短横线命名法 (kebab-case)
     UPPER_SNAKE_CASE - 大写下划线命名法 (UPPER_SNAKE_CASE)
     lowerCamelCase - 小驼峰命名法 (lowerCamelCase)
   
   正在生成变量名...
   
   生成的变量名：
     1. get_user_info
   ? 是否保存到历史记录？ [y/N]:
   ```

### 批量处理模式

支持从文件读取多个描述进行批量处理：

1. **创建输入文件**（如：vars.txt）
   ```
   获取用户信息
   创建数据库连接
   更新用户密码
   删除文件记录
   ```

2. **执行批量处理**
   ```bash
   var-gen --file ./vars.txt --style snake --output ./results.txt
   ```

3. **查看输出文件**（results.txt）
   ```
   get_user_info
   create_database_connection
   update_user_password
   delete_file_record
   ```

### 配置管理

#### API密钥配置
```bash
# 设置API密钥
var-gen --set-api-key "your-api-key"

# 清除API密钥
var-gen --clear-api-key
```

#### 支持的模型
- `qwen-tiny`（默认）：通义千问
- `xinghuo-lite`：讯飞星火

#### 配置文件示例（~/.var-gen/config.toml）
```toml
api_key = "your-api-key"
model = "qwen-tiny"
default_style = "snake"
cache_enabled = true
cache_ttl = 86400
```

### 高级用法

#### 强制使用规则引擎
```bash
# 跳过LLM，直接使用规则引擎
var-gen --description "获取用户信息" --style snake --force-rule
```

#### 查看最近历史记录
```bash
var-gen --history
```

#### 清除所有历史记录
```bash
var-gen --clear-history
```

#### 查看支持的命名规范
```bash
var-gen --all-styles
```

## 常见问题

### Q: 为什么中文描述会生成中文变量名？
A: 这是因为没有配置有效的API密钥，程序回退到了规则引擎。配置API密钥后，LLM会生成英文变量名。

### Q: 交互式模式在Windows下无法使用？
A: 某些终端可能不支持交互式功能，建议使用PowerShell或Windows Terminal。

### Q: 如何扩展中文词汇映射？
A: 现在支持通过配置文件扩展词汇映射：
   1. 复制 `mapping-config-example.json` 作为模板
   2. 编辑映射关系和停用词
   3. 在 `config.toml` 中设置 `mapping_config_path` 指向您的配置文件
   4. 重新启动程序即可生效

### Q: 配置文件格式错误怎么办？
A: 程序会自动验证配置文件格式，如果检测到错误会提供详细的错误信息。建议：
   - 检查JSON格式是否正确（使用JSON验证工具）
   - 确保所有必填字段都存在
   - 参考 `mapping-config-example.json` 示例文件
   - 查看程序输出的具体错误提示

### Q: 支持哪些操作系统？
A: 支持Windows、macOS、Linux等主流操作系统。

### Q: 如何处理复杂的变量描述？
A: 对于复杂的描述，建议：
   - 使用简洁明了的中文描述
   - 避免过长的句子，推荐使用短语
   - 利用自定义映射配置文件扩展专业词汇
   - 如结果不理想，可尝试调整描述方式或手动修改

## 技术架构

### 双引擎设计
1. **LLM引擎**：基于大语言模型，提供智能语义理解
2. **规则引擎**：基于预定义规则，确保离线可用性

### 自动降级机制
- 网络不可用时自动切换到规则引擎
- API密钥无效时自动切换到规则引擎
- 用户指定`--force-rule`时强制使用规则引擎

### 配置管理系统
- 支持自定义词汇映射配置文件
- 灵活的配置项管理（API密钥、默认命名规范、缓存设置等）
- 配置文件热加载和动态更新
- 用户友好的配置验证和错误提示

## 贡献指南

欢迎提交Issue和Pull Request！

### 开发环境搭建
```bash
git clone https://github.com/yourusername/var-gen.git
cd var-gen
cargo build
```

### 运行测试
```bash
cargo test
```

### 代码质量
- ✅ 所有测试通过，无编译警告
- ✅ 完善的错误处理机制
- ✅ 代码结构清晰，模块化设计
- ✅ 支持自定义配置和扩展

## 更新日志

### v0.2.0 (最新版本)
- ✨ 新增自定义词汇映射配置功能
- ✨ 支持通过配置文件扩展中文到英文翻译规则
- ✨ 优化规则引擎的分词算法
- ✨ 改进配置管理系统，支持更多配置选项
- ✨ 增强错误处理和用户提示
- ✨ 完善测试覆盖，确保代码稳定性

### v0.1.0
- ✨ 新增交互式箭头选择功能
- ✨ 优化中文转英文编码名功能
- ✨ 支持6种命名规范
- ✨ 添加批量处理功能
- ✨ 完善历史记录管理

## 许可证

[MIT](LICENSE)