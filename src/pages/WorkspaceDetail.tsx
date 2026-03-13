import { useState } from "react";
import { useStudio, DEFAULT_MODEL_PROFILES } from "../store/studio";
import { StudioPage } from "../components/shell/AppShell";
import { ArtifactVisibility } from "../types";

interface WorkspaceDetailProps {
  workspaceId: string;
  onNavigate: (page: StudioPage, extra?: string) => void;
}

export default function WorkspaceDetail({ workspaceId, onNavigate }: WorkspaceDetailProps) {
  const {
    employees,
    workspaces,
    tasks,
    artifacts,
    messages,
    missions,
    addArtifact,
    publishMessage,
    updateEmployee,
  } = useStudio();

  const workspace = workspaces.find((w) => w.id === workspaceId);
  const employee = employees.find((e) => e.workspaceId === workspaceId);
  const wsTasks = tasks.filter((t) => t.workspaceId === workspaceId);
  const wsArtifacts = artifacts.filter((a) => a.workspaceId === workspaceId);
  const wsMessages = messages.filter(
    (m) => m.workspaceId === workspaceId || m.toEmployeeId === employee?.id
  );

  const [artifactName, setArtifactName] = useState("");
  const [artifactContent, setArtifactContent] = useState("");
  const [artifactVisibility, setArtifactVisibility] = useState<ArtifactVisibility>("private");
  const [progressNote, setProgressNote] = useState("");

  if (!workspace || !employee) {
    return (
      <div className="page">
        <div className="empty-state">
          <div className="empty-state-icon">❓</div>
          <div className="empty-state-text">工作空间不存在</div>
          <button className="btn btn-secondary" style={{ marginTop: 16 }}
            onClick={() => onNavigate("workspaces")}>
            返回
          </button>
        </div>
      </div>
    );
  }

  const activeMission = missions.find((m) => m.status === "active");

  const handlePublishArtifact = () => {
    if (!artifactName.trim() || !artifactContent.trim()) return;
    addArtifact({
      workspaceId,
      name: artifactName.trim(),
      type: "note",
      content: artifactContent.trim(),
      visibility: artifactVisibility,
    });
    setArtifactName("");
    setArtifactContent("");
  };

  const handleReportProgress = () => {
    if (!progressNote.trim()) return;
    publishMessage({
      channel: "workspace",
      type: "ProgressReported",
      fromEmployeeId: employee.id,
      toEmployeeId: "emp-ceo",
      missionId: activeMission?.id,
      workspaceId,
      payload: { note: progressNote },
    });
    setProgressNote("");
  };

  return (
    <div className="page">
      <div className="page-header">
        <button
          style={{ background: "none", border: "none", color: "#718096", cursor: "pointer",
            fontSize: 13, padding: "0 0 8px", display: "flex", alignItems: "center", gap: 4 }}
          onClick={() => onNavigate("workspaces")}
        >
          ← 返回团队工作空间
        </button>
        <h1 className="page-title">
          {employee.avatar} {employee.name} 的工作空间
        </h1>
        <p className="page-subtitle">{employee.title} · {employee.description}</p>
      </div>

      {/* Workspace header row */}
      <div className="card" style={{ display: "flex", alignItems: "center", gap: 16, padding: "14px 20px" }}>
        <div style={{ flex: 1 }}>
          <div style={{ fontSize: 13, color: "#718096", marginBottom: 4 }}>模型配置</div>
          <select
            className="form-input"
            style={{ width: "auto", padding: "4px 8px", fontSize: 13 }}
            value={employee.modelProfileId ?? "mp-default"}
            onChange={(e) => updateEmployee(employee.id, { modelProfileId: e.target.value })}
          >
            {DEFAULT_MODEL_PROFILES.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name} ({p.provider})
              </option>
            ))}
          </select>
        </div>
        <div>
          <span className={`employee-card-status status-${employee.status}`}>
            {employee.status === "active" ? "活跃" : employee.status === "busy" ? "忙碌" : "空闲"}
          </span>
        </div>
        <div style={{ fontSize: 12, color: "#4a5568" }}>
          🆔 {workspace.id}
        </div>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 14 }}>
        {/* Tasks */}
        <div className="card">
          <div className="card-title">📋 任务列表 ({wsTasks.length})</div>
          {wsTasks.length === 0 ? (
            <div className="empty-state" style={{ padding: "20px 0" }}>
              <div style={{ fontSize: 24, marginBottom: 8 }}>📭</div>
              <div style={{ fontSize: 12, color: "#4a5568" }}>暂无任务</div>
            </div>
          ) : (
            <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
              {wsTasks.map((task) => (
                <div
                  key={task.id}
                  style={{
                    padding: "8px 10px",
                    background: "#0f1117",
                    borderRadius: 6,
                    border: "1px solid #2d3748",
                  }}
                >
                  <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
                    <span style={{ flex: 1, fontSize: 13, color: "#e2e8f0" }}>{task.title}</span>
                    <span className={`badge ${task.status === "completed" ? "badge-green" : task.status === "in_progress" ? "badge-yellow" : "badge-gray"}`}>
                      {task.status === "pending" ? "待处理" : task.status === "in_progress" ? "进行中" : "已完成"}
                    </span>
                  </div>
                  {task.description && (
                    <div style={{ fontSize: 11, color: "#718096", marginTop: 4 }}>
                      {task.description}
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Artifacts */}
        <div className="card">
          <div className="card-title">📦 产物区 ({wsArtifacts.length})</div>
          {wsArtifacts.map((a) => (
            <div
              key={a.id}
              style={{
                padding: "8px 10px",
                background: "#0f1117",
                borderRadius: 6,
                border: "1px solid #2d3748",
                marginBottom: 6,
              }}
            >
              <div style={{ display: "flex", alignItems: "center", gap: 6, marginBottom: 4 }}>
                <span style={{ flex: 1, fontSize: 13, color: "#e2e8f0" }}>{a.name}</span>
                <span className={`badge ${a.visibility === "private" ? "badge-gray" : "badge-blue"}`}>
                  {a.visibility === "private" ? "私有" : a.visibility === "shared" ? "已共享" : "任务共享"}
                </span>
              </div>
              <div style={{ fontSize: 12, color: "#718096" }}>{a.content.slice(0, 80)}{a.content.length > 80 ? "..." : ""}</div>
            </div>
          ))}

          {/* Publish artifact form */}
          <div style={{ marginTop: 12, borderTop: "1px solid #2d3748", paddingTop: 12 }}>
            <div style={{ fontSize: 12, color: "#718096", marginBottom: 8 }}>发布新产物</div>
            <input
              className="form-input"
              style={{ marginBottom: 6, fontSize: 12 }}
              placeholder="产物名称"
              value={artifactName}
              onChange={(e) => setArtifactName(e.target.value)}
            />
            <textarea
              className="form-textarea"
              style={{ minHeight: 60, fontSize: 12, marginBottom: 6 }}
              placeholder="产物内容..."
              value={artifactContent}
              onChange={(e) => setArtifactContent(e.target.value)}
            />
            <div style={{ display: "flex", gap: 6, alignItems: "center" }}>
              <select
                className="form-input"
                style={{ flex: 1, fontSize: 12, padding: "6px 8px" }}
                value={artifactVisibility}
                onChange={(e) => setArtifactVisibility(e.target.value as ArtifactVisibility)}
              >
                <option value="private">私有</option>
                <option value="shared">共享</option>
                <option value="mission-shared">任务共享</option>
              </select>
              <button
                className="btn btn-primary"
                style={{ fontSize: 12, padding: "6px 12px" }}
                disabled={!artifactName.trim() || !artifactContent.trim()}
                onClick={handlePublishArtifact}
              >
                发布
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Progress report */}
      <div className="card">
        <div className="card-title">📊 汇报进度</div>
        <div style={{ display: "flex", gap: 10 }}>
          <input
            className="form-input"
            style={{ flex: 1 }}
            placeholder="向 CEO 汇报工作进度..."
            value={progressNote}
            onChange={(e) => setProgressNote(e.target.value)}
          />
          <button
            className="btn btn-primary"
            disabled={!progressNote.trim()}
            onClick={handleReportProgress}
          >
            发送
          </button>
        </div>
      </div>

      {/* Workspace messages */}
      {wsMessages.length > 0 && (
        <div className="card">
          <div className="card-title">📡 工作空间消息</div>
          <div className="message-feed">
            {[...wsMessages].reverse().map((msg) => (
              <div key={msg.id} className="message-item">
                <div className="message-header">
                  <span className={`message-channel channel-${msg.channel}`}>{msg.channel}</span>
                  <span className="message-type">{msg.type}</span>
                  <span className="message-timestamp">
                    {new Date(msg.timestamp).toLocaleTimeString()}
                  </span>
                </div>
                {msg.payload && Object.keys(msg.payload).length > 0 && (
                  <div className="message-body">
                    {String(msg.payload.note ?? msg.payload.title ?? msg.payload.plan ?? "")}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
