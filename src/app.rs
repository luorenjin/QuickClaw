use crate::chat::ChatState;
use crate::config::ClawConfig;
use crate::wizard::WizardState;
use egui::RichText;

/// 应用程序的主要状态
#[derive(PartialEq)]
enum AppScreen {
    /// 安装向导
    Wizard,
    /// 主聊天界面
    Chat,
    /// 设置页面
    Settings,
}

/// QuickClaw 主应用
pub struct QuickClawApp {
    /// 当前配置
    config: ClawConfig,
    /// 当前显示的界面
    screen: AppScreen,
    /// 安装向导状态
    wizard: WizardState,
    /// 聊天状态
    chat: ChatState,
    /// 设置页面临时状态
    settings_traits_input: String,
    /// 设置保存结果
    settings_save_result: Option<Result<(), String>>,
}

impl QuickClawApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = ClawConfig::load();
        let wizard = WizardState::new(&config);
        let screen = if config.configured {
            AppScreen::Chat
        } else {
            AppScreen::Wizard
        };
        let settings_traits_input = config.traits_as_string();

        Self {
            config,
            screen,
            wizard,
            chat: ChatState::default(),
            settings_traits_input,
            settings_save_result: None,
        }
    }
}

impl eframe::App for QuickClawApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部导航栏（仅在非向导模式下显示）
        if self.screen != AppScreen::Wizard {
            egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("🦀 QuickClaw").size(16.0).strong());
                    ui.add_space(16.0);

                    if ui
                        .selectable_label(self.screen == AppScreen::Chat, "💬 对话")
                        .clicked()
                    {
                        self.screen = AppScreen::Chat;
                    }

                    if ui
                        .selectable_label(self.screen == AppScreen::Settings, "⚙ 设置")
                        .clicked()
                    {
                        self.screen = AppScreen::Settings;
                        self.settings_traits_input = self.config.traits_as_string();
                        self.settings_save_result = None;
                    }
                });
            });
        }

        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                AppScreen::Wizard => {
                    let done =
                        crate::wizard::render(ui, &mut self.wizard, &mut self.config);
                    if done {
                        self.screen = AppScreen::Chat;
                        self.chat = ChatState::default();
                    }
                }
                AppScreen::Chat => {
                    crate::chat::render(ui, &mut self.chat, &self.config);
                }
                AppScreen::Settings => {
                    self.render_settings(ui);
                }
            }
        });
    }
}

impl QuickClawApp {
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // 服务器配置
            ui.label(RichText::new("服务器配置").size(16.0).strong());
            ui.add_space(8.0);

            ui.label("服务器地址：");
            ui.add(
                egui::TextEdit::singleline(&mut self.config.server_url)
                    .hint_text("http://localhost:8080")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(8.0);

            ui.label("API 密钥（可选）：");
            ui.add(
                egui::TextEdit::singleline(&mut self.config.api_key)
                    .password(true)
                    .hint_text("留空表示不需要认证")
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // Claw 身份
            ui.label(RichText::new("Claw 身份定义").size(16.0).strong());
            ui.add_space(8.0);

            ui.label("Claw 名称：");
            ui.add(
                egui::TextEdit::singleline(&mut self.config.claw_name)
                    .hint_text("Claw")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(8.0);

            ui.label("角色定位：");
            ui.add(
                egui::TextEdit::singleline(&mut self.config.claw_role)
                    .hint_text("智能助手")
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // 性格定义
            ui.label(RichText::new("Claw 性格定义").size(16.0).strong());
            ui.add_space(8.0);

            ui.label("性格特征：");
            ui.add(
                egui::TextEdit::singleline(&mut self.settings_traits_input)
                    .hint_text("友善、专业、耐心")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new("使用逗号、顿号分隔多个特征")
                    .size(12.0)
                    .color(egui::Color32::GRAY),
            );

            ui.add_space(8.0);
            ui.label("系统提示词：");
            ui.add(
                egui::TextEdit::multiline(&mut self.config.system_prompt)
                    .hint_text("描述 Claw 的性格和行为准则...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(6),
            );

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // 操作按钮
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::new("保存配置").size(14.0))
                            .min_size(egui::vec2(120.0, 32.0)),
                    )
                    .clicked()
                {
                    self.config.personality_traits =
                        ClawConfig::parse_traits(&self.settings_traits_input);
                    self.settings_save_result = Some(self.config.save());
                }

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("重新运行向导").size(14.0).color(egui::Color32::GRAY),
                        )
                        .fill(egui::Color32::from_rgb(40, 40, 50)),
                    )
                    .clicked()
                {
                    self.wizard = WizardState::new(&ClawConfig::default());
                    self.screen = AppScreen::Wizard;
                }
            });

            if let Some(result) = &self.settings_save_result {
                ui.add_space(8.0);
                match result {
                    Ok(()) => {
                        ui.label(
                            RichText::new("✓ 配置已保存")
                                .color(egui::Color32::GREEN)
                                .size(13.0),
                        );
                    }
                    Err(err) => {
                        ui.label(
                            RichText::new(format!("✗ 保存失败: {}", err))
                                .color(egui::Color32::from_rgb(255, 100, 100))
                                .size(13.0),
                        );
                    }
                }
            }
        });
    }
}
