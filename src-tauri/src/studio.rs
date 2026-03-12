/// Studio domain models and minimal Tauri commands
use serde::{Deserialize, Serialize};

// ─── Domain Models ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelProfile {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model: String,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub organization_id: String,
    #[serde(rename = "type")]
    pub employee_type: String, // "human" | "digital"
    pub kind: String, // "individual_contributor" | "manager"
    pub name: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub workspace_id: String,
    pub reports_to_employee_id: Option<String>,
    pub model_profile_id: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub organization_id: String,
    pub employee_id: String,
    pub name: String,
    pub status: String, // "active" | "idle" | "busy"
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub status: String, // "draft" | "active" | "completed" | "paused"
    pub created_by_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub mission_id: String,
    pub workspace_id: String,
    pub assigned_to_employee_id: String,
    pub title: String,
    pub description: String,
    pub status: String, // "pending" | "in_progress" | "completed" | "blocked"
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: String,
    pub workspace_id: String,
    pub task_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub content: String,
    pub visibility: String, // "private" | "shared" | "mission-shared"
    pub created_at: String,
}

/// Message envelope for the Studio coordination bus.
/// Channels: human_ceo | mission | workspace | handoff
/// Types: GoalSubmitted | TaskAssigned | ProgressReported |
///        ArtifactPublished | HandoffRequested | TaskCompleted | SummaryReported
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StudioMessage {
    pub id: String,
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub from_employee_id: String,
    pub to_employee_id: Option<String>,
    pub mission_id: Option<String>,
    pub workspace_id: Option<String>,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

// ─── Tauri Commands ───────────────────────────────────────────────────────────

/// Return the default seed organization with mock employees/workspaces.
/// The frontend manages live state; this command provides an initial snapshot
/// that could later be replaced by persisted storage.
#[tauri::command]
pub fn get_studio_seed() -> serde_json::Value {
    let now = chrono::Utc::now().to_rfc3339();
    let org_id = "org-01";

    let employees = vec![
        Employee {
            id: "emp-ceo".into(),
            organization_id: org_id.into(),
            employee_type: "digital".into(),
            kind: "manager".into(),
            name: "Alex".into(),
            title: "CEO".into(),
            description: "数字首席执行官，统筹团队任务".into(),
            status: "active".into(),
            workspace_id: "ws-ceo".into(),
            reports_to_employee_id: None,
            model_profile_id: Some("mp-default".into()),
            avatar: Some("🤖".into()),
        },
        Employee {
            id: "emp-researcher".into(),
            organization_id: org_id.into(),
            employee_type: "digital".into(),
            kind: "individual_contributor".into(),
            name: "Riley".into(),
            title: "研究员".into(),
            description: "收集信息、分析竞品、输出研究摘要".into(),
            status: "idle".into(),
            workspace_id: "ws-researcher".into(),
            reports_to_employee_id: Some("emp-ceo".into()),
            model_profile_id: Some("mp-default".into()),
            avatar: Some("🔬".into()),
        },
    ];

    let workspaces: Vec<Workspace> = employees
        .iter()
        .map(|e| Workspace {
            id: e.workspace_id.clone(),
            organization_id: org_id.into(),
            employee_id: e.id.clone(),
            name: format!("{} 的工作空间", e.name),
            status: if e.status == "active" {
                "active".into()
            } else {
                "idle".into()
            },
            created_at: now.clone(),
        })
        .collect();

    serde_json::json!({
        "organization": {
            "id": org_id,
            "name": "我的一人公司",
            "mode": "general",
            "ownerId": "human-owner",
            "createdAt": now,
        },
        "employees": employees,
        "workspaces": workspaces,
    })
}

/// Validate a message envelope shape before it is stored/forwarded.
/// Returns the message back if valid, or an error string.
#[tauri::command]
pub fn validate_studio_message(message: StudioMessage) -> Result<StudioMessage, String> {
    const VALID_CHANNELS: &[&str] = &["human_ceo", "mission", "workspace", "handoff"];
    const VALID_TYPES: &[&str] = &[
        "GoalSubmitted",
        "TaskAssigned",
        "ProgressReported",
        "ArtifactPublished",
        "HandoffRequested",
        "TaskCompleted",
        "SummaryReported",
    ];

    if !VALID_CHANNELS.contains(&message.channel.as_str()) {
        return Err(format!(
            "无效的消息频道: {}。合法值: {:?}",
            message.channel, VALID_CHANNELS
        ));
    }
    if !VALID_TYPES.contains(&message.message_type.as_str()) {
        return Err(format!(
            "无效的消息类型: {}。合法值: {:?}",
            message.message_type, VALID_TYPES
        ));
    }
    if message.from_employee_id.is_empty() {
        return Err("fromEmployeeId 不能为空".into());
    }
    Ok(message)
}
