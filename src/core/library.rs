use serde::{Deserialize, Serialize};

/// 库文件定义
///
/// 用于描述需要引入的外部库文件，支持多种来源类型。
/// 库文件会被嵌入到生成的 shell 脚本中。
///
/// # 示例
///
/// ```toml
/// [[libs]]
/// type = "crompt"
/// path = "std/log"
///
/// [[libs]]
/// type = "file"
/// path = "/path/to/mylib.sh"
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Library {
    /// 库类型
    ///
    /// 指定库文件的来源类型，决定如何解析和加载库文件。
    #[serde(rename = "type")]
    pub lib_type: LibType,

    /// 库路径
    ///
    /// 根据 `lib_type` 的不同，可以是：
    /// - `crompt`: Crompt 标准库的路径（如 "std/log"）
    /// - `file`: 本地文件系统的绝对或相对路径
    /// - `url`: 远程文件的 HTTP/HTTPS URL
    pub path: String,
}

/// 库类型枚举
///
/// 定义库文件的来源类型。
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LibType {
    /// Crompt 标准库
    ///
    /// 从 Crompt 的标准库目录加载库文件。
    #[default]
    Crompt,

    /// 本地文件
    ///
    /// 从本地文件系统加载库文件。
    File,

    /// 远程 URL
    ///
    /// 通过 HTTP/HTTPS 从远程服务器下载库文件。
    Url,
}
