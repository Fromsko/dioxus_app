# GitHub Actions 自动构建 Android APK

开箱即用，无需任何配置！推送代码即可自动构建已签名的 Android APK。

## � 快作速开始

### 1. 推送代码自动构建

```bash
git add .
git commit -m "Update app"
git push origin main
```

构建完成后，在 Actions 页面下载 APK。

### 2. 发布版本

```bash
git tag v1.0.0
git push origin v1.0.0
```

自动创建 GitHub Release 并上传 APK。

### 3. 手动触发

1. 进入 GitHub 仓库
2. 点击 "Actions" 标签
3. 选择 "Android Build" 工作流
4. 点击 "Run workflow"

## � 构建产物

每次构建生成两个已签名的 APK：

- **dioxus_app-arm64-v8a.apk** - 64 位 ARM（现代设备，推荐）
- **dioxus_app-armeabi-v7a.apk** - 32 位 ARM（旧设备）

## ✅ 自动签名

工作流自动生成 debug 签名并签名 APK，**无需任何配置**！

生成的 APK 可以直接安装到任何 Android 设备。

> 💡 使用 debug 签名，适合开发和个人使用。不适合发布到应用商店。

## 📦 下载 APK

### 从 Actions 下载（每次构建）

1. 进入 Actions 页面
2. 点击最新的工作流运行
3. 在 Artifacts 区域下载 `android-apks`
4. 解压 ZIP 文件获取 APK

### 从 Releases 下载（标签触发）

1. 进入仓库的 Releases 页面
2. 选择版本
3. 直接下载 APK 文件

## 📱 安装 APK

1. 将 APK 传输到 Android 设备
2. 在设置中允许安装未知来源应用：
   - 设置 → 安全 → 未知来源
   - 或：设置 → 应用 → 特殊访问权限 → 安装未知应用
3. 点击 APK 文件安装

## 🛠️ 自定义配置

### 修改应用包名

编辑工作流文件中的 `package` 字段：

```yaml
package = "com.yourcompany.yourapp"
```

### 修改应用名称

编辑工作流文件中的 `label` 字段：

```yaml
label = "Your App Name"
```

### 修改应用图标

在项目中添加图标文件：

```
res/
  mipmap-hdpi/ic_launcher.png
  mipmap-mdpi/ic_launcher.png
  mipmap-xhdpi/ic_launcher.png
  mipmap-xxhdpi/ic_launcher.png
  mipmap-xxxhdpi/ic_launcher.png
```

## 🐛 常见问题

### Q: APK 无法安装？

A: 确保：

- 手机允许安装未知来源应用
- 选择了正确的架构（现代手机用 ARM64）
- 如果之前安装过，先卸载旧版本

### Q: 如何知道手机是什么架构？

A: 大多数 2019 年后的手机都是 ARM64。如果不确定，先试 ARM64 版本。

### Q: 构建失败怎么办？

A: 查看 Actions 页面的构建日志，通常是依赖问题。可以重新运行工作流。

### Q: 构建需要多久？

A: 首次构建约 15-30 分钟（下载依赖），后续构建约 5-10 分钟（使用缓存）。

### Q: 可以构建其他架构吗？

A: 可以！在工作流中添加 `x86_64-linux-android` 或 `i686-linux-android` 目标。

### Q: 签名安全吗？

A: 使用的是 debug 签名，适合开发和个人使用。每次构建使用相同的签名，可以直接覆盖安装。

## 📋 工作流做了什么

1. ✅ 安装 Java 17
2. ✅ 安装 Android SDK 和 NDK
3. ✅ 安装 Rust 工具链和 Android 目标
4. ✅ 配置 Android 元数据
5. ✅ 构建 ARM64 和 ARMv7 APK
6. ✅ 自动生成 debug 签名
7. ✅ 签名并对齐 APK
8. ✅ 上传到 Artifacts 或 Release

## 🔧 本地构建

如果想在本地构建：

```bash
# 安装工具
cargo install cargo-apk

# 添加 Android 目标
rustup target add aarch64-linux-android armv7-linux-androideabi

# 配置 Cargo.toml（参考工作流中的配置）

# 构建
cargo apk build --release --target aarch64-linux-android
```

## 📚 相关资源

- [Dioxus 官方文档](https://dioxuslabs.com)
- [cargo-apk](https://github.com/rust-mobile/cargo-apk)
- [Android 开发者文档](https://developer.android.com)

## 💡 提示

- 推荐使用标签发布正式版本（v1.0.0, v1.1.0 等）
- 每次推送都会触发构建，可以在工作流中调整触发条件
- APK 文件名包含架构信息，方便识别
- 构建缓存可以显著加快后续构建速度

---

🎉 现在你可以自动构建 Android APK 了！推送代码，等待几分钟，下载安装即可。
