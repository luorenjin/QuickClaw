# QuickClaw 🦀

**QuickClaw** 是 [OpenClaw](https://github.com/luorenjin/OpenClaw) 的 Rust 全平台可用的一站式桌面客户端，是养虾人的 Work Studio。一键引导式安装，快速实现 Claw 身份性格定义，安装完成即可用。

## 特性

- 🔍 **环境自动检测** — 自动检测系统中的 Node.js、Claude Code，缺失时一键自动安装
- 📦 **OpenClaw 便捷安装** — 通过 npm 一键安装 OpenClaw，自动安装依赖、启动本地服务
- 🚀 **引导式安装向导** — 七步向导（欢迎 → 环境检查 → 安装OpenClaw → 服务器配置 → 身份定义 → 性格定义 → 完成），几分钟内完成所有设置
- 🎭 **Claw 身份定义** — 为您的 AI 助手定义独特的名字和角色定位
- ✨ **性格定制** — 灵活配置 Claw 的性格特征和行为风格（系统提示词）
- 💬 **即开即用** — 配置完成后立即开始与 Claw 对话
- 🌐 **全平台支持** — Windows、macOS、Linux 均可运行

## 快速开始

### 前置要求（QuickClaw 本身的构建依赖）

- [Rust](https://rustup.rs/) 1.75+
- Linux 额外需要：`libgtk-3-dev`、`libssl-dev`（用于 OpenGL 渲染）

> **注意**：Node.js、Claude Code 等 OpenClaw 运行依赖会由 QuickClaw 自动检测并引导安装，无需手动配置。Git 仅作为可选的开发工具，OpenClaw 可通过 npm 直接安装。

### 构建与运行

```bash
# 克隆仓库
git clone https://github.com/luorenjin/QuickClaw.git
cd QuickClaw

# 开发模式运行
cargo run

# 发布版本构建
cargo build --release
# 可执行文件位于 target/release/quickclaw
```

### 使用流程

1. **首次启动** — 自动进入配置向导
2. **第一步：系统环境检查** — 自动检测 Node.js、Claude Code（AI 能力插件）；缺失时支持一键自动安装
3. **第二步：安装 OpenClaw** — 选择安装目录，通过 npm 自动安装 OpenClaw 及其依赖
4. **第三步：服务器配置** — 确认 OpenClaw 服务器地址（自动安装后会自动填写 `localhost:8080`）
5. **第四步：Claw 身份定义** — 设置 Claw 的名字和角色定位
6. **第五步：Claw 性格定义** — 选择性格特征，编写系统提示词
7. **完成** — 开始与您的专属 Claw 进行对话

## 系统依赖说明

| 组件 | 必需 | 用途 | 自动安装方式 |
|------|------|------|------------|
| Node.js | ✅ | OpenClaw 运行时（需 v18+）| Linux: apt / macOS: brew / Windows: winget |
| Claude Code | ⭕ 可选 | AI 能力插件（@anthropic-ai/claude-code）| npm install -g |
| Git | ⭕ 可选 | 开发工具（仅用于 QuickClaw 开发）| Linux: apt / macOS: brew / Windows: winget |

## 配置文件

配置文件自动保存在系统标准配置目录：
- **Linux**: `~/.config/QuickClaw/config.json`
- **macOS**: `~/Library/Application Support/com.QuickClaw.QuickClaw/config.json`
- **Windows**: `%APPDATA%\QuickClaw\QuickClaw\config\config.json`

## 开发

```bash
# 运行单元测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 许可证

MIT License — 详见 [LICENSE](LICENSE) 文件
