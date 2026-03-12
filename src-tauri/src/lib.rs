mod config;
mod environment;

use config::ClawConfig;
use environment::{
    all_dependencies, detect, detect_all, install_dependency_async, install_openclaw_async,
    start_openclaw, Dependency, DepStatus, InstallProgress, OpenClawInstallConfig, SharedProgress,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

// ─── Shared app state ─────────────────────────────────────────────────────────

/// 安装任务状态（由后台线程写入，Tauri 命令读取）
pub struct InstallState {
    /// key = 依赖名称（"Node.js" / "Git" / "Claude Code"）
    pub dep_progress: HashMap<String, SharedProgress>,
    /// OpenClaw 安装进度
    pub openclaw_progress: SharedProgress,
}

impl Default for InstallState {
    fn default() -> Self {
        Self {
            dep_progress: HashMap::new(),
            openclaw_progress: Arc::new(Mutex::new(InstallProgress::default())),
        }
    }
}

pub type SharedInstallState = Mutex<InstallState>;

// ─── Shared structs ───────────────────────────────────────────────────────────

/// 依赖状态的序列化结构（供前端使用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepInfo {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub status: String,
    pub version: Option<String>,
}

impl From<&Dependency> for DepInfo {
    fn from(d: &Dependency) -> Self {
        let (status_key, version) = match &d.status {
            DepStatus::Unknown => ("unknown", None),
            DepStatus::Installed(v) => ("installed", Some(v.clone())),
            DepStatus::Missing => ("missing", None),
            DepStatus::Installing => ("installing", None),
            DepStatus::Failed(e) => ("failed", Some(e.clone())),
        };
        Self {
            name: d.name.to_string(),
            description: d.description.to_string(),
            required: d.required,
            status: status_key.to_string(),
            version,
        }
    }
}

/// 安装进度快照（供前端轮询）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressSnapshot {
    pub current_step: String,
    pub log: Vec<String>,
    pub finished: bool,
    pub error: Option<String>,
}

impl From<&InstallProgress> for ProgressSnapshot {
    fn from(p: &InstallProgress) -> Self {
        Self {
            current_step: p.current_step.clone(),
            log: p.log.clone(),
            finished: p.finished,
            error: p.error.clone(),
        }
    }
}

/// OpenClaw 安装配置（前端传入）
#[derive(Debug, Deserialize)]
pub struct OpenClawInstallRequest {
    pub install_dir: String,
    pub npm_package: String,
}

/// 聊天消息格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

pub mod commands {
    use super::*;

    /// 从磁盘加载配置
    #[tauri::command]
    pub fn load_config() -> ClawConfig {
        ClawConfig::load()
    }

    /// 保存配置到磁盘
    #[tauri::command]
    pub fn save_config(config: ClawConfig) -> Result<(), String> {
        config.save()
    }

    /// 检测所有依赖，返回状态列表
    #[tauri::command]
    pub fn check_dependencies() -> Vec<DepInfo> {
        let mut deps = all_dependencies();
        detect_all(&mut deps);
        deps.iter().map(DepInfo::from).collect()
    }

    /// 检测单个依赖
    #[tauri::command]
    pub fn check_single_dependency(name: String) -> DepInfo {
        let mut deps = all_dependencies();
        if let Some(dep) = deps.iter_mut().find(|d| d.name == name) {
            dep.status = match detect(&name) {
                Some(v) => DepStatus::Installed(v),
                None => DepStatus::Missing,
            };
            DepInfo::from(dep as &Dependency)
        } else {
            DepInfo {
                name,
                description: String::new(),
                required: false,
                status: "unknown".into(),
                version: None,
            }
        }
    }

    /// 启动后台线程安装指定依赖，通过 Tauri 事件推送进度
    #[tauri::command]
    pub async fn install_dependency(
        name: String,
        app: AppHandle,
        state: State<'_, SharedInstallState>,
    ) -> Result<(), String> {
        let progress: SharedProgress = Arc::new(Mutex::new(InstallProgress::default()));
        {
            let mut s = state.lock().unwrap();
            s.dep_progress.insert(name.clone(), Arc::clone(&progress));
        }

        let app_clone = app.clone();
        let name_static: &'static str = Box::leak(name.clone().into_boxed_str());
        let progress_clone = Arc::clone(&progress);

        // Spawn install thread and emit events to frontend
        std::thread::spawn(move || {
            install_dependency_async(name_static, Arc::clone(&progress_clone));
            // Poll and emit progress events until done
            loop {
                std::thread::sleep(std::time::Duration::from_millis(200));
                let snap = {
                    let p = progress_clone.lock().unwrap();
                    ProgressSnapshot::from(&*p)
                };
                let finished = snap.finished;
                let _ = app_clone.emit(&format!("dep-progress:{}", name_static), snap);
                if finished {
                    break;
                }
            }
        });

        Ok(())
    }

    /// 读取依赖安装进度快照
    #[tauri::command]
    pub fn get_dep_progress(name: String, state: State<'_, SharedInstallState>) -> ProgressSnapshot {
        let s = state.lock().unwrap();
        match s.dep_progress.get(&name) {
            Some(p) => ProgressSnapshot::from(&*p.lock().unwrap()),
            None => ProgressSnapshot {
                current_step: String::new(),
                log: Vec::new(),
                finished: false,
                error: None,
            },
        }
    }

    /// 检测 OpenClaw 是否已安装
    #[tauri::command]
    pub fn detect_openclaw(install_dir: String) -> bool {
        environment::detect_openclaw(&install_dir)
    }

    /// 返回默认安装配置（安装目录、npm 包名）
    #[tauri::command]
    pub fn default_openclaw_config() -> serde_json::Value {
        let cfg = OpenClawInstallConfig::default();
        serde_json::json!({
            "install_dir": cfg.install_dir,
            "npm_package": cfg.npm_package,
        })
    }

    /// 启动后台线程安装 OpenClaw，通过 Tauri 事件推送进度
    #[tauri::command]
    pub async fn install_openclaw(
        request: OpenClawInstallRequest,
        app: AppHandle,
        state: State<'_, SharedInstallState>,
    ) -> Result<(), String> {
        let progress: SharedProgress = {
            let mut s = state.lock().unwrap();
            *s.openclaw_progress.lock().unwrap() = InstallProgress::default();
            Arc::clone(&s.openclaw_progress)
        };

        let cfg = OpenClawInstallConfig {
            install_dir: request.install_dir,
            npm_package: request.npm_package,
        };

        let app_clone = app.clone();
        let progress_clone = Arc::clone(&progress);

        std::thread::spawn(move || {
            install_openclaw_async(cfg, Arc::clone(&progress_clone));
            loop {
                std::thread::sleep(std::time::Duration::from_millis(200));
                let snap = {
                    let p = progress_clone.lock().unwrap();
                    ProgressSnapshot::from(&*p)
                };
                let finished = snap.finished;
                let _ = app_clone.emit("openclaw-progress", snap);
                if finished {
                    break;
                }
            }
        });

        Ok(())
    }

    /// 读取 OpenClaw 安装进度快照
    #[tauri::command]
    pub fn get_openclaw_progress(state: State<'_, SharedInstallState>) -> ProgressSnapshot {
        let snap = {
            let s = state.lock().unwrap();
            let p = s.openclaw_progress.lock().unwrap();
            ProgressSnapshot::from(&*p)
        };
        snap
    }

    /// 启动 OpenClaw 本地服务，返回监听端口
    #[tauri::command]
    pub fn launch_openclaw(install_dir: String) -> Result<u16, String> {
        start_openclaw(&install_dir)
    }

    /// 测试与 OpenClaw 服务器的连接
    #[tauri::command]
    pub fn test_connection(url: String, api_key: String) -> Result<String, String> {
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

    /// 发送聊天消息，返回 AI 回复
    #[tauri::command]
    pub fn send_chat_message(
        config: ClawConfig,
        history: Vec<ChatMessage>,
        message: String,
    ) -> Result<String, String> {
        let mut msgs = vec![serde_json::json!({
            "role": "system",
            "content": config.system_prompt
        })];

        for msg in &history {
            msgs.push(serde_json::json!({
                "role": msg.role,
                "content": msg.content
            }));
        }

        msgs.push(serde_json::json!({
            "role": "user",
            "content": message
        }));

        let body = serde_json::json!({
            "model": "default",
            "messages": msgs,
            "stream": false
        });

        let url = format!(
            "{}/v1/chat/completions",
            config.server_url.trim_end_matches('/')
        );

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let mut req = client.post(&url).json(&body);
        if !config.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", config.api_key));
        }

        let resp = req
            .send()
            .map_err(|e| format!("连接服务器失败，请检查服务器地址是否正确: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!(
                "服务器返回错误: {} {}",
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or("未知错误")
            ));
        }

        let data: serde_json::Value = resp
            .json()
            .map_err(|e| format!("解析响应失败: {}", e))?;

        Ok(data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("（空响应）")
            .to_string())
    }
}

// ─── App entry ────────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(SharedInstallState::default())
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::check_dependencies,
            commands::check_single_dependency,
            commands::install_dependency,
            commands::get_dep_progress,
            commands::detect_openclaw,
            commands::default_openclaw_config,
            commands::install_openclaw,
            commands::get_openclaw_progress,
            commands::launch_openclaw,
            commands::test_connection,
            commands::send_chat_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
