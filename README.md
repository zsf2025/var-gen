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
git clone https://github.com/zsf2025/var-gen.git
cd var-gen

# 编译安装
cargo build --release

# 添加到PATH（可选）
cargo install --path .
```

#### 从预编译二进制文件安装
> ⚠️ 注意：目前项目尚未发布预编译二进制文件，敬请期待后续版本。
> 
> 您可以：
> 1. 使用上面的**从源码构建**方式安装
> 2. 关注项目的 [GitHub Releases](https://github.com/zsf2025/var-gen/releases) 页面获取后续更新
> 3. 自行编译生成二进制文件（见下方说明）

##### 自行编译二进制文件
如果您希望获得独立的二进制文件，可以执行以下命令：

```bash
# 克隆项目
git clone https://github.com/zsf2025/var-gen.git
cd var-gen

# 编译发布版本（会生成优化后的二进制文件）
cargo build --release

# 二进制文件位置
# Windows: target\release\var-gen.exe
# Linux/macOS: target/release/var-gen

# 您可以将二进制文件复制到系统PATH中的任意目录
cp target/release/var-gen ~/.local/bin/  # Linux/macOS
# 或
copy target\release\var-gen.exe C:\Windows\System32\  # Windows（需要管理员权限）
```

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

## 详细使用方法

### 交互式模式

交互式模式提供直观的箭头选择界面：

```bash
# 启动交互式模式
var-gen

# 或者强制使用交互式模式
var-gen --interactive
```

操作流程：
- 输入变量描述（如：获取用户信息）
- 使用↑↓箭头键选择命名规范
- 查看生成的变量名结果
- 使用←→箭头键或y/n选择是否保存历史记录

### 高级功能

```bash
# 查看历史记录
var-gen --history

# 清除历史记录
var-gen --clear-history

# 查看支持的命名规范
var-gen --all-styles

# 强制使用规则引擎（跳过LLM）
var-gen --description "获取用户信息" --style snake --force-rule
```

## 配置管理

### 配置文件位置
配置文件位于用户目录下的`.var-gen`文件夹：

```
~/.var-gen/
├── config.toml  # 主配置文件
├── db/          # 数据库目录
└── custom-mapping.json  # 自定义映射文件（可选）
```

### 基础配置
编辑 `~/.var-gen/config.toml` 文件：

```toml
# API配置（可选）
api_key = "your-api-key"
model = "qwen-tiny"  # 或 "xinghuo-lite"

# 默认设置
default_style = "snake"
cache_enabled = true
cache_ttl = 86400

# 自定义映射文件路径（可选）
mapping_config_path = "/path/to/your/custom-mapping.json"
```

### 自定义词汇映射

1. **创建映射文件**：
   ```bash
   cp mapping-config-example.json ~/.var-gen/custom-mapping.json
   ```

2. **编辑映射规则**：
   - 添加或修改中文到英文的词汇映射
   - 定义停用词列表
   - 支持版本管理和描述信息

3. **配置路径**：在 `config.toml` 中设置 `mapping_config_path`

4. **测试配置**：
   ```bash
   var-gen --description "您的测试描述" --style snake
   ```

**配置优先级**：配置文件中的映射优先于内置映射

## 支持的命名规范

- **camel (驼峰命名法)**：如 `userName`
- **pascal (帕斯卡命名法)**：如 `UserName`
- **snake (下划线命名法)**：如 `user_name`
- **kebab (短横线命名法)**：如 `user-name`
- **upper_snake (大写下划线命名法)**：如 `USER_NAME`
- **lower_camel (小驼峰命名法)**：如 `userName`（与camel相同）

## 工作原理

### 双引擎智能处理

var-gen采用双引擎架构，智能选择最佳处理方式：

1. **大模型引擎**（在线模式）
   - 利用大语言模型深度理解语义
   - 生成更智能、符合编程习惯的变量名
   - 需要有效API密钥和网络连接

2. **规则引擎**（离线模式）
   - 基于预定义规则的本地处理
   - 支持中文分词和词汇映射
   - 无需网络，响应更快，稳定可靠

### 智能降级机制

- **网络异常** → 自动切换到规则引擎
- **API密钥无效** → 自动切换到规则引擎  
- **用户指定** → `--force-rule` 强制使用规则引擎

### 中文处理流程

规则引擎处理中文描述的步骤：

1. **智能分词**：使用Jieba分词器进行中文分词
2. **词汇映射**：将中文词汇翻译成英文（支持自定义映射）
3. **命名转换**：根据指定规范生成最终变量名

**示例**：
- "获取用户信息" → [`获取`, `用户`, `信息`] → [`get`, `user`, `info`] → snake_case → `get_user_info`
- "创建数据库连接" → [`创建`, `数据库`, `连接`] → [`create`, `database`, `connection`] → camelCase → `createDatabaseConnection`

## 使用示例

### 基础示例

```bash
# 中文转英文（规则引擎）
var-gen --description "获取用户信息" --style snake
# 输出：get_user_info

var-gen --description "创建数据库连接" --style camel  
# 输出：createDatabaseConnection

# 英文描述（大模型模式）
var-gen --description "user login time" --style pascal
# 输出：UserLoginTime
```

### 批量处理

```bash
# 从文件批量处理
var-gen --file ./vars.txt --style snake --output ./results.txt

# 查看所有支持的命名规范
var-gen --all-styles
```

## 高级功能

### 交互式模式

提供直观的箭头选择界面：

```bash
# 启动交互式模式
var-gen
# 或
var-gen --interactive
```

**操作流程**：
1. 输入变量描述
2. 使用↑↓选择命名规范  
3. 查看生成结果
4. 选择是否保存历史记录

### 中文词汇映射

规则引擎内置丰富的中文词汇映射：

**开发相关**：获取→get、设置→set、更新→update、删除→delete、创建→create

**用户相关**：用户→user、信息→info、密码→password、账户→account

**数据相关**：数据→data、数据库→database、文件→file、记录→record

### 配置管理

```bash
# 设置API密钥
var-gen --set-api-key "your-api-key"

# 清除API密钥  
var-gen --clear-api-key

# 查看历史记录
var-gen --history

# 强制使用规则引擎
var-gen --description "获取用户信息" --style snake --force-rule
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
git clone https://github.com/zsf2025/var-gen.git
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