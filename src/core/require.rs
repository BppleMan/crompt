use serde::{Deserialize, Serialize};

/// 依赖项定义
///
/// 用于声明脚本运行所需的外部依赖，如系统命令或环境变量。
/// 脚本会在执行前检查这些依赖是否满足，不满足时会报错并退出。
///
/// # 示例
///
/// ```toml
/// [[requires]]
/// type = "command"
/// name = "git"
///
/// [[requires]]
/// type = "env"
/// name = "HOME"
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Require {
    /// 依赖类型
    ///
    /// 指定依赖项的类型，可以是系统命令或环境变量。
    #[serde(rename = "type")]
    pub require_type: RequireType,

    /// 依赖的名称
    ///
    /// 根据 `require_type` 的不同，可以是：
    /// - `command`: 系统命令的名称（如 "git"、"curl"）
    /// - `env`: 环境变量的名称（如 "HOME"、"PATH"）
    pub name: String,
}

/// 依赖类型枚举
///
/// 定义脚本可能依赖的外部资源类型。
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequireType {
    /// 系统命令
    ///
    /// 要求系统中存在指定的可执行命令，会使用 `command -v` 检查。
    #[default]
    Command,

    /// 环境变量
    ///
    /// 要求存在指定的环境变量，会检查该变量是否已设置且非空。
    Env,
}
