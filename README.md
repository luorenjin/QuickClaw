# QuickClaw 🦀

**QuickClaw** 是 [OpenClaw](https://github.com/luorenjin/OpenClaw) 的 Rust 全平台可用的一站式桌面客户端，是养虾人的 Work Studio。一键引导式安装，快速实现 Claw 身份性格定义，安装完成即可用。

## 特性

- 🚀 **一键引导式安装** — 五步向导式配置，几分钟内完成所有设置
- 🎭 **Claw 身份定义** — 为您的 AI 助手定义独特的名字和角色定位
- ✨ **性格定制** — 灵活配置 Claw 的性格特征和行为风格（系统提示词）
- 💬 **即开即用** — 配置完成后立即开始与 Claw 对话
- 🌐 **全平台支持** — Windows、macOS、Linux 均可运行

## 快速开始

### 前置要求

- [Rust](https://rustup.rs/) 1.75+
- Linux 额外需要：`libgtk-3-dev`、`libssl-dev`（用于 OpenGL 渲染）

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
2. **第一步：服务器配置** — 输入 OpenClaw 服务器地址和（可选）API 密钥
3. **第二步：Claw 身份定义** — 设置 Claw 的名字和角色定位
4. **第三步：Claw 性格定义** — 选择性格特征，编写系统提示词
5. **完成** — 开始与您的专属 Claw 进行对话

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
