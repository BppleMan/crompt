use toml;

fn main() {
    let toml_str = r#"
# 方式1: 数组的表格（Array of Tables）
[[commands]]
name = "test1"

[[commands.args]]
name = "arg1"
short = "a"

# 方式2: 内联表数组（Array of Inline Tables）
[[commands2]]
name = "test2"
args = [
    {name = "arg1", short = "a", long = "arg1", help = "描述1", required = true},
    {name = "arg2", short = "b", long = "arg2", help = "描述2", required = false}
]
"#;

    match toml::from_str::<toml::Value>(toml_str) {
        Ok(value) => {
            println!("✅ TOML 解析成功！\n");
            println!("{:#?}", value);
        }
        Err(e) => {
            println!("❌ TOML 解析失败: {}", e);
        }
    }
}

