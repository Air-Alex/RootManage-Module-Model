# RMM Workflows

## 工作流说明

### release.yml - 跨平台发布工作流

自动构建和发布多平台二进制文件的工作流。该工作流已完善，支持自动发布构建产物。

**触发条件：**
- 推送标签（格式：`v*`，例如 `v0.3.6`）- 创建正式版本发布
- 手动触发（workflow_dispatch）- 创建开发构建发布

**手动触发参数：**
- `release_name`: 自定义发布名称（可选，留空则自动生成）
- `prerelease`: 标记为预发布版本（默认为 true）

**支持平台：**
- Linux x86_64 (`rmm-linux-x86_64`)
- Linux ARM64 (`rmm-linux-aarch64`)
- macOS x86_64 (`rmm-macos-x86_64`)
- macOS ARM64/Apple Silicon (`rmm-macos-aarch64`)
- Windows x86_64 (`rmm-windows-x86_64.exe`)
- Android ARM64 (`rmm-android-aarch64`)

**使用方法：**

1. **创建正式版本** - 创建新标签并推送：
   ```bash
   git tag v0.3.6
   git push origin v0.3.6
   ```
   这将创建一个名为 "Release v0.3.6" 的正式发布。

2. **创建开发构建** - 在 GitHub Actions 页面手动触发工作流：
   - 进入 Actions → Release Cross-Platform Binaries → Run workflow
   - 可选择填写发布名称和是否标记为预发布
   - 默认会创建带时间戳的开发构建（如 "Development Build 20251108-193000"）

**发布流程：**
1. **构建阶段** (build job):
   - 并行为所有平台构建优化的二进制文件
   - 对二进制文件进行 strip（Linux/macOS）以减小体积
   - 统一重命名为友好的文件名（如 `rmm-linux-x86_64`）
   - 上传 artifacts 供下载（保留 7 天）

2. **发布阶段** (release job):
   - 下载所有平台的构建产物
   - 自动生成发布说明（包含构建信息、平台列表、安装说明）
   - 创建 GitHub Release 并上传所有平台的二进制文件
   - 根据触发方式自动设置发布类型和标签

**发布类型：**
- **标签触发**: 创建正式发布，标记为 latest
- **手动触发**: 创建开发构建，默认标记为 prerelease，使用格式 `build-YYYYMMDD-HHMMSS-SHORT_SHA` 的标签

### website.yml / website-new.yml - 网站构建工作流

用于构建和部署项目网站到 GitHub Pages。

---

## 高级构建模式

旧版RMM 被我扔垃圾桶了，现在支持更加高级的操作。
