import { useStudio } from "../store/studio";
import { StudioPage } from "../components/shell/AppShell";

interface DashboardProps {
  onNavigate: (page: StudioPage, extra?: string) => void;
}

export default function Dashboard({ onNavigate }: DashboardProps) {
  const { organization, employees, missions, messages } = useStudio();
  const activeMissions = missions.filter((m) => m.status === "active");
  const recentMessages = [...messages]
    .sort((a, b) => b.timestamp.localeCompare(a.timestamp))
    .slice(0, 5);

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">🏠 {organization.name}</h1>
        <p className="page-subtitle">欢迎回来！以下是您的工作台概览。</p>
      </div>

      {/* Quick action */}
      <div className="card" style={{ background: "linear-gradient(135deg, #1a2035, #1e3a5f)" }}>
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
          <div>
            <div className="card-title" style={{ marginBottom: 4 }}>
              {activeMissions.length > 0
                ? `🎯 当前有 ${activeMissions.length} 个进行中任务`
                : "开始一个新任务"}
            </div>
            <p style={{ margin: 0, fontSize: 13, color: "#718096" }}>
              {activeMissions.length > 0
                ? activeMissions[0].title
                : "向 CEO 汇报您的目标，启动团队协作"}
            </p>
          </div>
          <button
            className="btn btn-primary"
            onClick={() => onNavigate(activeMissions.length > 0 ? "mission-feed" : "brief")}
          >
            {activeMissions.length > 0 ? "查看动态" : "开始汇报"}
          </button>
        </div>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 14 }}>
        {/* Team overview */}
        <div className="card">
          <div className="section-row">
            <span className="section-title">👥 团队成员</span>
            <button className="btn btn-secondary" style={{ padding: "5px 12px", fontSize: 12 }}
              onClick={() => onNavigate("workspaces")}>
              查看全部
            </button>
          </div>
          {employees.map((e) => (
            <div
              key={e.id}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 10,
                padding: "8px 0",
                borderBottom: "1px solid #2d3748",
                cursor: "pointer",
              }}
              onClick={() => onNavigate("workspace-detail", e.workspaceId)}
            >
              <span style={{ fontSize: 20 }}>{e.avatar || "👤"}</span>
              <div style={{ flex: 1 }}>
                <div style={{ fontSize: 13, fontWeight: 500, color: "#e2e8f0" }}>{e.name}</div>
                <div style={{ fontSize: 11, color: "#718096" }}>{e.title}</div>
              </div>
              <span className={`employee-card-status status-${e.status}`}>
                {e.status === "active" ? "活跃" : e.status === "busy" ? "忙碌" : "空闲"}
              </span>
            </div>
          ))}
        </div>

        {/* Recent messages */}
        <div className="card">
          <div className="section-row">
            <span className="section-title">📡 近期动态</span>
            <button className="btn btn-secondary" style={{ padding: "5px 12px", fontSize: 12 }}
              onClick={() => onNavigate("mission-feed")}>
              查看全部
            </button>
          </div>
          {recentMessages.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-icon">📭</div>
              <div className="empty-state-text">暂无动态</div>
            </div>
          ) : (
            <div className="message-feed">
              {recentMessages.map((msg) => (
                <div key={msg.id} className="message-item" style={{ marginBottom: 0 }}>
                  <div className="message-header">
                    <span className={`message-channel channel-${msg.channel}`}>{msg.channel}</span>
                    <span className="message-type">{msg.type}</span>
                    <span className="message-timestamp">
                      {new Date(msg.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
