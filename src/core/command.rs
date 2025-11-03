use crate::core::arg::Arg;
use serde::{Deserialize, Serialize};

/// 命令定义
///
/// 表示一个命令或子命令的配置信息，支持递归的子命令结构。
/// 每个命令可以有自己的参数列表和子命令列表。
///
/// # 示例
///
/// ```toml
/// [[commands]]
/// name = "build"
/// about = "构建项目"
/// aliases = ["b"]
///
/// [[commands.args]]
/// name = "target"
/// long = "target"
/// help = "构建目标"
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Command {
    /// 命令名称
    ///
    /// 用于识别和调用该命令的唯一标识符。
    pub name: String,

    /// 命令说明（可选）
    ///
    /// 在帮助文档中显示的命令描述信息。
    pub about: Option<String>,

    /// 命令别名列表
    ///
    /// 可用于调用此命令的替代名称，方便用户快速输入。
    #[serde(default)]
    pub aliases: Vec<String>,

    /// 是否隐藏该命令
    ///
    /// 如果为 `true`，该命令不会在帮助信息中显示，但仍可使用。
    /// 通常用于内部命令或实验性功能。
    #[serde(default)]
    pub hidden: bool,

    /// 是否已弃用
    ///
    /// 如果为 `true`，该命令被标记为已弃用，使用时会显示警告信息。
    #[serde(default)]
    pub deprecated: bool,

    /// 命令的参数列表
    ///
    /// 该命令接受的所有参数定义。
    #[serde(default)]
    pub args: Vec<Arg>,

    /// 子命令列表（递归结构）
    ///
    /// 该命令的子命令定义。支持多层嵌套，形成命令树结构。
    #[serde(default)]
    pub subcommands: Vec<Command>,
}
