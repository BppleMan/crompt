# Crompt 设计文档

## 项目目标
自动化生成 shell script 的参数解析框架，类似 Rust 的 clap，但用于生成 shell 脚本。

## 实现边界

### 输入：TOML 配置文件
用户通过 TOML 文件声明命令结构和参数

### 输出：Shell Script 模板
生成带有完整参数解析逻辑的 shell 脚本，用户只需填充具体的业务逻辑

## 全局配置

### 基本信息
```toml
name = "example"              # [必需] 脚本名称
version = "0.0.1"             # [必需] 版本号
authors = ["Name <email>"]    # [可选] 作者信息
help = true                   # [可选，默认 true] 是否生成帮助信息
```

### Shell 配置
```toml
shebang = "#!/usr/bin/env zsh"  # [可选，默认 "#!/bin/bash"] Shebang 行
permission = "0755"              # [可选，默认 "0755"] 文件权限
```

### 库文件配置
```toml
common_lib = true                    # [可选，默认 false] 是否注入常用辅助函数
libs = ["./libs/mylib.zsh"]         # [可选] 额外引入的自定义库文件
```

**common_lib 包含的功能：**
- 日志函数：`log_info()`, `log_warn()`, `log_error()`
- 颜色支持：`COLOR_RED`, `COLOR_GREEN`, `COLOR_YELLOW` 等
- 工具函数：`confirm()`, `spinner()`, `progress_bar()` 等

**libs 用法：**
- 在生成的脚本中会自动 source 指定的库文件
- 支持相对路径和绝对路径
- 生成时会验证文件是否存在（可选）

## 参数设计（Args）

### 参数类型

#### 1. 选项参数（Option Arguments）
带有短名称（-x）或长名称（--xxx）的参数，需要一个值

```toml
[[commands.args]]
name = "name"           # 参数的内部变量名
short = "n"             # 短选项 -n
long = "name"           # 长选项 --name
help = "帮助信息"        # 帮助文本
required = true         # 是否必需
takes_value = true      # 是否需要值（true 表示选项参数）
default = "默认值"       # 可选：默认值
```

**使用示例：**
```bash
script greet --name John
script greet -n John
```

#### 2. 标志参数（Flag Arguments）
布尔型参数，存在即为 true，不需要值

```toml
[[commands.args]]
name = "verbose"
short = "v"
long = "verbose"
help = "详细输出"
required = false
takes_value = false     # false 表示这是一个 flag
```

**使用示例：**
```bash
script greet --verbose
script greet -v
```

#### 3. 位置参数（Positional Arguments）
按照位置顺序的参数，不需要前缀

```toml
[[commands.args]]
name = "key"
help = "配置项的键"
required = true
takes_value = true
is_positional = true    # 标记为位置参数
position = 0            # 位置索引（从 0 开始）
```

**使用示例：**
```bash
script config set mykey myvalue
# mykey 是 position=0
# myvalue 是 position=1
```

### 完整的参数属性

```toml
[[commands.args]]
name = "参数名"              # [必需] 内部变量名，生成的脚本中使用
short = "单字母"             # [可选] 短选项，如 "n" 对应 -n
long = "长名称"              # [可选] 长选项，如 "name" 对应 --name
help = "帮助信息"            # [可选] 参数说明
required = true/false       # [可选，默认 false] 是否必需
takes_value = true/false    # [必需] 是否接受值（false 表示 flag）
default = "默认值"           # [可选] 默认值（仅当 required=false 时有效）
is_positional = true/false  # [可选，默认 false] 是否为位置参数
position = 0                # [可选] 位置索引（仅当 is_positional=true 时需要）
```

### 参数设计原则

1. **互斥性**：
   - 位置参数不能有 `short` 和 `long`
   - Flag 参数（`takes_value=false`）不能有 `default`

2. **优先级**：
   - `is_positional=true` 时，忽略 `short` 和 `long`
   - 位置参数按 `position` 排序

3. **验证规则**：
   - `required=true` 的参数必须提供
   - 位置参数按顺序验证
   - 选项参数可以任意顺序

## 命令结构

### 主命令
```toml
[[commands]]
name = "命令名"
about = "命令说明"
# args 定义该命令的参数
# subcommands 定义该命令的子命令
```

### 子命令
```toml
[[commands.subcommands]]
name = "子命令名"
about = "子命令说明"
# args 定义该子命令的参数
```

## 生成的 Shell Script 结构

### 1. 头部信息
- Shebang
- 版本信息
- 作者信息

### 2. 工具函数
- 错误处理
- 帮助信息显示
- 版本信息显示

### 3. 帮助函数
- 每个命令/子命令都有对应的 `show_xxx_help()` 函数

### 4. 命令实现函数
- 每个命令/子命令对应一个 `cmd_xxx()` 函数
- 包含完整的参数解析逻辑
- 参数验证
- TODO 注释区域供用户填充业务逻辑

### 5. 主函数
- 全局选项处理（--help, --version）
- 命令路由

## 使用场景示例

### 场景 1：带选项的简单命令
```bash
example greet --name John --times 3 --loud
```

### 场景 2：子命令
```bash
example greet morning --name John
```

### 场景 3：位置参数
```bash
example config set database.host localhost
example config get database.host
```

### 场景 4：混合使用
```toml
[[commands.args]]
name = "input"
is_positional = true
position = 0

[[commands.args]]
name = "verbose"
short = "v"
long = "verbose"
takes_value = false
```

使用：
```bash
example process input.txt --verbose
```

## Args 设计总结

你当前的 `example.toml` 设计已经很好了！建议的改进：

### ✅ 已经很好的地方：
- `takes_value` 区分了选项参数和标志参数
- `required` 和 `default` 清晰
- `is_positional` 和 `position` 用于位置参数

### 💡 建议补充：
1. **验证类型**（可选的未来功能）：
   ```toml
   value_type = "string|number|path|email"  # 值类型验证
   ```

2. **多值参数**（可选的未来功能）：
   ```toml
   multiple = true  # 允许多次指定，如 --tag rust --tag cli
   ```

3. **参数依赖**（可选的未来功能）：
   ```toml
   requires = ["other_arg"]  # 需要其他参数同时存在
   conflicts_with = ["other_arg"]  # 与其他参数互斥
   ```

### 当前阶段建议：
保持简洁，先实现核心功能：
- ✅ 选项参数（带值）
- ✅ 标志参数（布尔）
- ✅ 位置参数
- ✅ 必需/可选
- ✅ 默认值

这已经足够覆盖 90% 的使用场景了！

