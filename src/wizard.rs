use crate::config::ClawConfig;
use crate::environment::{
    self, all_dependencies, Dependency, DepStatus, InstallProgress, OpenClawInstallConfig,
    SharedProgress,
};
use egui::RichText;
use std::sync::{Arc, Mutex};

// ─── Wizard step enum ─────────────────────────────────────────────────────────

/// 安装向导步骤
#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    /// 欢迎页面
    Welcome,
    /// 系统环境检查与安装
    EnvCheck,
    /// 安装 OpenClaw 本地服务
    InstallOpenClaw,
    /// 服务器连接配置
    ServerConfig,
    /// Claw 身份定义
    ClawIdentity,
    /// Claw 性格定义
    ClawPersonality,
    /// 完成
    Finish,
}

impl WizardStep {
    /// 在进度指示器中的下标（Welcome/Finish 不展示在进度条上）
    pub fn progress_index(&self) -> usize {
        match self {
            WizardStep::Welcome => 0,
            WizardStep::EnvCheck => 1,
            WizardStep::InstallOpenClaw => 2,
            WizardStep::ServerConfig => 3,
            WizardStep::ClawIdentity => 4,
            WizardStep::ClawPersonality => 5,
            WizardStep::Finish => 6,
        }
    }

    pub fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Welcome => Some(WizardStep::EnvCheck),
            WizardStep::EnvCheck => Some(WizardStep::InstallOpenClaw),
            WizardStep::InstallOpenClaw => Some(WizardStep::ServerConfig),
            WizardStep::ServerConfig => Some(WizardStep::ClawIdentity),
            WizardStep::ClawIdentity => Some(WizardStep::ClawPersonality),
            WizardStep::ClawPersonality => Some(WizardStep::Finish),
            WizardStep::Finish => None,
        }
    }

    pub fn prev(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Welcome => None,
            WizardStep::EnvCheck => Some(WizardStep::Welcome),
            WizardStep::InstallOpenClaw => Some(WizardStep::EnvCheck),
            WizardStep::ServerConfig => Some(WizardStep::InstallOpenClaw),
            WizardStep::ClawIdentity => Some(WizardStep::ServerConfig),
            WizardStep::ClawPersonality => Some(WizardStep::ClawIdentity),
            WizardStep::Finish => Some(WizardStep::ClawPersonality),
        }
    }
}

// ─── Wizard state ─────────────────────────────────────────────────────────────

/// 向导整体状态
pub struct WizardState {
    pub step: WizardStep,

    // ── 环境检查 ──
    /// 依赖组件列表（带检测状态）
    pub deps: Vec<Dependency>,
    /// 是否已触发过检测
    pub env_checked: bool,
    /// 各组件的独立安装进度（与 deps 等长）
    pub dep_progress: Vec<SharedProgress>,

    // ── OpenClaw 安装 ──
    pub openclaw_config: OpenClawInstallConfig,
    pub openclaw_progress: SharedProgress,
    /// OpenClaw 是否已就绪（已安装或本次安装完成）
    pub openclaw_ready: bool,

    // ── 性格定义 ──
    pub traits_input: String,

    // ── 服务器配置 ──
    pub connection_status: Option<Result<String, String>>,

    // ── 完成页 ──
    pub save_result: Option<Result<(), String>>,
}

impl WizardState {
    pub fn new(config: &ClawConfig) -> Self {
        let deps = all_dependencies();
        let n = deps.len();
        let openclaw_config = OpenClawInstallConfig::default();
        let openclaw_ready = environment::detect_openclaw(&openclaw_config.install_dir);
        Self {
            step: if config.configured {
                WizardStep::Finish
            } else {
                WizardStep::Welcome
            },
            deps,
            env_checked: false,
            dep_progress: (0..n)
                .map(|_| Arc::new(Mutex::new(InstallProgress::default())))
                .collect(),
            openclaw_config,
            openclaw_progress: Arc::new(Mutex::new(InstallProgress::default())),
            openclaw_ready,
            traits_input: config.traits_as_string(),
            connection_status: None,
            save_result: None,
        }
    }

    /// 所有必需依赖是否已就绪
    pub fn required_deps_ok(&self) -> bool {
        self.deps
            .iter()
            .filter(|d| d.required)
            .all(|d| d.status.is_ok())
    }

    /// 是否有任何安装任务正在运行
    pub fn any_dep_installing(&self) -> bool {
        self.deps.iter().any(|d| d.status.is_busy())
    }
}

// ─── Top-level render ─────────────────────────────────────────────────────────

/// 渲染向导界面，返回 true 表示向导完成（可切换到主界面）
pub fn render(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) -> bool {
    let mut finished = false;

    // 进度指示器（仅在中间步骤显示）
    let show_progress = !matches!(
        state.step,
        WizardStep::Welcome | WizardStep::Finish
    );
    if show_progress {
        render_progress(ui, &state.step);
        ui.add_space(16.0);
    }

    match state.step.clone() {
        WizardStep::Welcome => render_welcome(ui, state),
        WizardStep::EnvCheck => render_env_check(ui, state),
        WizardStep::InstallOpenClaw => render_install_openclaw(ui, state, config),
        WizardStep::ServerConfig => render_server_config(ui, state, config),
        WizardStep::ClawIdentity => render_identity(ui, state, config),
        WizardStep::ClawPersonality => render_personality(ui, state, config),
        WizardStep::Finish => {
            finished = render_finish(ui, state, config);
        }
    }

    finished
}

// ─── Progress bar ─────────────────────────────────────────────────────────────

fn render_progress(ui: &mut egui::Ui, step: &WizardStep) {
    // The Welcome step is not shown in the progress indicator; we display
    // the 5 post-welcome steps: EnvCheck (1), InstallOpenClaw (2),
    // ServerConfig (3), ClawIdentity (4), ClawPersonality (5).
    let labels = ["环境检查", "安装 OpenClaw", "服务器配置", "身份定义", "性格定义"];
    let current = step.progress_index(); // 1-5 for non-Welcome/Finish steps

    ui.horizontal(|ui| {
        for (i, label) in labels.iter().enumerate() {
            let step_num = i + 1; // 1-indexed
            let is_current = step_num == current;
            let is_done = step_num < current;

            if i > 0 {
                ui.label(
                    RichText::new("──")
                        .color(if is_done || is_current {
                            egui::Color32::from_rgb(0, 120, 215)
                        } else {
                            egui::Color32::DARK_GRAY
                        })
                        .size(11.0),
                );
            }

            let (text_color, bg_color) = if is_current {
                (egui::Color32::WHITE, egui::Color32::from_rgb(0, 120, 215))
            } else if is_done {
                (egui::Color32::WHITE, egui::Color32::from_rgb(0, 160, 80))
            } else {
                (egui::Color32::GRAY, egui::Color32::from_rgb(40, 40, 50))
            };

            egui::Frame::new()
                .fill(bg_color)
                .corner_radius(10)
                .inner_margin(egui::Margin::symmetric(7, 3))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(if is_done {
                            format!("✓ {}", label)
                        } else {
                            label.to_string()
                        })
                        .color(text_color)
                        .size(11.0),
                    );
                });
        }
    });
}

// ─── Step: Welcome ────────────────────────────────────────────────────────────

fn render_welcome(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("🦀").size(64.0));
        ui.add_space(16.0);
        ui.label(RichText::new("欢迎使用 QuickClaw").size(28.0).strong());
        ui.add_space(8.0);
        ui.label(
            RichText::new("OpenClaw 的 Rust 全平台一站式桌面客户端")
                .size(16.0)
                .color(egui::Color32::GRAY),
        );
        ui.add_space(32.0);

        let features = [
            ("🔍", "环境自动检测", "自动检测并安装 Node.js、Git、Claude Code"),
            ("📦", "OpenClaw 一键安装", "自动下载并启动本地 OpenClaw 服务"),
            ("🎭", "Claw 身份定义", "为您的 AI 助手赋予独特的身份"),
            ("✨", "性格定制", "定义 Claw 的性格特征和行为风格"),
            ("💬", "即开即用", "配置完成后立即开始对话"),
        ];

        for (icon, title, desc) in &features {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(30, 30, 40))
                .corner_radius(8)
                .inner_margin(egui::Margin::symmetric(16, 10))
                .show(ui, |ui| {
                    ui.set_min_width(420.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(*icon).size(22.0));
                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(*title).size(14.0).strong());
                            ui.label(
                                RichText::new(*desc)
                                    .size(12.0)
                                    .color(egui::Color32::GRAY),
                            );
                        });
                    });
                });
            ui.add_space(6.0);
        }

        ui.add_space(24.0);
        if ui
            .add(
                egui::Button::new(RichText::new("开始配置 →").size(16.0))
                    .min_size(egui::vec2(200.0, 44.0)),
            )
            .clicked()
        {
            state.step = WizardStep::EnvCheck;
        }
    });
}

// ─── Step: Environment Check ──────────────────────────────────────────────────

fn render_env_check(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("第一步：系统环境检查");
    ui.add_space(6.0);
    ui.label(
        RichText::new(
            "QuickClaw 需要以下组件才能运行。点击「检查环境」扫描您的系统，\
             缺失的组件可以由 QuickClaw 自动安装。",
        )
        .size(13.0)
        .color(egui::Color32::GRAY),
    );
    ui.add_space(14.0);

    // ── 依赖列表 ──
    for (idx, dep) in state.deps.iter().enumerate() {
        let prog = state.dep_progress[idx].lock().unwrap().clone();
        render_dep_row(ui, dep, &prog);
    }

    ui.add_space(12.0);

    // ── 操作按钮行 ──
    ui.horizontal(|ui| {
        let checking = state.any_dep_installing();

        if ui
            .add_enabled(!checking, egui::Button::new("🔍  检查环境"))
            .clicked()
        {
            environment::detect_all(&mut state.deps);
            state.env_checked = true;
        }

        if state.env_checked {
            let missing_required: Vec<usize> = state
                .deps
                .iter()
                .enumerate()
                .filter(|(_, d)| d.required && !d.status.is_ok() && !d.status.is_busy())
                .map(|(i, _)| i)
                .collect();

            let missing_optional: Vec<usize> = state
                .deps
                .iter()
                .enumerate()
                .filter(|(_, d)| !d.required && !d.status.is_ok() && !d.status.is_busy())
                .map(|(i, _)| i)
                .collect();

            if !missing_required.is_empty() {
                if ui
                    .add_enabled(
                        !checking,
                        egui::Button::new(
                            RichText::new(format!(
                                "🚀  自动安装缺失必需组件 ({})",
                                missing_required.len()
                            ))
                            .color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(200, 80, 30)),
                    )
                    .clicked()
                {
                    start_install_deps(&mut state.deps, &state.dep_progress, &missing_required);
                }
            }

            if !missing_optional.is_empty() {
                if ui
                    .add_enabled(
                        !checking,
                        egui::Button::new(format!(
                            "📦  安装可选插件 ({})",
                            missing_optional.len()
                        )),
                    )
                    .clicked()
                {
                    start_install_deps(&mut state.deps, &state.dep_progress, &missing_optional);
                }
            }
        }
    });

    // 若有安装任务正在运行，轮询状态并刷新 UI
    if state.any_dep_installing() {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
        // poll finished installs to update dep status
        let indices: Vec<usize> = state
            .deps
            .iter()
            .enumerate()
            .filter(|(_, d)| d.status.is_busy())
            .map(|(i, _)| i)
            .collect();
        for idx in indices {
            let finished = {
                let p = state.dep_progress[idx].lock().unwrap();
                if p.finished {
                    Some(p.error.clone())
                } else {
                    None
                }
            };
            if let Some(maybe_err) = finished {
                state.deps[idx].status = match maybe_err {
                    None => match environment::detect(state.deps[idx].name) {
                        Some(v) => DepStatus::Installed(v),
                        None => DepStatus::Failed("安装后未找到可执行文件".into()),
                    },
                    Some(e) => DepStatus::Failed(e),
                };
            }
        }
    } else if state.env_checked {
        // Low-frequency poll while idle (user may have installed tools manually)
        ui.ctx().request_repaint_after(std::time::Duration::from_secs(5));
    }

    ui.add_space(14.0);

    let can_proceed = !state.any_dep_installing()
        && (!state.env_checked || state.required_deps_ok());
    render_nav_buttons(ui, state, can_proceed, None);

    if state.env_checked && !state.required_deps_ok() && !state.any_dep_installing() {
        ui.add_space(6.0);
        ui.label(
            RichText::new(
                "⚠ 请先安装所有必需组件，或点击「跳过检查」继续（不推荐）。",
            )
            .size(12.0)
            .color(egui::Color32::from_rgb(255, 180, 60)),
        );
        if ui.small_button("跳过检查 →").clicked() {
            state.step = WizardStep::InstallOpenClaw;
        }
    }
}

fn render_dep_row(ui: &mut egui::Ui, dep: &Dependency, prog: &InstallProgress) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(25, 28, 38))
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_min_width(500.0);
            ui.horizontal(|ui| {
                let (icon, icon_color) = match &dep.status {
                    DepStatus::Unknown => ("◌", egui::Color32::GRAY),
                    DepStatus::Installed(_) => ("✓", egui::Color32::from_rgb(0, 200, 100)),
                    DepStatus::Missing => ("✗", egui::Color32::from_rgb(220, 80, 60)),
                    DepStatus::Installing => ("⟳", egui::Color32::from_rgb(100, 180, 255)),
                    DepStatus::Failed(_) => ("!", egui::Color32::from_rgb(255, 120, 50)),
                };
                ui.label(RichText::new(icon).color(icon_color).size(18.0));
                ui.add_space(6.0);

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(dep.name).strong().size(14.0));
                        if !dep.required {
                            ui.label(
                                RichText::new("可选插件")
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(100, 150, 200)),
                            );
                        }
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(
                                    RichText::new(dep.status.to_string())
                                        .size(12.0)
                                        .color(match &dep.status {
                                            DepStatus::Installed(_) => {
                                                egui::Color32::from_rgb(0, 200, 100)
                                            }
                                            DepStatus::Missing | DepStatus::Failed(_) => {
                                                egui::Color32::from_rgb(220, 80, 60)
                                            }
                                            DepStatus::Installing => {
                                                egui::Color32::from_rgb(100, 180, 255)
                                            }
                                            _ => egui::Color32::GRAY,
                                        }),
                                );
                            },
                        );
                    });
                    ui.label(
                        RichText::new(dep.description)
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                    );
                    // Show last few log lines while installing or on failure
                    if dep.status.is_busy() || matches!(&dep.status, DepStatus::Failed(_)) {
                        let last_lines: Vec<&str> = prog
                            .log
                            .iter()
                            .rev()
                            .take(3)
                            .rev()
                            .map(|s| s.as_str())
                            .collect();
                        if !last_lines.is_empty() {
                            ui.label(
                                RichText::new(last_lines.join("\n"))
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(150, 150, 160))
                                    .monospace(),
                            );
                        }
                        if dep.status.is_busy() {
                            ui.spinner();
                        }
                    }
                });
            });
        });
    ui.add_space(4.0);
}

fn start_install_deps(
    deps: &mut Vec<Dependency>,
    dep_progress: &[SharedProgress],
    indices: &[usize],
) {
    for &idx in indices {
        if idx >= deps.len() {
            continue;
        }
        let dep_name = deps[idx].name;
        *dep_progress[idx].lock().unwrap() = InstallProgress::default();
        deps[idx].status = DepStatus::Installing;
        environment::install_dependency_async(dep_name, Arc::clone(&dep_progress[idx]));
    }
}

// ─── Step: Install OpenClaw ───────────────────────────────────────────────────

fn render_install_openclaw(
    ui: &mut egui::Ui,
    state: &mut WizardState,
    config: &mut ClawConfig,
) {
    ui.heading("第二步：安装 OpenClaw 本地服务");
    ui.add_space(6.0);
    ui.label(
        RichText::new(
            "OpenClaw 是驱动 Claw 智能能力的本地服务内核。\
             QuickClaw 将自动将其下载并安装到您的计算机。",
        )
        .size(13.0)
        .color(egui::Color32::GRAY),
    );
    ui.add_space(14.0);

    let prog = state.openclaw_progress.lock().unwrap().clone();
    let is_running = !prog.finished && !prog.log.is_empty();

    if state.openclaw_ready {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 50, 30))
            .corner_radius(8)
            .inner_margin(egui::Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("✓").color(egui::Color32::GREEN).size(20.0));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("OpenClaw 已安装").strong());
                        ui.label(
                            RichText::new(format!(
                                "安装目录: {}",
                                state.openclaw_config.install_dir
                            ))
                            .size(12.0)
                            .color(egui::Color32::GRAY),
                        );
                    });
                });
            });
        ui.add_space(10.0);
        if ui.button("重新安装 / 更新").clicked() {
            state.openclaw_ready = false;
            *state.openclaw_progress.lock().unwrap() = InstallProgress::default();
        }
    } else if is_running {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(
                RichText::new(&prog.current_step)
                    .size(13.0)
                    .color(egui::Color32::from_rgb(100, 180, 255)),
            );
        });
        ui.add_space(6.0);
        render_install_log(ui, &prog.log);
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
    } else if prog.finished {
        if let Some(err) = &prog.error {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(60, 20, 20))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("✗ 安装失败: {}", err))
                            .color(egui::Color32::from_rgb(255, 120, 100))
                            .size(13.0),
                    );
                });
            ui.add_space(6.0);
            render_install_log(ui, &prog.log);
            ui.add_space(8.0);
            if ui.button("重试").clicked() {
                *state.openclaw_progress.lock().unwrap() = InstallProgress::default();
            }
        } else {
            state.openclaw_ready = true;
            config.server_url = "http://localhost:8080".into();
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(20, 50, 30))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("✓ OpenClaw 安装成功！")
                            .color(egui::Color32::GREEN)
                            .size(14.0),
                    );
                });
            ui.add_space(6.0);
            render_install_log(ui, &prog.log);
        }
    } else {
        // Idle: show install form
        ui.label("安装目录：");
        ui.add(
            egui::TextEdit::singleline(&mut state.openclaw_config.install_dir)
                .hint_text("~/.quickclaw")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new("OpenClaw 将被克隆至该目录下的 openclaw/ 子目录")
                .size(12.0)
                .color(egui::Color32::GRAY),
        );
        ui.add_space(8.0);
        ui.label("仓库地址：");
        ui.add(
            egui::TextEdit::singleline(&mut state.openclaw_config.repo_url)
                .desired_width(f32::INFINITY),
        );
        ui.add_space(14.0);

        if ui
            .add(
                egui::Button::new(RichText::new("🚀  开始安装 OpenClaw").size(14.0))
                    .min_size(egui::vec2(200.0, 36.0)),
            )
            .clicked()
        {
            let cfg = OpenClawInstallConfig {
                install_dir: state.openclaw_config.install_dir.clone(),
                repo_url: state.openclaw_config.repo_url.clone(),
            };
            environment::install_openclaw_async(cfg, Arc::clone(&state.openclaw_progress));
        }
    }

    ui.add_space(14.0);
    render_nav_buttons(ui, state, state.openclaw_ready, None);

    if !state.openclaw_ready && !is_running && !prog.finished {
        ui.add_space(4.0);
        if ui
            .small_button("跳过安装 →（已有自托管服务）")
            .clicked()
        {
            state.step = WizardStep::ServerConfig;
        }
    }
}

fn render_install_log(ui: &mut egui::Ui, log: &[String]) {
    if log.is_empty() {
        return;
    }
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .stick_to_bottom(true)
        .id_salt("install_log")
        .show(ui, |ui| {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(12, 12, 18))
                .corner_radius(4)
                .inner_margin(egui::Margin::symmetric(10, 6))
                .show(ui, |ui| {
                    ui.set_min_width(480.0);
                    for line in log {
                        ui.label(
                            RichText::new(line)
                                .size(11.5)
                                .color(egui::Color32::from_rgb(180, 200, 180))
                                .monospace(),
                        );
                    }
                });
        });
}

// ─── Step: Server Config ──────────────────────────────────────────────────────

fn render_server_config(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第三步：配置 OpenClaw 服务器");
    ui.add_space(6.0);
    ui.label(
        RichText::new(
            "确认 OpenClaw 服务器地址。如果您使用了 QuickClaw 自动安装，\
             地址已自动填好。",
        )
        .size(13.0)
        .color(egui::Color32::GRAY),
    );
    ui.add_space(14.0);

    ui.label("服务器地址：");
    ui.add(
        egui::TextEdit::singleline(&mut config.server_url)
            .hint_text("例如: http://localhost:8080")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("支持 http:// 和 https:// 协议")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(14.0);
    ui.label("API 密钥（可选）：");
    ui.add(
        egui::TextEdit::singleline(&mut config.api_key)
            .hint_text("如果服务器需要认证，请输入 API 密钥")
            .password(true)
            .desired_width(f32::INFINITY),
    );

    ui.add_space(20.0);
    ui.horizontal(|ui| {
        if ui.button("测试连接").clicked() {
            state.connection_status =
                Some(test_connection(&config.server_url, &config.api_key));
        }
        if let Some(status) = &state.connection_status {
            match status {
                Ok(msg) => {
                    ui.label(
                        RichText::new(format!("✓ {}", msg)).color(egui::Color32::GREEN),
                    );
                }
                Err(msg) => {
                    ui.label(
                        RichText::new(format!("✗ {}", msg))
                            .color(egui::Color32::from_rgb(255, 100, 100)),
                    );
                }
            }
        }
    });

    ui.add_space(14.0);
    render_nav_buttons(ui, state, true, Some(config));
}

// ─── Step: Claw Identity ──────────────────────────────────────────────────────

fn render_identity(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第四步：定义 Claw 身份");
    ui.add_space(6.0);
    ui.label(
        RichText::new("为您的 Claw 助手定义一个独特的身份，包括名字和角色定位。")
            .size(13.0)
            .color(egui::Color32::GRAY),
    );
    ui.add_space(14.0);

    ui.label("Claw 名称：");
    ui.add(
        egui::TextEdit::singleline(&mut config.claw_name)
            .hint_text("例如: 小助、ARIA、Max...")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("给您的 AI 助手一个专属名字")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(14.0);
    ui.label("Claw 角色定位：");
    ui.add(
        egui::TextEdit::singleline(&mut config.claw_role)
            .hint_text("例如: 工作助手、编程专家、知识百科...")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(14.0);
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(20, 28, 48))
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.label(RichText::new("预览").size(12.0).color(egui::Color32::GRAY));
            ui.horizontal(|ui| {
                ui.label(RichText::new("🦀").size(22.0));
                ui.add_space(6.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} ({})",
                            if config.claw_name.is_empty() {
                                "Claw"
                            } else {
                                &config.claw_name
                            },
                            if config.claw_role.is_empty() {
                                "AI助手"
                            } else {
                                &config.claw_role
                            }
                        ))
                        .strong(),
                    );
                    ui.label(
                        RichText::new("你好！有什么我可以帮助你的吗？")
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                });
            });
        });

    ui.add_space(14.0);
    render_nav_buttons(ui, state, !config.claw_name.is_empty(), Some(config));
}

// ─── Step: Claw Personality ───────────────────────────────────────────────────

fn render_personality(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第五步：定义 Claw 性格");
    ui.add_space(6.0);
    ui.label(
        RichText::new("定义 Claw 的性格特征，让它更符合您的使用习惯。")
            .size(13.0)
            .color(egui::Color32::GRAY),
    );
    ui.add_space(14.0);

    ui.label("常用性格特征（点击切换）：");
    ui.add_space(4.0);
    let preset_traits = [
        "友善", "专业", "幽默", "耐心", "简洁", "详细", "创意", "严谨", "温柔", "活泼",
        "博学", "务实",
    ];
    ui.horizontal_wrapped(|ui| {
        for trait_name in &preset_traits {
            let is_selected = state
                .traits_input
                .split(&[',', '，', '、', ';', '；'][..])
                .any(|t| t.trim() == *trait_name);
            let resp = ui.add(
                egui::Button::new(RichText::new(*trait_name).size(13.0))
                    .fill(if is_selected {
                        egui::Color32::from_rgb(0, 120, 215)
                    } else {
                        egui::Color32::from_rgb(50, 50, 60)
                    })
                    .corner_radius(12),
            );
            if resp.clicked() {
                if is_selected {
                    let traits: Vec<String> = ClawConfig::parse_traits(&state.traits_input)
                        .into_iter()
                        .filter(|t| t != trait_name)
                        .collect();
                    state.traits_input = traits.join("、");
                } else {
                    if !state.traits_input.is_empty() {
                        state.traits_input.push('、');
                    }
                    state.traits_input.push_str(trait_name);
                }
            }
        }
    });

    ui.add_space(10.0);
    ui.label("自定义特征：");
    ui.add(
        egui::TextEdit::singleline(&mut state.traits_input)
            .hint_text("例如: 友善、专业、幽默")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(12.0);
    ui.label("系统提示词：");
    ui.add(
        egui::TextEdit::multiline(&mut config.system_prompt)
            .hint_text("描述 Claw 的性格和行为准则...")
            .desired_width(f32::INFINITY)
            .desired_rows(5),
    );

    ui.add_space(14.0);
    config.personality_traits = ClawConfig::parse_traits(&state.traits_input);
    render_nav_buttons(ui, state, true, Some(config));
}

// ─── Step: Finish ─────────────────────────────────────────────────────────────

fn render_finish(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) -> bool {
    let mut done = false;
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("🎉").size(64.0));
        ui.add_space(14.0);
        ui.label(RichText::new("配置完成！").size(28.0).strong());
        ui.add_space(6.0);
        ui.label(
            RichText::new(format!(
                "{} 已准备就绪，可以开始使用了",
                config.claw_name
            ))
            .size(15.0)
            .color(egui::Color32::GRAY),
        );
        ui.add_space(22.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 28, 48))
            .corner_radius(8)
            .inner_margin(egui::Margin::symmetric(16, 12))
            .show(ui, |ui| {
                ui.set_min_width(380.0);
                ui.label(RichText::new("配置摘要").strong());
                ui.add_space(6.0);
                for (key, value) in &[
                    ("服务器", config.server_url.as_str()),
                    ("Claw 名称", config.claw_name.as_str()),
                    ("角色定位", config.claw_role.as_str()),
                ] {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{}：", key))
                                .size(13.0)
                                .color(egui::Color32::GRAY),
                        );
                        ui.label(
                            RichText::new(if value.is_empty() { "（未设置）" } else { value })
                                .size(13.0),
                        );
                    });
                }
                if !config.personality_traits.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("性格特征：")
                                .size(13.0)
                                .color(egui::Color32::GRAY),
                        );
                        ui.label(RichText::new(config.traits_as_string()).size(13.0));
                    });
                }
            });

        if let Some(Err(err)) = &state.save_result {
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("⚠ 保存失败: {}", err))
                    .color(egui::Color32::from_rgb(255, 150, 150))
                    .size(13.0),
            );
        }

        ui.add_space(22.0);
        if ui
            .add(
                egui::Button::new(RichText::new("开始使用 QuickClaw 🚀").size(16.0))
                    .min_size(egui::vec2(240.0, 44.0)),
            )
            .clicked()
        {
            config.configured = true;
            state.save_result = Some(config.save());
            if state
                .save_result
                .as_ref()
                .map(|r| r.is_ok())
                .unwrap_or(false)
            {
                done = true;
            }
        }
        ui.add_space(6.0);
        if ui
            .button(RichText::new("← 返回修改").size(13.0).color(egui::Color32::GRAY))
            .clicked()
        {
            state.step = WizardStep::ClawPersonality;
        }
    });
    done
}

// ─── Navigation helper ────────────────────────────────────────────────────────

fn render_nav_buttons(
    ui: &mut egui::Ui,
    state: &mut WizardState,
    can_proceed: bool,
    config: Option<&mut ClawConfig>,
) {
    ui.horizontal(|ui| {
        if let Some(prev_step) = state.step.prev() {
            if ui.button("← 上一步").clicked() {
                state.step = prev_step;
                state.connection_status = None;
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let next_label = if state.step.next() == Some(WizardStep::Finish) {
                "完成配置 →"
            } else {
                "下一步 →"
            };

            if ui
                .add_enabled(
                    can_proceed,
                    egui::Button::new(RichText::new(next_label).size(14.0))
                        .min_size(egui::vec2(120.0, 32.0)),
                )
                .clicked()
            {
                if let Some(next_step) = state.step.next() {
                    if next_step == WizardStep::Finish {
                        if let Some(cfg) = config {
                            cfg.personality_traits =
                                ClawConfig::parse_traits(&state.traits_input);
                        }
                    }
                    state.step = next_step;
                    state.connection_status = None;
                }
            }
        });
    });
}

// ─── Connection test ──────────────────────────────────────────────────────────

fn test_connection(url: &str, api_key: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let full_url = format!("{}/v1/models", url.trim_end_matches('/'));
    let mut req = client.get(&full_url);
    if !api_key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", api_key));
    }

    match req.send() {
        Ok(resp) if resp.status().is_success() => Ok("连接成功！服务器正常运行。".into()),
        Ok(resp) if resp.status().as_u16() == 401 => {
            Err("认证失败，请检查 API 密钥。".into())
        }
        Ok(resp) if resp.status().as_u16() == 404 => {
            Ok("服务器已连接（兼容模式）。".into())
        }
        Ok(resp) => Err(format!("服务器返回 {} 状态码", resp.status().as_u16())),
        Err(e) if e.is_connect() => {
            Err("无法连接到服务器，请检查地址是否正确。".into())
        }
        Err(e) if e.is_timeout() => {
            Err("连接超时，请检查服务器是否正在运行。".into())
        }
        Err(e) => Err(format!("连接失败: {}", e)),
    }
}
