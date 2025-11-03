use serde::{Deserialize, Serialize};

/// 命令行参数定义
///
/// 用于描述命令行参数的配置信息，包括参数名称、标志、帮助信息、验证规则等。
/// 支持短标志（如 `-v`）、长标志（如 `--verbose`）以及位置参数。
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Arg {
    /// 参数名称
    ///
    /// 用于在生成的脚本中引用该参数的标识符。
    pub name: String,

    /// 短标志（可选）
    ///
    /// 单字符标志，如 `-v`。不包含前导的 `-` 符号。
    pub short: Option<String>,

    /// 长标志（可选）
    ///
    /// 长格式标志，如 `--verbose`。不包含前导的 `--` 符号。
    pub long: Option<String>,

    /// 帮助信息（可选）
    ///
    /// 在帮助文档中显示的参数说明文本。
    pub help: Option<String>,

    /// 是否为必需参数
    ///
    /// 如果为 `true`，则在未提供该参数时会报错。
    pub required: bool,

    /// 是否接受值
    ///
    /// 如果为 `true`，则该参数后需要跟随一个值（如 `--name value`）。
    /// 如果为 `false`，则该参数为布尔开关（如 `--verbose`）。
    pub takes_value: bool,

    /// 默认值（可选）
    ///
    /// 当参数未提供时使用的默认值。仅在 `takes_value` 为 `true` 时有效。
    pub default: Option<String>,

    /// 验证器（可选）
    ///
    /// 用于验证参数值的 shell 表达式或函数名。
    pub validator: Option<String>,

    /// 允许的值列表
    ///
    /// 限制参数只能取列表中的值。空列表表示不限制。
    pub allowed: Vec<String>,

    /// 是否允许多次出现
    ///
    /// 如果为 `true`，则该参数可以多次提供，值会被收集到数组中。
    pub multiple: bool,

    /// 位置参数索引（可选）
    ///
    /// 如果设置，则该参数为位置参数，索引表示其在命令行中的位置（从 0 开始）。
    /// 位置参数不需要指定标志名。
    pub position: Option<u32>,
}
