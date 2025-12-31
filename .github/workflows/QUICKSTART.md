# 🚀 Android 构建快速参考

## 一键构建

```bash
# 推送代码 → 自动构建
git push

# 发布版本 → 自动创建 Release
git tag v1.0.0 && git push --tags
```

## 下载 APK

- **Actions 页面** → 点击运行 → 下载 Artifacts
- **Releases 页面** → 选择版本 → 下载 APK

## 选择哪个 APK？

- 📱 **现代手机**（2019+）→ `arm64-v8a.apk`
- 📱 **旧手机** → `armeabi-v7a.apk`
- 🤷 **不确定** → 先试 `arm64-v8a.apk`

## 安装步骤

1. 下载 APK 到手机
2. 设置 → 安全 → 允许未知来源
3. 点击 APK 安装

## 常见问题

| 问题     | 解决方案                            |
| -------- | ----------------------------------- |
| 无法安装 | 允许未知来源应用                    |
| 签名冲突 | 卸载旧版本重装                      |
| 构建失败 | 查看 Actions 日志                   |
| 构建太慢 | 首次需要 15-30 分钟，后续 5-10 分钟 |

## 自定义

编辑 `.github/workflows/android-simple.yml`：

```yaml
# 修改包名
package = "com.yourcompany.app"

# 修改应用名
label = "Your App"

# 修改版本
versionCode 1
versionName "1.0"
```

---

就这么简单！🎉
