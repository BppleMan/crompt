# TOML 参数格式对比

## ✅ 答案：两种格式都可以使用！

你的 `example.toml` 现在使用的是**内联表数组格式**，这是完全合法的 TOML 语法。

---

## 格式对比

### 方式 1: 数组的表格（Array of Tables）

**特点：** 结构清晰，每个参数单独一个表格

```toml
[[commands]]
name = "greet"

[[commands.args]]
name = "name"
short = "n"
long = "name"
help = "要打招呼的人的名字"
required = true
takes_value = true

[[commands.args]]
name = "times"
short = "t"
long = "times"
help = "重复打招呼的次数"
required = false
takes_value = true
default = "1"
```

**优点：**
- ✅ 结构清晰，易读
- ✅ 适合参数属性很多的情况
- ✅ 便于添加注释

**缺点：**
- ❌ 比较冗长
- ❌ 参数多时占用空间大

---

### 方式 2: 内联表数组（Array of Inline Tables）⭐ 推荐

**特点：** 紧凑简洁，一行定义一个参数

```toml
[[commands]]
name = "greet"
args = [
    {name = "name", short = "n", long = "name", help = "要打招呼的人的名字", required = true, takes_value = true},
    {name = "times", short = "t", long = "times", help = "重复打招呼的次数", required = false, takes_value = true, default = "1"},
    {name = "loud", short = "l", long = "loud", help = "使用大写字母", required = false, takes_value = false}
]
```

**优点：**
- ✅ 紧凑简洁
- ✅ 一目了然，所有参数集中显示
- ✅ 适合参数定义标准化的场景（像我们的项目）
- ✅ 减少文件长度

**缺点：**
- ❌ 单行过长时可读性下降
- ❌ 不便于添加详细注释

---

## 💡 最佳实践建议

### 对于 Crompt 项目，推荐使用 **内联表数组格式**，原因：

1. **参数属性固定**：我们的参数结构是标准化的（name, short, long, help, required, takes_value 等）
2. **减少冗余**：避免重复的 `[[commands.args]]` 声明
3. **提高可读性**：所有参数集中在一起，便于对比和修改
4. **符合习惯**：类似 Rust Cargo.toml 的 dependencies 格式

### 格式化建议

```toml
# ✅ 好的格式 - 每个参数一行
args = [
    {name = "name", short = "n", long = "name", help = "说明", required = true, takes_value = true},
    {name = "verbose", short = "v", long = "verbose", help = "详细输出", required = false, takes_value = false}
]

# ✅ 也可以接受 - 简单参数可以更紧凑
args = [
    {name = "name", short = "n", help = "名字", required = true},
    {name = "age", short = "a", help = "年龄", required = false, default = "18"}
]

# ❌ 避免 - 太长的行
args = [{name = "name", short = "n", long = "name", help = "要打招呼的人的名字，这个说明非常长，会导致行太长", required = true, takes_value = true, default = "John", validation = "^[a-zA-Z]+$"}]
```

---

## 🎯 当前项目使用的格式

你的 `example/example.toml` 现在使用的是 **内联表数组格式**：

```toml
[[commands]]
name = "greet"
about = "向用户打招呼"
args = [
    {name = "name", short = "n", long = "name", help = "要打招呼的人的名字", required = true, takes_value = true},
    {name = "times", short = "t", long = "times", help = "重复打招呼的次数", required = false, takes_value = true, default = "1"},
    {name = "loud", short = "l", long = "loud", help = "使用大写字母", required = false, takes_value = false}
]
```

这个格式：
- ✅ 已验证可以正确解析
- ✅ 简洁明了
- ✅ 适合我们的项目需求
- ✅ 符合 TOML 标准

---

## 📚 参考

- TOML 规范：https://toml.io/en/v1.0.0#array-of-tables
- 内联表格：https://toml.io/en/v1.0.0#inline-table
- 类似项目：Cargo.toml 的 dependencies 使用内联格式

