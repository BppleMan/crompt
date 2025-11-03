use std::fs;
use serde::{Deserialize, Serialize};

// 模拟 Arg 结构
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    #[serde(default)]
    pub short: Option<String>,
    #[serde(default)]
    pub long: Option<String>,
    #[serde(default)]
    pub help: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub takes_value: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub validator: Option<String>,
    #[serde(default)]
    pub allowed: Vec<String>,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub position: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct SubCommand {
    name: String,
    about: Option<String>,
    #[serde(default)]
    args: Vec<Arg>,
}

#[derive(Debug, Deserialize)]
struct Command {
    name: String,
    about: Option<String>,
    #[serde(default)]
    args: Vec<Arg>,
    #[serde(default)]
    subcommands: Vec<SubCommand>,
}

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: String,
    authors: Vec<String>,
    shebang: Option<String>,
    permission: Option<String>,
    #[serde(default)]
    commands: Vec<Command>,
}

fn main() {
    let toml_path = "example/example.toml";

    match fs::read_to_string(toml_path) {
        Ok(content) => {
            println!("📄 读取文件: {}", toml_path);

            match toml::from_str::<Config>(&content) {
                Ok(config) => {
                    println!("✅ TOML 解析成功并正确反序列化到结构体！\n");

                    println!("📦 项目: {} v{}", config.name, config.version);
                    println!("👥 作者: {:?}", config.authors);
                    println!("🔧 Shebang: {}", config.shebang.unwrap_or_default());
                    println!("🔐 权限: {}\n", config.permission.unwrap_or_default());

                    println!("📋 命令详情:");
                    for cmd in &config.commands {
                        println!("\n🎯 命令: {}", cmd.name);
                        if let Some(about) = &cmd.about {
                            println!("   说明: {}", about);
                        }

                        if !cmd.args.is_empty() {
                            println!("   参数:");
                            for arg in &cmd.args {
                                print!("     - {}", arg.name);
                                if let Some(short) = &arg.short {
                                    print!(" (-{})", short);
                                }
                                if let Some(long) = &arg.long {
                                    print!(" (--{})", long);
                                }
                                if arg.required {
                                    print!(" [必需]");
                                }
                                if let Some(default) = &arg.default {
                                    print!(" [默认: {}]", default);
                                }
                                if let Some(pos) = arg.position {
                                    print!(" [位置: {}]", pos);
                                }
                                println!();
                            }
                        }

                        if !cmd.subcommands.is_empty() {
                            println!("   子命令:");
                            for subcmd in &cmd.subcommands {
                                println!("     • {}", subcmd.name);
                                if !subcmd.args.is_empty() {
                                    for arg in &subcmd.args {
                                        print!("       - {}", arg.name);
                                        if let Some(pos) = arg.position {
                                            print!(" [位置: {}]", pos);
                                        }
                                        println!();
                                    }
                                }
                            }
                        }
                    }

                    println!("\n✅ 所有字段都成功映射到 Arg 结构体！");
                }
                Err(e) => {
                    println!("❌ TOML 反序列化失败: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("❌ 读取文件失败: {}", e);
            std::process::exit(1);
        }
    }
}

