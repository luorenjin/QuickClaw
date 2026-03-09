use crate::config::ClawConfig;

/// 对话消息
#[derive(Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    #[allow(dead_code)]
    pub timestamp: std::time::SystemTime,
}

/// 消息角色
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl Message {
    pub fn user(content: String) -> Self {
        Self {
            role: MessageRole::User,
            content,
            timestamp: std::time::SystemTime::now(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// 聊天接口状态
#[derive(Default)]
pub struct ChatState {
    /// 对话历史
    pub messages: Vec<Message>,
    /// 当前输入框内容
    pub input: String,
    /// 是否正在等待响应
    pub waiting: bool,
    /// 错误信息
    pub error: Option<String>,
}

impl ChatState {
    /// 发送消息并获取 AI 回复
    pub fn send_message(&mut self, config: &ClawConfig) {
        let content = self.input.trim().to_string();
        if content.is_empty() {
            return;
        }

        self.input.clear();
        self.error = None;
        self.waiting = true;
        self.messages.push(Message::user(content.clone()));

        // 构建请求体（兼容 OpenAI/OpenClaw API 格式）
        let mut msgs = vec![serde_json::json!({
            "role": "system",
            "content": config.system_prompt
        })];

        for msg in &self.messages {
            let role = match msg.role {
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::System => "system",
            };
            msgs.push(serde_json::json!({
                "role": role,
                "content": msg.content
            }));
        }

        let request_body = serde_json::json!({
            "model": "default",
            "messages": msgs,
            "stream": false
        });

        let url = format!("{}/v1/chat/completions", config.server_url.trim_end_matches('/'));
        let api_key = config.api_key.clone();

        // 同步 HTTP 调用
        let result = (|| -> Result<String, String> {
            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

            let mut req = client.post(&url).json(&request_body);

            if !api_key.is_empty() {
                req = req.header("Authorization", format!("Bearer {}", api_key));
            }

            let response = req
                .send()
                .map_err(|e| format!("连接服务器失败，请检查服务器地址是否正确: {}", e))?;

            if !response.status().is_success() {
                return Err(format!(
                    "服务器返回错误: {} {}",
                    response.status().as_u16(),
                    response.status().canonical_reason().unwrap_or("未知错误")
                ));
            }

            let data: serde_json::Value = response
                .json()
                .map_err(|e| format!("解析响应失败: {}", e))?;

            let text = data["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("（空响应）")
                .to_string();

            Ok(text)
        })();

        self.waiting = false;

        match result {
            Ok(reply) => {
                self.messages.push(Message::assistant(reply));
            }
            Err(err) => {
                self.error = Some(err);
            }
        }
    }

    /// 清空对话历史
    pub fn clear(&mut self) {
        self.messages.clear();
        self.error = None;
        self.input.clear();
    }
}

/// 渲染聊天界面
pub fn render(ui: &mut egui::Ui, state: &mut ChatState, config: &ClawConfig) {
    // 标题栏
    ui.horizontal(|ui| {
        ui.heading(format!("与 {} 对话", config.claw_name));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("清空对话").clicked() {
                state.clear();
            }
        });
    });

    ui.separator();

    // 对话历史区域
    let available_height = ui.available_height() - 80.0;
    egui::ScrollArea::vertical()
        .max_height(available_height)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            if state.messages.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "你好！我是 {}，{}",
                            config.claw_name, config.claw_role
                        ))
                        .size(16.0),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("请输入消息开始对话...")
                            .size(14.0)
                            .color(egui::Color32::GRAY),
                    );
                });
            }

            for msg in &state.messages {
                match msg.role {
                    MessageRole::User => {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(0, 120, 215))
                                .corner_radius(8)
                                .inner_margin(egui::Margin::symmetric(12, 8))
                                .show(ui, |ui| {
                                    ui.set_max_width(ui.available_width() * 0.75);
                                    ui.label(
                                        egui::RichText::new(&msg.content)
                                            .color(egui::Color32::WHITE)
                                            .size(14.0),
                                    );
                                });
                        });
                        ui.add_space(4.0);
                    }
                    MessageRole::Assistant => {
                        ui.horizontal_top(|ui| {
                            // Claw 头像
                            ui.label(
                                egui::RichText::new("🦀")
                                    .size(24.0),
                            );
                            ui.add_space(4.0);
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(50, 50, 60))
                                .corner_radius(8)
                                .inner_margin(egui::Margin::symmetric(12, 8))
                                .show(ui, |ui| {
                                    ui.set_max_width(ui.available_width() * 0.75);
                                    ui.label(
                                        egui::RichText::new(&msg.content)
                                            .color(egui::Color32::WHITE)
                                            .size(14.0),
                                    );
                                });
                        });
                        ui.add_space(4.0);
                    }
                    MessageRole::System => {}
                }
            }

            if state.waiting {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🦀").size(24.0));
                    ui.add_space(4.0);
                    ui.spinner();
                    ui.label(
                        egui::RichText::new("正在思考...")
                            .color(egui::Color32::GRAY)
                            .size(14.0),
                    );
                });
            }
        });

    // 错误提示
    if let Some(err) = &state.error.clone() {
        ui.add_space(4.0);
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(100, 30, 30))
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("⚠ {}", err))
                        .color(egui::Color32::from_rgb(255, 150, 150))
                        .size(13.0),
                );
            });
    }

    // 输入框
    ui.add_space(4.0);
    ui.separator();
    ui.horizontal(|ui| {
        let input_id = ui.id().with("chat_input");
        let response = ui.add(
            egui::TextEdit::singleline(&mut state.input)
                .id(input_id)
                .hint_text("输入消息，按 Enter 发送...")
                .desired_width(ui.available_width() - 70.0)
                .font(egui::TextStyle::Body),
        );

        let send_clicked = ui
            .add_enabled(
                !state.waiting && !state.input.trim().is_empty(),
                egui::Button::new("发送"),
            )
            .clicked();

        let enter_pressed = response.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
            && !state.input.trim().is_empty()
            && !state.waiting;

        if send_clicked || enter_pressed {
            state.send_message(config);
        }

        // 自动聚焦输入框
        if !state.waiting {
            response.request_focus();
        }
    });
}
