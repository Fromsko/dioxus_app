# Dioxus 快速开始指南

## ✅ 已完成

你的 Dioxus 项目已经创建并运行成功！

### 当前状态

- ✅ 项目结构已创建
- ✅ 依赖已配置
- ✅ 桌面版本已编译并运行
- ✅ 计数器应用正在运行

### 项目文件

```
dioxus_app/
├── src/
│   └── main.rs          # 主应用代码（计数器）
├── Cargo.toml           # 项目配置
├── index.html           # Web 版本 HTML 模板
├── README.md            # 项目说明
└── GUIDE.md             # 本指南
```

## 🚀 运行方式

### 桌面版本（当前正在运行）

```bash
cargo run --release
```

### Web 版本（需要额外工具）

#### 方法 1: 使用 Trunk（推荐）

```bash
# 安装 Trunk
cargo install trunk

# 运行开发服务器
trunk serve

# 访问 http://127.0.0.1:8080
```

#### 方法 2: 使用 Dioxus CLI

```bash
# 安装 dx（需要 CMake 和完整的构建环境）
cargo install dioxus-cli

# 运行
dx serve --platform web
```

## 📱 Android 版本

### 准备工作

1. 安装 Android SDK 和 NDK
2. 安装 Java JDK 11+
3. 安装 Dioxus CLI

### 运行

```bash
dx serve --platform android
```

## 🎨 代码说明

### 核心概念

**1. 组件函数**

```rust
fn app() -> Element {
    // 组件逻辑
}
```

**2. 状态管理（Signals）**

```rust
let mut count = use_signal(|| 0);  // 创建响应式状态
count += 1;                         // 修改状态会自动更新 UI
```

**3. RSX 语法（类似 JSX）**

```rust
rsx! {
    div {
        h1 { "标题" }
        button {
            onclick: move |_| count += 1,
            "点击"
        }
    }
}
```

## 🔧 下一步

### 1. 添加更多功能

- 表单输入
- 路由导航
- API 调用
- 本地存储

### 2. 样式优化

- 使用 CSS 文件
- Tailwind CSS 集成
- 响应式设计

### 3. 部署

- Web: 构建静态文件部署到任何服务器
- Desktop: 打包成可执行文件
- Mobile: 构建 APK/IPA

## 📚 学习资源

- 官方文档: https://dioxuslabs.com/learn/0.7/
- GitHub: https://github.com/DioxusLabs/dioxus
- 示例: https://github.com/DioxusLabs/dioxus/tree/main/examples

## 💡 提示

- 修改 `src/main.rs` 后重新运行 `cargo run` 查看变化
- 桌面版本支持热重载（使用 dx serve）
- 同一套代码可以编译到多个平台
- Rust 的类型安全让你的应用更可靠

## 🐛 常见问题

**Q: 如何添加新页面？**
A: 使用 `dioxus-router` crate

**Q: 如何调用 API？**
A: 使用 `reqwest` 或 `gloo-net`

**Q: 如何添加样式？**
A: 在 RSX 中使用 `style` 属性或外部 CSS 文件

**Q: 性能如何？**
A: Dioxus 使用虚拟 DOM 和增量渲染，性能接近原生

---

🎉 恭喜！你已经成功创建并运行了第一个 Dioxus 应用！
