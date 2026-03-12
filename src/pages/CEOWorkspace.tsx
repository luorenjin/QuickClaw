import { useState } from "react";
import { useStudio } from "../store/studio";
import { StudioPage } from "../components/shell/AppShell";

interface CEOWorkspaceProps {
  onNavigate: (page: StudioPage, extra?: string) => void;
}

const TASK_TEMPLATES = [
  { title: "竞品调研", desc: "分析市场竞品，输出研究摘要", assignTo: "emp-researcher" },
  { title: "技术方案", desc: "制定技术架构与实现方案", assignTo: "emp-builder" },
  { title: "市场推广计划", desc: "制定内容与增长策略", assignTo: "emp-marketer" },
];

export default function CEOWorkspace({ onNavigate }: CEOWorkspaceProps) {
  const { employees, missions, tasks, messages, addTask, publishMessage } = useStudio();
  const ceo = employees.find((e) => e.id === "emp-ceo");
  const activeMission = missions.find((m) => m.status === "active");
  const missionTasks = activeMission ? tasks.filter((t) => t.missionId === activeMission.id) : [];
  const ceoMessages = messages.filter(
    (m) => m.fromEmployeeId === "emp-ceo" || m.toEmployeeId === "emp-ceo"
  );

  const [plan, setPlan] = useState("");
  const [planSaved, setPlanSaved] = useState(false);

  const handleSavePlan = () => {
    if (!plan.trim() || !activeMission) return;
    publishMessage({
      channel: "mission",
      type: "SummaryReported",
      fromEmployeeId: "emp-ceo",
      missionId: activeMission.id,
      payload: { plan },
    });
    setPlanSaved(true);
  };

  const handleAssignTask = (template: (typeof TASK_TEMPLATES)[number]) => {
    if (!activeMission) return;
    const employee = employees.find((e) => e.id === template.assignTo);
    if (!employee) return;
    addTask({
      missionId: activeMission.id,
      workspaceId: employee.workspaceId,
      assignedToEmployeeId: employee.id,
      title: template.title,
      description: template.desc,
      status: "pending",
    });
  };

  if (!activeMission) {
    return (
      <div className="page">
        <div className="page-header">
          <h1 className="page-title">🤖 CEO 工作台</h1>
        </div>
        <div className="empty-state">
          <div className="empty-state-icon">📭</div>
          <div className="empty-state-text" style={{ marginBottom: 16 }}>暂无进行中任务</div>
          <button className="btn btn-primary" onClick={() => onNavigate("brief")}>
            提交一个新目标
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">🤖 CEO 工作台</h1>
        <p className="page-subtitle">
          {ceo?.name} — {ceo?.title}
        </p>
      </div>

      {/* Active mission */}
      <div className="card" style={{ borderColor: "#2b6cb0" }}>
        <div style={{ display: "flex", alignItems: "flex-start", gap: 14 }}>
          <span style={{ fontSize: 28 }}>🎯</span>
          <div style={{ flex: 1 }}>
            <div style={{ fontSize: 15, fontWeight: 600, color: "#e2e8f0", marginBottom: 4 }}>
              {activeMission.title}
            </div>
            <p style={{ margin: 0, fontSize: 13, color: "#a0aec0", lineHeight: 1.6 }}>
              {activeMission.brief}
            </p>
          </div>
          <span className="badge badge-blue">进行中</span>
        </div>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 14 }}>
        {/* Plan area */}
        <div className="card">
          <div className="card-title">📋 制定计划</div>
          {planSaved ? (
            <div style={{ color: "#68d391", fontSize: 13 }}>✅ 计划已发布到任务动态</div>
          ) : (
            <>
              <textarea
                className="form-textarea"
                style={{ minHeight: 100 }}
                placeholder="输入执行计划，将通过任务消息总线发布..."
                value={plan}
                onChange={(e) => setPlan(e.target.value)}
              />
              <button
                className="btn btn-primary"
                style={{ marginTop: 10 }}
                disabled={!plan.trim()}
                onClick={handleSavePlan}
              >
                发布计划
              </button>
            </>
          )}
        </div>

        {/* Quick assign */}
        <div className="card">
          <div className="card-title">👥 快速派任务</div>
          <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
            {TASK_TEMPLATES.map((t) => {
              const emp = employees.find((e) => e.id === t.assignTo);
              const alreadyAssigned = missionTasks.some(
                (mt) => mt.title === t.title && mt.missionId === activeMission.id
              );
              return (
                <div
                  key={t.title}
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 8,
                    padding: "8px 10px",
                    background: "#0f1117",
                    borderRadius: 6,
                    border: "1px solid #2d3748",
                  }}
                >
                  <span style={{ flex: 1, fontSize: 13, color: "#e2e8f0" }}>{t.title}</span>
                  <span style={{ fontSize: 11, color: "#718096" }}>→ {emp?.name}</span>
                  <button
                    className="btn btn-secondary"
                    style={{ padding: "4px 10px", fontSize: 11 }}
                    disabled={alreadyAssigned}
                    onClick={() => handleAssignTask(t)}
                  >
                    {alreadyAssigned ? "已派" : "派发"}
                  </button>
                </div>
              );
            })}
          </div>
        </div>
      </div>

      {/* Assigned tasks */}
      {missionTasks.length > 0 && (
        <div className="card">
          <div className="card-title">📋 已分配任务</div>
          <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
            {missionTasks.map((task) => {
              const assignee = employees.find((e) => e.id === task.assignedToEmployeeId);
              return (
                <div
                  key={task.id}
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 10,
                    padding: "8px 0",
                    borderBottom: "1px solid #2d3748",
                  }}
                >
                  <span style={{ fontSize: 14 }}>{assignee?.avatar || "👤"}</span>
                  <div style={{ flex: 1 }}>
                    <div style={{ fontSize: 13, color: "#e2e8f0" }}>{task.title}</div>
                    <div style={{ fontSize: 11, color: "#718096" }}>
                      {assignee?.name} · {task.description}
                    </div>
                  </div>
                  <span className={`badge ${task.status === "completed" ? "badge-green" : task.status === "in_progress" ? "badge-yellow" : "badge-gray"}`}>
                    {task.status === "pending" ? "待处理" : task.status === "in_progress" ? "进行中" : "已完成"}
                  </span>
                  <button
                    className="btn btn-secondary"
                    style={{ padding: "4px 10px", fontSize: 11 }}
                    onClick={() => onNavigate("workspace-detail", assignee?.workspaceId)}
                  >
                    查看工作空间
                  </button>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* CEO's recent messages */}
      {ceoMessages.length > 0 && (
        <div className="card">
          <div className="card-title">📡 CEO 消息记录</div>
          <div className="message-feed">
            {[...ceoMessages].reverse().slice(0, 5).map((msg) => (
              <div key={msg.id} className="message-item">
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
        </div>
      )}
    </div>
  );
}
