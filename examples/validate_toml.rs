use std::fs;
use toml;

fn main() {
    let toml_path = "example/example.toml";

    match fs::read_to_string(toml_path) {
        Ok(content) => {
            println!("📄 读取文件: {}", toml_path);

            match toml::from_str::<toml::Value>(&content) {
                Ok(value) => {
                    println!("✅ TOML 解析成功！\n");

                    // 打印关键信息
                    if let Some(table) = value.as_table() {
                        if let Some(name) = table.get("name") {
                            println!("📦 项目名称: {}", name);
                        }
                        if let Some(version) = table.get("version") {
                            println!("📌 版本: {}", version);
                        }
                        if let Some(commands) = table.get("commands") {
                            if let Some(cmds) = commands.as_array() {
                                println!("🎯 命令数量: {}", cmds.len());
                                for (i, cmd) in cmds.iter().enumerate() {
                                    if let Some(cmd_table) = cmd.as_table() {
                                        if let Some(cmd_name) = cmd_table.get("name") {
                                            println!("  {}. {}", i + 1, cmd_name);

                                            // 打印 args
                                            if let Some(args) = cmd_table.get("args") {
                                                if let Some(args_array) = args.as_array() {
                                                    println!("     参数数量: {}", args_array.len());
                                                }
                                            }

                                            // 打印 subcommands
                                            if let Some(subcmds) = cmd_table.get("subcommands") {
                                                if let Some(subcmds_array) = subcmds.as_array() {
                                                    println!("     子命令数量: {}", subcmds_array.len());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    println!("\n📋 完整结构:");
                    println!("{:#?}", value);
                }
                Err(e) => {
                    println!("❌ TOML 解析失败: {}", e);
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

