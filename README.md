# Dioxus 应用

这是一个跨平台的 Dioxus 应用，支持 Web、桌面和移动端。

## 运行方式

### Web 版本

```bash
dx serve --platform web
# 或者使用 trunk
trunk serve
```

### 桌面版本

```bash
cargo run
```

### Android 版本

```bash
dx serve --platform android
```

## 项目结构

- `src/main.rs` - 主应用代码
- `index.html` - Web 版本的 HTML 模板
- `Cargo.toml` - 项目配置

## 功能

- ✅ 计数器功能
- ✅ 响应式 UI
- ✅ 跨平台支持（Web/Desktop/Mobile）
- ✅ 热重载（使用 dx serve）

## 下一步

1. 安装 Dioxus CLI: `cargo install dioxus-cli`
2. 或者安装 Trunk: `cargo install trunk`
3. 运行开发服务器
4. 开始开发你的应用！
