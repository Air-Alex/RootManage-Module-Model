# RMM Workflows

## 工作流说明

### release.yml - 跨平台发布工作流

自动构建和发布多平台二进制文件的工作流。

**触发条件：**
- 推送标签（格式：`v*`，例如 `v0.3.6`）
- 手动触发（workflow_dispatch）

**支持平台：**
- Linux x86_64 (`rmm-linux-x86_64`)
- Linux ARM64 (`rmm-linux-aarch64`)
- macOS x86_64 (`rmm-macos-x86_64`)
- macOS ARM64/Apple Silicon (`rmm-macos-aarch64`)
- Windows x86_64 (`rmm-windows-x86_64.exe`)

**使用方法：**

1. 创建新标签并推送：
   ```bash
   git tag v0.3.6
   git push origin v0.3.6
   ```

2. 或在 GitHub Actions 页面手动触发工作流

**发布流程：**
1. 自动为每个平台构建优化的二进制文件
2. 对二进制文件进行 strip（Linux/macOS）以减小体积
3. 上传 artifacts 供下载（保留 7 天）
4. 如果是标签触发，自动创建 GitHub Release 并上传所有平台的二进制文件

### website.yml / website-new.yml - 网站构建工作流

用于构建和部署项目网站到 GitHub Pages。

---

## 高级构建模式

旧版RMM 被我扔垃圾桶了，现在支持更加高级的操作。
