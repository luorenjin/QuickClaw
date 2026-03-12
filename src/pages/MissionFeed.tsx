import { useStudio } from "../store/studio";

const MESSAGE_TYPE_LABELS: Record<string, string> = {
  GoalSubmitted: "🎯 目标提交",
  TaskAssigned: "📋 任务派发",
  ProgressReported: "📊 进度汇报",
  ArtifactPublished: "📦 产物发布",
  HandoffRequested: "🔀 交接请求",
  TaskCompleted: "✅ 任务完成",
  SummaryReported: "📝 摘要汇报",
};

const CHANNEL_LABELS: Record<string, string> = {
  human_ceo: "人类→CEO",
  mission: "任务频道",
  workspace: "工作空间",
  handoff: "交接频道",
};

export default function MissionFeed() {
  const { missions, messages, employees, tasks } = useStudio();
  const activeMission = missions.find((m) => m.status === "active");
  const allMissions = [...missions].sort((a, b) => b.createdAt.localeCompare(a.createdAt));

  const getEmployeeName = (id: string) => {
    if (id === "human-owner") return "👤 您";
    const emp = employees.find((e) => e.id === id);
    return emp ? `${emp.avatar || "👤"} ${emp.name}` : id;
  };

  if (allMissions.length === 0) {
    return (
      <div className="page">
        <div className="page-header">
          <h1 className="page-title">📡 任务动态</h1>
          <p className="page-subtitle">任务消息总线 — 团队协作的实时记录</p>
        </div>
        <div className="empty-state">
          <div className="empty-state-icon">📭</div>
          <div className="empty-state-text">还没有任务，快去向 CEO 汇报目标吧！</div>
        </div>
      </div>
    );
  }

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">📡 任务动态</h1>
        <p className="page-subtitle">任务消息总线 — 团队协作的实时记录</p>
      </div>

      {allMissions.map((mission) => {
        const missionMessages = messages
          .filter((m) => m.missionId === mission.id || (!m.missionId && mission.id === activeMission?.id))
          .sort((a, b) => b.timestamp.localeCompare(a.timestamp));
        const missionTasks = tasks.filter((t) => t.missionId === mission.id);

        return (
          <div key={mission.id} className="card" style={{ marginBottom: 20 }}>
            {/* Mission header */}
            <div style={{ display: "flex", alignItems: "flex-start", gap: 12, marginBottom: 16 }}>
              <span style={{ fontSize: 24 }}>🎯</span>
              <div style={{ flex: 1 }}>
                <div style={{ fontSize: 15, fontWeight: 600, color: "#e2e8f0" }}>
                  {mission.title}
                </div>
                <div style={{ fontSize: 12, color: "#718096", marginTop: 2 }}>
                  {new Date(mission.createdAt).toLocaleString()}
                </div>
              </div>
              <span
                className={`badge ${
                  mission.status === "active"
                    ? "badge-green"
                    : mission.status === "completed"
                    ? "badge-blue"
                    : "badge-gray"
                }`}
              >
                {mission.status === "active" ? "进行中" : mission.status === "completed" ? "已完成" : mission.status}
              </span>
            </div>

            <div style={{ padding: "10px 12px", background: "#0f1117", borderRadius: 6, marginBottom: 14,
              borderLeft: "3px solid #2b6cb0" }}>
              <div style={{ fontSize: 12, color: "#718096", marginBottom: 4 }}>目标描述</div>
              <div style={{ fontSize: 13, color: "#a0aec0", lineHeight: 1.6 }}>{mission.brief}</div>
            </div>

            {/* Task summary */}
            {missionTasks.length > 0 && (
              <div style={{ marginBottom: 14 }}>
                <div style={{ fontSize: 12, color: "#718096", marginBottom: 8 }}>📋 任务 ({missionTasks.length})</div>
                <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
                  {missionTasks.map((t) => {
                    const assignee = employees.find((e) => e.id === t.assignedToEmployeeId);
                    return (
                      <div
                        key={t.id}
                        style={{
                          display: "flex",
                          alignItems: "center",
                          gap: 6,
                          padding: "4px 10px",
                          background: "#1a2035",
                          borderRadius: 6,
                          border: "1px solid #2d3748",
                          fontSize: 12,
                        }}
                      >
                        <span>{assignee?.avatar}</span>
                        <span style={{ color: "#e2e8f0" }}>{t.title}</span>
                        <span className={`badge ${t.status === "completed" ? "badge-green" : t.status === "in_progress" ? "badge-yellow" : "badge-gray"}`}
                          style={{ fontSize: 10 }}>
                          {t.status === "pending" ? "待处理" : t.status === "in_progress" ? "进行中" : "已完成"}
                        </span>
                      </div>
                    );
                  })}
                </div>
              </div>
            )}

            {/* Message timeline */}
            <div>
              <div style={{ fontSize: 12, color: "#718096", marginBottom: 8 }}>
                📡 消息动态 ({missionMessages.length})
              </div>
              {missionMessages.length === 0 ? (
                <div style={{ fontSize: 12, color: "#4a5568", padding: "12px 0" }}>
                  暂无消息动态
                </div>
              ) : (
                <div className="message-feed">
                  {missionMessages.map((msg) => (
                    <div key={msg.id} className="message-item">
                      <div className="message-header">
                        <span className={`message-channel channel-${msg.channel}`}>
                          {CHANNEL_LABELS[msg.channel] || msg.channel}
                        </span>
                        <span className="message-type">
                          {MESSAGE_TYPE_LABELS[msg.type] || msg.type}
                        </span>
                        <span className="message-timestamp">
                          {new Date(msg.timestamp).toLocaleTimeString()}
                        </span>
                      </div>
                      <div className="message-body">
                        <span style={{ color: "#718096" }}>
                          {getEmployeeName(msg.fromEmployeeId)}
                          {msg.toEmployeeId && ` → ${getEmployeeName(msg.toEmployeeId)}`}
                        </span>
                        {(() => {
                          const text = (msg.payload.note || msg.payload.title || msg.payload.plan || msg.payload.brief) as string | undefined;
                          return text ? <div style={{ marginTop: 4 }}>{text}</div> : null;
                        })()}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
