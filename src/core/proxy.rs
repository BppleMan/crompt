use serde::{Deserialize, Serialize};

/// 代理配置
///
/// 用于配置网络请求的代理设置，支持 HTTP、HTTPS 等协议。
/// 这些配置会作为环境变量应用到生成的脚本中。
///
/// # 示例
///
/// ```toml
/// [proxy]
/// http_proxy = "http://proxy.example.com:8080"
/// https_proxy = "http://proxy.example.com:8080"
/// no_proxy = "localhost,127.0.0.1"
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Proxy {
    /// HTTP 代理地址（可选）
    ///
    /// 用于 HTTP 协议请求的代理服务器地址，格式为 "http://host:port"。
    pub http_proxy: Option<String>,

    /// HTTPS 代理地址（可选）
    ///
    /// 用于 HTTPS 协议请求的代理服务器地址，格式为 "http://host:port"。
    pub https_proxy: Option<String>,

    /// 通用代理地址（可选）
    ///
    /// 用于所有协议的默认代理服务器地址，优先级低于协议专用代理。
    pub all_proxy: Option<String>,

    /// 不使用代理的地址列表（可选）
    ///
    /// 以逗号分隔的主机名或 IP 地址列表，这些地址的请求不经过代理。
    /// 例如："localhost,127.0.0.1,.example.com"
    pub no_proxy: Option<String>,
}
