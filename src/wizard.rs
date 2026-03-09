use crate::config::ClawConfig;
use egui::RichText;

/// 安装向导步骤
#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    /// 欢迎页面
    Welcome,
    /// 服务器配置
    ServerConfig,
    /// Claw 身份定义
    ClawIdentity,
    /// Claw 性格定义
    ClawPersonality,
    /// 完成
    Finish,
}

impl WizardStep {
    /// Returns the number of steps shown in the progress indicator.
    /// The Welcome step is excluded from the progress indicator, so this
    /// reflects steps 1–4: ServerConfig, ClawIdentity, ClawPersonality, Finish.
    #[allow(dead_code)]
    pub fn total_steps() -> usize {
        4
    }

    pub fn current_index(&self) -> usize {
        match self {
            WizardStep::Welcome => 0,
            WizardStep::ServerConfig => 1,
            WizardStep::ClawIdentity => 2,
            WizardStep::ClawPersonality => 3,
            WizardStep::Finish => 4,
        }
    }

    pub fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Welcome => Some(WizardStep::ServerConfig),
            WizardStep::ServerConfig => Some(WizardStep::ClawIdentity),
            WizardStep::ClawIdentity => Some(WizardStep::ClawPersonality),
            WizardStep::ClawPersonality => Some(WizardStep::Finish),
            WizardStep::Finish => None,
        }
    }

    pub fn prev(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Welcome => None,
            WizardStep::ServerConfig => Some(WizardStep::Welcome),
            WizardStep::ClawIdentity => Some(WizardStep::ServerConfig),
            WizardStep::ClawPersonality => Some(WizardStep::ClawIdentity),
            WizardStep::Finish => Some(WizardStep::ClawPersonality),
        }
    }
}

/// 向导状态
pub struct WizardState {
    pub step: WizardStep,
    /// 临时存储用户输入的性格特征字符串
    pub traits_input: String,
    /// 连接测试结果
    pub connection_status: Option<Result<String, String>>,
    /// 保存配置的结果
    pub save_result: Option<Result<(), String>>,
}

impl WizardState {
    pub fn new(config: &ClawConfig) -> Self {
        Self {
            step: if config.configured {
                WizardStep::Finish
            } else {
                WizardStep::Welcome
            },
            traits_input: config.traits_as_string(),
            connection_status: None,
            save_result: None,
        }
    }
}

/// 渲染向导界面，返回 true 表示向导完成
pub fn render(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) -> bool {
    let mut finished = false;

    // 进度指示器（非欢迎/完成页面）
    if state.step != WizardStep::Welcome && state.step != WizardStep::Finish {
        render_progress(ui, &state.step);
        ui.add_space(16.0);
    }

    match state.step.clone() {
        WizardStep::Welcome => {
            render_welcome(ui, state);
        }
        WizardStep::ServerConfig => {
            render_server_config(ui, state, config);
        }
        WizardStep::ClawIdentity => {
            render_identity(ui, state, config);
        }
        WizardStep::ClawPersonality => {
            render_personality(ui, state, config);
        }
        WizardStep::Finish => {
            finished = render_finish(ui, state, config);
        }
    }

    finished
}

fn render_progress(ui: &mut egui::Ui, step: &WizardStep) {
    let current = step.current_index();
    // The Welcome step is not shown in the progress indicator; we display
    // the 4 post-welcome steps: ServerConfig (1), Identity (2), Personality (3), Finish (4).
    let labels = ["服务器配置", "身份定义", "性格定义", "完成"];

    ui.horizontal(|ui| {
        for (i, label) in labels.iter().enumerate() {
            let is_current = i + 1 == current;
            let is_done = i + 1 < current;

            if i > 0 {
                ui.label(
                    RichText::new("─────")
                        .color(if is_done || is_current {
                            egui::Color32::from_rgb(0, 120, 215)
                        } else {
                            egui::Color32::DARK_GRAY
                        })
                        .size(12.0),
                );
            }

            let (text_color, bg_color) = if is_current {
                (egui::Color32::WHITE, egui::Color32::from_rgb(0, 120, 215))
            } else if is_done {
                (
                    egui::Color32::WHITE,
                    egui::Color32::from_rgb(0, 160, 80),
                )
            } else {
                (egui::Color32::GRAY, egui::Color32::DARK_GRAY)
            };

            egui::Frame::new()
                .fill(bg_color)
                .corner_radius(12)
                .inner_margin(egui::Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(if is_done {
                            format!("✓ {}", label)
                        } else {
                            label.to_string()
                        })
                        .color(text_color)
                        .size(12.0),
                    );
                });
        }
    });
}

fn render_welcome(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("🦀").size(64.0));
        ui.add_space(16.0);
        ui.label(RichText::new("欢迎使用 QuickClaw").size(28.0).strong());
        ui.add_space(8.0);
        ui.label(
            RichText::new("OpenClaw 的一站式桌面客户端")
                .size(16.0)
                .color(egui::Color32::GRAY),
        );
        ui.add_space(32.0);

        let features = [
            ("🚀", "一键引导式安装", "几分钟内完成所有配置"),
            ("🎭", "Claw 身份定义", "为您的 AI 助手赋予独特的身份"),
            ("✨", "性格定制", "定义 Claw 的性格特征和行为风格"),
            ("💬", "即开即用", "配置完成后立即开始对话"),
        ];

        for (icon, title, desc) in &features {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(30, 30, 40))
                .corner_radius(8)
                .inner_margin(egui::Margin::symmetric(16, 12))
                .show(ui, |ui| {
                    ui.set_min_width(400.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(*icon).size(24.0));
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
            ui.add_space(8.0);
        }

        ui.add_space(24.0);

        if ui
            .add(
                egui::Button::new(RichText::new("开始配置 →").size(16.0))
                    .min_size(egui::vec2(200.0, 44.0)),
            )
            .clicked()
        {
            state.step = WizardStep::ServerConfig;
        }
    });
}

fn render_server_config(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第一步：配置 OpenClaw 服务器");
    ui.add_space(8.0);
    ui.label(
        RichText::new("请输入您的 OpenClaw 服务器地址，以便 QuickClaw 连接并与 Claw 通信。")
            .color(egui::Color32::GRAY),
    );
    ui.add_space(16.0);

    ui.label("服务器地址：");
    ui.add(
        egui::TextEdit::singleline(&mut config.server_url)
            .hint_text("例如: http://localhost:8080 或 https://api.example.com")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("支持 http:// 和 https:// 协议")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(16.0);
    ui.label("API 密钥（可选）：");
    ui.add(
        egui::TextEdit::singleline(&mut config.api_key)
            .hint_text("如果服务器需要认证，请输入 API 密钥")
            .password(true)
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("如果您的 OpenClaw 服务器不需要认证，可以留空")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(24.0);

    // 连接测试
    ui.horizontal(|ui| {
        if ui.button("测试连接").clicked() {
            state.connection_status = Some(test_connection(&config.server_url, &config.api_key));
        }

        if let Some(status) = &state.connection_status {
            match status {
                Ok(msg) => {
                    ui.label(RichText::new(format!("✓ {}", msg)).color(egui::Color32::GREEN));
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

    ui.add_space(16.0);
    render_nav_buttons(ui, state, config, true);
}

fn render_identity(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第二步：定义 Claw 身份");
    ui.add_space(8.0);
    ui.label(
        RichText::new("为您的 Claw 助手定义一个独特的身份，包括名字和角色定位。")
            .color(egui::Color32::GRAY),
    );
    ui.add_space(16.0);

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

    ui.add_space(16.0);
    ui.label("Claw 角色定位：");
    ui.add(
        egui::TextEdit::singleline(&mut config.claw_role)
            .hint_text("例如: 工作助手、编程专家、知识百科...")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("描述 Claw 的主要职责和专长领域")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(16.0);

    // 预览
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(20, 30, 50))
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.label(RichText::new("预览").size(12.0).color(egui::Color32::GRAY));
            ui.horizontal(|ui| {
                ui.label(RichText::new("🦀").size(24.0));
                ui.add_space(8.0);
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

    ui.add_space(16.0);
    render_nav_buttons(ui, state, config, !config.claw_name.is_empty());
}

fn render_personality(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) {
    ui.heading("第三步：定义 Claw 性格");
    ui.add_space(8.0);
    ui.label(
        RichText::new("定义 Claw 的性格特征，让它更符合您的使用习惯和期望。")
            .color(egui::Color32::GRAY),
    );
    ui.add_space(16.0);

    // 快速选择性格特征
    ui.label("常用性格特征（点击添加）：");
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

            let button_color = if is_selected {
                egui::Color32::from_rgb(0, 120, 215)
            } else {
                egui::Color32::from_rgb(50, 50, 60)
            };

            let resp = ui.add(
                egui::Button::new(RichText::new(*trait_name).size(13.0))
                    .fill(button_color)
                    .corner_radius(12),
            );

            if resp.clicked() {
                if is_selected {
                    // 移除
                    let traits: Vec<String> = ClawConfig::parse_traits(&state.traits_input)
                        .into_iter()
                        .filter(|t| t != trait_name)
                        .collect();
                    state.traits_input = traits.join("、");
                } else {
                    // 添加
                    if !state.traits_input.is_empty() {
                        state.traits_input.push('、');
                    }
                    state.traits_input.push_str(trait_name);
                }
            }
        }
    });

    ui.add_space(12.0);
    ui.label("自定义性格特征（可用逗号、顿号分隔多个特征）：");
    ui.add(
        egui::TextEdit::singleline(&mut state.traits_input)
            .hint_text("例如: 友善、专业、幽默、耐心")
            .desired_width(f32::INFINITY),
    );

    ui.add_space(16.0);
    ui.label("系统提示词（详细描述 Claw 的性格和行为准则）：");
    ui.add(
        egui::TextEdit::multiline(&mut config.system_prompt)
            .hint_text(
                "例如: 你是一个友善、专业的智能助手。你会用简洁清晰的语言回答问题...",
            )
            .desired_width(f32::INFINITY)
            .desired_rows(5),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new("系统提示词会在每次对话开始时发送给 Claw，定义其行为方式")
            .size(12.0)
            .color(egui::Color32::GRAY),
    );

    ui.add_space(16.0);

    // 更新 config 中的 traits
    config.personality_traits = ClawConfig::parse_traits(&state.traits_input);

    render_nav_buttons(ui, state, config, true);
}

fn render_finish(ui: &mut egui::Ui, state: &mut WizardState, config: &mut ClawConfig) -> bool {
    let mut done = false;

    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("🎉").size(64.0));
        ui.add_space(16.0);
        ui.label(RichText::new("配置完成！").size(28.0).strong());
        ui.add_space(8.0);
        ui.label(
            RichText::new(format!(
                "{} 已准备就绪，可以开始使用了",
                config.claw_name
            ))
            .size(16.0)
            .color(egui::Color32::GRAY),
        );

        ui.add_space(24.0);

        // 配置摘要
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(20, 30, 50))
            .corner_radius(8)
            .inner_margin(egui::Margin::symmetric(16, 12))
            .show(ui, |ui| {
                ui.set_min_width(380.0);
                ui.label(RichText::new("配置摘要").strong());
                ui.add_space(8.0);

                let items = [
                    ("服务器", config.server_url.as_str()),
                    ("Claw 名称", config.claw_name.as_str()),
                    ("角色定位", config.claw_role.as_str()),
                ];

                for (key, value) in &items {
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
                        ui.label(
                            RichText::new(config.traits_as_string()).size(13.0),
                        );
                    });
                }
            });

        if let Some(Err(err)) = &state.save_result {
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("⚠ 保存配置时出错: {}", err))
                    .color(egui::Color32::from_rgb(255, 150, 150))
                    .size(13.0),
            );
        }

        ui.add_space(24.0);

        if ui
            .add(
                egui::Button::new(RichText::new("开始使用 QuickClaw 🚀").size(16.0))
                    .min_size(egui::vec2(240.0, 44.0)),
            )
            .clicked()
        {
            config.configured = true;
            state.save_result = Some(config.save());
            if state.save_result.as_ref().map(|r| r.is_ok()).unwrap_or(false) {
                done = true;
            }
        }

        ui.add_space(8.0);

        if ui
            .button(RichText::new("← 返回修改").size(13.0).color(egui::Color32::GRAY))
            .clicked()
        {
            state.step = WizardStep::ClawPersonality;
        }
    });

    done
}

fn render_nav_buttons(
    ui: &mut egui::Ui,
    state: &mut WizardState,
    config: &mut ClawConfig,
    can_proceed: bool,
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
                    state.step = next_step;
                    state.connection_status = None;
                    // 在进入完成步骤前同步 traits
                    if state.step == WizardStep::Finish {
                        config.personality_traits =
                            ClawConfig::parse_traits(&state.traits_input);
                    }
                }
            }
        });
    });
}

/// 测试与 OpenClaw 服务器的连接
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
        Ok(resp) if resp.status().is_success() => Ok("连接成功！服务器正常运行。".to_string()),
        Ok(resp) if resp.status().as_u16() == 401 => {
            Err("认证失败，请检查 API 密钥。".to_string())
        }
        Ok(resp) if resp.status().as_u16() == 404 => {
            // 404 可能是服务器存在但端点不同
            Ok("服务器已连接（兼容模式）。".to_string())
        }
        Ok(resp) => Err(format!(
            "服务器返回 {} 状态码",
            resp.status().as_u16()
        )),
        Err(e) if e.is_connect() => Err("无法连接到服务器，请检查地址是否正确。".to_string()),
        Err(e) if e.is_timeout() => Err("连接超时，请检查服务器是否正在运行。".to_string()),
        Err(e) => Err(format!("连接失败: {}", e)),
    }
}
