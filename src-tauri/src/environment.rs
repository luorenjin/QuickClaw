use std::fmt;
use std::process::Command;
use std::sync::{Arc, Mutex};

// ─── Dependency status ────────────────────────────────────────────────────────

/// 依赖组件的安装状态
#[derive(Debug, Clone, PartialEq)]
pub enum DepStatus {
    /// 尚未检测
    Unknown,
    /// 已安装，包含版本号
    Installed(String),
    /// 未安装
    Missing,
    /// 正在安装
    Installing,
    /// 安装失败，包含错误信息
    Failed(String),
}

impl DepStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, DepStatus::Installed(_))
    }
    pub fn is_busy(&self) -> bool {
        matches!(self, DepStatus::Installing)
    }
}

impl fmt::Display for DepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DepStatus::Unknown => write!(f, "未检测"),
            DepStatus::Installed(v) => write!(f, "已安装 {}", v),
            DepStatus::Missing => write!(f, "未安装"),
            DepStatus::Installing => write!(f, "安装中..."),
            DepStatus::Failed(e) => write!(f, "安装失败: {}", e),
        }
    }
}

// ─── Dependency descriptor ────────────────────────────────────────────────────

/// 依赖组件描述
#[derive(Debug, Clone)]
pub struct Dependency {
    /// 组件名称（用于显示）
    pub name: &'static str,
    /// 组件说明
    pub description: &'static str,
    /// 是否必需（false = 可选插件）
    pub required: bool,
    /// 当前状态
    pub status: DepStatus,
}

/// 所有 QuickClaw 依赖的规范列表
pub fn all_dependencies() -> Vec<Dependency> {
    vec![
        Dependency {
            name: "Node.js",
            description: "OpenClaw 运行时，需要 v18 或更高版本",
            required: true,
            status: DepStatus::Unknown,
        },
        Dependency {
            name: "Git",
            description: "用于下载和更新 OpenClaw",
            required: true,
            status: DepStatus::Unknown,
        },
        Dependency {
            name: "Claude Code",
            description: "AI 能力插件（@anthropic-ai/claude-code），提供 Claw 智能核心",
            required: false,
            status: DepStatus::Unknown,
        },
    ]
}

// ─── Detection ────────────────────────────────────────────────────────────────

/// 检测单个依赖是否已安装，返回版本号字符串或 None
pub fn detect(dep_name: &str) -> Option<String> {
    match dep_name {
        "Node.js" => run_version_cmd("node", &["--version"]),
        "Git" => run_version_cmd("git", &["--version"]),
        "Claude Code" => detect_claude_code(),
        _ => None,
    }
}

fn run_version_cmd(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn detect_claude_code() -> Option<String> {
    // 先尝试 `claude --version`
    if let Some(v) = run_version_cmd("claude", &["--version"]) {
        return Some(v);
    }
    // 再通过 npm 全局包检查
    if let Some(out) = run_version_cmd("npm", &["list", "-g", "--depth=0", "@anthropic-ai/claude-code"]) {
        if out.contains("@anthropic-ai/claude-code") {
            let version = out
                .lines()
                .find(|l| l.contains("@anthropic-ai/claude-code"))
                .and_then(|l| l.split('@').last())
                .unwrap_or("installed")
                .trim()
                .to_string();
            return Some(version);
        }
    }
    None
}

/// 检测所有依赖，更新状态列表
pub fn detect_all(deps: &mut Vec<Dependency>) {
    for dep in deps.iter_mut() {
        dep.status = match detect(dep.name) {
            Some(v) => DepStatus::Installed(v),
            None => DepStatus::Missing,
        };
    }
}

// ─── Installation ─────────────────────────────────────────────────────────────

/// 安装进度（由安装线程写入，UI 线程读取）
#[derive(Debug, Clone, Default)]
pub struct InstallProgress {
    /// 当前正在执行的步骤
    pub current_step: String,
    /// 执行日志（每行一条）
    pub log: Vec<String>,
    /// 是否已完成（成功或失败）
    pub finished: bool,
    /// 最终错误（None = 成功）
    pub error: Option<String>,
}

impl InstallProgress {
    pub fn push_log(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }
    pub fn succeed(&mut self) {
        self.finished = true;
        self.error = None;
    }
    pub fn fail(&mut self, err: impl Into<String>) {
        self.finished = true;
        self.error = Some(err.into());
    }
}

pub type SharedProgress = Arc<Mutex<InstallProgress>>;

/// 在后台线程中安装单个依赖，通过共享进度对象报告状态
pub fn install_dependency_async(dep_name: &'static str, progress: SharedProgress) {
    std::thread::spawn(move || {
        {
            let mut p = progress.lock().unwrap();
            p.current_step = format!("正在安装 {}...", dep_name);
            p.push_log(format!("► 开始安装 {}", dep_name));
        }

        let result = install_dependency(dep_name, &progress);

        let mut p = progress.lock().unwrap();
        match result {
            Ok(()) => {
                p.push_log(format!("✓ {} 安装成功", dep_name));
                p.succeed();
            }
            Err(e) => {
                p.push_log(format!("✗ {} 安装失败: {}", dep_name, e));
                p.fail(e);
            }
        }
    });
}

/// 同步安装单个依赖
fn install_dependency(dep_name: &str, progress: &SharedProgress) -> Result<(), String> {
    match dep_name {
        "Node.js" => install_nodejs(progress),
        "Git" => install_git(progress),
        "Claude Code" => install_claude_code(progress),
        other => Err(format!("未知依赖: {}", other)),
    }
}

fn log(progress: &SharedProgress, msg: impl Into<String>) {
    if let Ok(mut p) = progress.lock() {
        p.push_log(msg.into());
    }
}

// ── Node.js ──

fn install_nodejs(progress: &SharedProgress) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        log(progress, "使用 apt 安装 Node.js...");
        run_cmd_logged(progress, "sudo", &["apt-get", "update", "-q"])?;
        run_cmd_logged(progress, "sudo", &["apt-get", "install", "-y", "nodejs", "npm"])?;
    }
    #[cfg(target_os = "macos")]
    {
        // 优先使用 brew；若未安装则下载官方 pkg
        if which("brew") {
            log(progress, "使用 Homebrew 安装 Node.js...");
            run_cmd_logged(progress, "brew", &["install", "node"])?;
        } else {
            return Err(
                "请先安装 Homebrew（https://brew.sh）或从 https://nodejs.org 手动安装 Node.js".into(),
            );
        }
    }
    #[cfg(target_os = "windows")]
    {
        log(progress, "使用 winget 安装 Node.js...");
        run_cmd_logged(progress, "winget", &["install", "-e", "--id", "OpenJS.NodeJS", "--silent"])?;
    }
    verify_install("Node.js", "node", &["--version"], progress)
}

// ── Git ──

fn install_git(progress: &SharedProgress) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        log(progress, "使用 apt 安装 Git...");
        run_cmd_logged(progress, "sudo", &["apt-get", "update", "-q"])?;
        run_cmd_logged(progress, "sudo", &["apt-get", "install", "-y", "git"])?;
    }
    #[cfg(target_os = "macos")]
    {
        if which("brew") {
            log(progress, "使用 Homebrew 安装 Git...");
            run_cmd_logged(progress, "brew", &["install", "git"])?;
        } else {
            return Err(
                "请先安装 Homebrew（https://brew.sh）或从 https://git-scm.com 手动安装 Git".into(),
            );
        }
    }
    #[cfg(target_os = "windows")]
    {
        log(progress, "使用 winget 安装 Git...");
        run_cmd_logged(progress, "winget", &["install", "-e", "--id", "Git.Git", "--silent"])?;
    }
    verify_install("Git", "git", &["--version"], progress)
}

// ── Claude Code ──

fn install_claude_code(progress: &SharedProgress) -> Result<(), String> {
    log(progress, "通过 npm 全局安装 Claude Code...");
    // Claude Code 作为 npm 全局包安装
    run_cmd_logged(progress, "npm", &["install", "-g", "@anthropic-ai/claude-code"])?;
    verify_install("Claude Code", "claude", &["--version"], progress)
}

// ── OpenClaw ──────────────────────────────────────────────────────────────────

/// OpenClaw 安装配置
pub struct OpenClawInstallConfig {
    /// 安装目标目录（父目录，openclaw 子目录将在其中创建）
    pub install_dir: String,
    /// OpenClaw Git 仓库地址
    pub repo_url: String,
}

impl Default for OpenClawInstallConfig {
    fn default() -> Self {
        let home = home_dir();
        Self {
            install_dir: format!("{}/.quickclaw", home),
            // NOTE: users should verify the repo URL before proceeding.
            // Future improvement: support GPG-signed tags or checksum validation
            // to guarantee integrity of downloaded code before execution.
            repo_url: "https://github.com/luorenjin/OpenClaw.git".into(),
        }
    }
}

/// 检测 OpenClaw 是否已安装（检查目录 + package.json 是否存在）
pub fn detect_openclaw(install_dir: &str) -> bool {
    let openclaw_dir = std::path::Path::new(install_dir).join("openclaw");
    openclaw_dir.join("package.json").exists()
}

/// 在后台线程中安装 OpenClaw
pub fn install_openclaw_async(config: OpenClawInstallConfig, progress: SharedProgress) {
    std::thread::spawn(move || {
        {
            let mut p = progress.lock().unwrap();
            p.current_step = "正在安装 OpenClaw...".into();
        }
        let result = install_openclaw_sync(&config, &progress);
        let mut p = progress.lock().unwrap();
        match result {
            Ok(()) => {
                p.push_log("✓ OpenClaw 安装完成！".to_string());
                p.succeed();
            }
            Err(e) => {
                p.push_log(format!("✗ 安装失败: {}", e));
                p.fail(e);
            }
        }
    });
}

fn install_openclaw_sync(config: &OpenClawInstallConfig, progress: &SharedProgress) -> Result<(), String> {
    let install_path = std::path::Path::new(&config.install_dir);
    let openclaw_dir = install_path.join("openclaw");

    // 1. 创建安装目录
    log(progress, format!("创建目录: {}", install_path.display()));
    std::fs::create_dir_all(install_path)
        .map_err(|e| format!("创建安装目录失败: {}", e))?;

    // 2. 克隆或更新仓库
    if openclaw_dir.exists() {
        log(progress, "OpenClaw 目录已存在，正在更新...");
        run_cmd_in_logged(progress, "git", &["pull", "--rebase"], &openclaw_dir)?;
    } else {
        log(progress, format!("克隆 OpenClaw 仓库: {}", config.repo_url));
        run_cmd_in_logged(
            progress,
            "git",
            &["clone", "--depth=1", &config.repo_url, "openclaw"],
            install_path,
        )?;
    }

    // 3. 安装 npm 依赖
    log(progress, "安装 npm 依赖...");
    run_cmd_in_logged(progress, "npm", &["install", "--prefer-offline"], &openclaw_dir)?;

    // 4. 尝试首次构建（如果有 build 脚本）
    let pkg_json = openclaw_dir.join("package.json");
    if let Ok(content) = std::fs::read_to_string(&pkg_json) {
        if content.contains("\"build\"") {
            log(progress, "构建 OpenClaw...");
            run_cmd_in_logged(progress, "npm", &["run", "build"], &openclaw_dir)?;
        }
    }

    log(progress, format!("OpenClaw 已安装至: {}", openclaw_dir.display()));
    Ok(())
}

/// 启动 OpenClaw 服务（后台运行）并返回监听端口
pub fn start_openclaw(install_dir: &str) -> Result<u16, String> {
    let openclaw_dir = std::path::Path::new(install_dir).join("openclaw");
    if !openclaw_dir.exists() {
        return Err("OpenClaw 未安装，请先运行安装向导".to_string());
    }

    let port = 8080u16;

    // 启动 OpenClaw（假设入口为 `npm start` 或 `node .`）
    // 在 Unix 上通过 setsid 命令脱离父进程，避免随 QuickClaw 关闭
    #[cfg(unix)]
    let mut cmd = {
        let mut c = Command::new("setsid");
        c.arg("npm")
            .arg("start")
            .current_dir(&openclaw_dir)
            .env("PORT", port.to_string())
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        c
    };
    #[cfg(not(unix))]
    let mut cmd = {
        let mut c = Command::new("npm");
        c.arg("start")
            .current_dir(&openclaw_dir)
            .env("PORT", port.to_string())
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        c
    };

    cmd.spawn()
        .map_err(|e| format!("启动 OpenClaw 失败: {}", e))?;

    Ok(port)
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// 运行命令并将 stdout/stderr 写入进度日志；返回 Err 包含输出
fn run_cmd_logged(progress: &SharedProgress, program: &str, args: &[&str]) -> Result<(), String> {
    run_cmd_in_logged(progress, program, args, &std::env::current_dir().unwrap_or_default())
}

fn run_cmd_in_logged(
    progress: &SharedProgress,
    program: &str,
    args: &[&str],
    dir: &std::path::Path,
) -> Result<(), String> {
    log(progress, format!("$ {} {}", program, args.join(" ")));

    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| format!("无法执行 `{}`：{}", program, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if let Ok(mut p) = progress.lock() {
        for line in stdout.lines().filter(|l| !l.trim().is_empty()) {
            p.push_log(format!("  {}", line));
        }
        for line in stderr.lines().filter(|l| !l.trim().is_empty()) {
            p.push_log(format!("  {}", line));
        }
    }

    if !output.status.success() {
        return Err(format!(
            "`{} {}` 退出码 {}",
            program,
            args.join(" "),
            output.status.code().unwrap_or(-1)
        ));
    }
    Ok(())
}

/// 安装后验证是否可以调用
fn verify_install(
    dep_name: &str,
    program: &str,
    args: &[&str],
    progress: &SharedProgress,
) -> Result<(), String> {
    match run_version_cmd(program, args) {
        Some(v) => {
            log(progress, format!("✓ {} 版本: {}", dep_name, v));
            Ok(())
        }
        None => Err(format!("{} 安装后仍无法找到可执行文件，请重启终端后重试", dep_name)),
    }
}

/// 检查程序是否在 PATH 中
fn which(program: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 获取用户主目录（跨平台）
fn home_dir() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dep_status_is_ok() {
        assert!(DepStatus::Installed("v18.0.0".into()).is_ok());
        assert!(!DepStatus::Missing.is_ok());
        assert!(!DepStatus::Unknown.is_ok());
        assert!(!DepStatus::Installing.is_ok());
        assert!(!DepStatus::Failed("err".into()).is_ok());
    }

    #[test]
    fn test_dep_status_is_busy() {
        assert!(DepStatus::Installing.is_busy());
        assert!(!DepStatus::Missing.is_busy());
        assert!(!DepStatus::Installed("v1".into()).is_busy());
    }

    #[test]
    fn test_dep_status_display() {
        assert_eq!(DepStatus::Unknown.to_string(), "未检测");
        assert_eq!(DepStatus::Missing.to_string(), "未安装");
        assert_eq!(DepStatus::Installing.to_string(), "安装中...");
        assert_eq!(
            DepStatus::Installed("v20.0.0".into()).to_string(),
            "已安装 v20.0.0"
        );
        assert!(DepStatus::Failed("boom".into()).to_string().contains("boom"));
    }

    #[test]
    fn test_all_dependencies_has_required_entries() {
        let deps = all_dependencies();
        assert!(deps.iter().any(|d| d.name == "Node.js" && d.required));
        assert!(deps.iter().any(|d| d.name == "Git" && d.required));
        assert!(deps.iter().any(|d| d.name == "Claude Code" && !d.required));
    }

    #[test]
    fn test_install_progress_succeed() {
        let mut p = InstallProgress::default();
        p.push_log("step 1");
        p.succeed();
        assert!(p.finished);
        assert!(p.error.is_none());
        assert_eq!(p.log, vec!["step 1"]);
    }

    #[test]
    fn test_install_progress_fail() {
        let mut p = InstallProgress::default();
        p.fail("something went wrong");
        assert!(p.finished);
        assert_eq!(p.error.as_deref(), Some("something went wrong"));
    }

    #[test]
    fn test_openclaw_default_install_config() {
        let cfg = OpenClawInstallConfig::default();
        assert!(cfg.install_dir.contains(".quickclaw"));
        assert!(cfg.repo_url.contains("OpenClaw"));
    }

    #[test]
    fn test_detect_openclaw_missing() {
        // 不存在的目录应返回 false
        assert!(!detect_openclaw("/tmp/nonexistent_quickclaw_test_dir_xyz"));
    }

    #[test]
    fn test_shared_progress_thread_safety() {
        let progress = Arc::new(Mutex::new(InstallProgress::default()));
        let p2 = Arc::clone(&progress);
        let handle = std::thread::spawn(move || {
            p2.lock().unwrap().push_log("from thread");
        });
        handle.join().unwrap();
        assert_eq!(progress.lock().unwrap().log, vec!["from thread"]);
    }
}
