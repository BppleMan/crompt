# Crompt 设计文档 仍在开发中

## 项目目标

自动化生成 shell script 的参数解析框架，类似 Rust 的 clap，但用于生成 shell 脚本。

## 实现边界

### 输入：TOML 配置文件

用户通过 TOML 文件声明命令结构和参数

### 输出：Shell Script 模板

生成带有完整参数解析逻辑的 shell 脚本，用户只需填充具体的业务逻辑
