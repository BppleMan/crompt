use crate::core::{command::Command, require::Require};
use serde::{Deserialize, Serialize};

/// 顶层配置结构体
///
/// 对应 TOML 配置文件的根结构，包含脚本的所有元数据和命令定义。
/// 这是 Crompt 配置的入口点，用于描述整个 shell 脚本的结构。
///
/// # 示例
///
/// ```toml
/// name = "mytool"
/// version = "1.0.0"
/// description = "我的命令行工具"
/// shebang = "#!/usr/bin/env bash"
/// permission = "755"
/// help = true
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// 脚本名称
    ///
    /// 生成的 shell 脚本的名称，也用于帮助信息的显示。
    pub name: String,

    /// 项目描述（可选）
    ///
    /// 对脚本功能的简短说明，会显示在帮助信息的顶部。
    pub description: Option<String>,

    /// 项目主页（可选）
    ///
    /// 项目的 URL 地址，可以是代码仓库或文档网站。
    pub homepage: Option<String>,

    /// 许可证（可选）
    ///
    /// 项目的开源许可证类型，如 "MIT"、"Apache-2.0" 等。
    pub license: Option<String>,

    /// 是否生成帮助信息
    ///
    /// 如果为 `true`，会自动生成 `-h`/`--help` 选项的处理逻辑。
    pub help: bool,

    /// 版本号
    ///
    /// 脚本的版本号，通常遵循语义化版本规范（如 "1.0.0"）。
    pub version: String,

    /// 作者信息列表
    ///
    /// 脚本作者的名称或邮箱列表。
    #[serde(default)]
    pub authors: Vec<String>,

    /// Shebang 行
    ///
    /// 脚本文件的第一行，指定解释器路径（如 "#!/usr/bin/env bash"）。
    pub shebang: String,

    /// 文件权限
    ///
    /// 生成的脚本文件的权限模式，采用八进制表示（如 "755"）。
    pub permission: String,

    /// 引入的库文件列表
    ///
    /// 需要在脚本中导入的外部库文件路径。
    #[serde(default)]
    pub libs: Vec<String>,

    /// 运行所需的外部依赖
    ///
    /// 脚本运行前需要检查的命令或环境变量。
    #[serde(default)]
    pub requires: Vec<Require>,

    /// 命令列表
    ///
    /// 脚本支持的所有顶层命令定义。
    #[serde(default)]
    pub commands: Vec<Command>,
}
