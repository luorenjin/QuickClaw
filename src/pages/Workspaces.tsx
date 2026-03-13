import { useStudio } from "../store/studio";
import { StudioPage } from "../components/shell/AppShell";

interface WorkspacesProps {
  onNavigate: (page: StudioPage, extra?: string) => void;
}

export default function Workspaces({ onNavigate }: WorkspacesProps) {
  const { employees, workspaces, tasks, artifacts, addEmployee } = useStudio();

  const handleAddEmployee = () => {
    addEmployee({
      type: "digital",
      kind: "individual_contributor",
      name: "新员工",
      title: "通用助手",
      description: "新数字员工，待配置",
      status: "idle",
      reportsToEmployeeId: "emp-ceo",
      modelProfileId: "mp-default",
      avatar: "🤖",
    });
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">🗂️ 团队工作空间</h1>
        <p className="page-subtitle">每位员工拥有独立、隔离的工作空间。</p>
      </div>

      <div className="section-row" style={{ marginBottom: 16 }}>
        <span className="section-title" />
        <button className="btn btn-primary" onClick={handleAddEmployee}>
          ＋ 新增数字员工
        </button>
      </div>

      <div className="employee-grid">
        {employees.map((employee) => {
          const workspace = workspaces.find((w) => w.id === employee.workspaceId);
          const empTasks = tasks.filter((t) => t.workspaceId === employee.workspaceId);
          const empArtifacts = artifacts.filter((a) => a.workspaceId === employee.workspaceId);
          const pendingTasks = empTasks.filter((t) => t.status !== "completed");

          return (
            <div
              key={employee.id}
              className="employee-card"
              onClick={() => onNavigate("workspace-detail", employee.workspaceId)}
            >
              <div className="employee-card-avatar">{employee.avatar || "👤"}</div>
              <div className="employee-card-name">{employee.name}</div>
              <div className="employee-card-title">{employee.title}</div>

              <div style={{ display: "flex", gap: 6, flexWrap: "wrap", marginBottom: 10 }}>
                <span className={`employee-card-status status-${employee.status}`}>
                  {employee.status === "active"
                    ? "活跃"
                    : employee.status === "busy"
                    ? "忙碌"
                    : "空闲"}
                </span>
                {employee.kind === "manager" && (
                  <span className="badge badge-blue" style={{ fontSize: 10 }}>管理层</span>
                )}
              </div>

              <div
                style={{
                  display: "flex",
                  gap: 12,
                  fontSize: 12,
                  color: "#718096",
                }}
              >
                <span>📋 {pendingTasks.length} 任务</span>
                <span>📦 {empArtifacts.length} 产物</span>
              </div>

              {workspace && (
                <div
                  style={{
                    marginTop: 10,
                    fontSize: 11,
                    color: "#4a5568",
                    display: "flex",
                    alignItems: "center",
                    gap: 4,
                  }}
                >
                  <span
                    style={{
                      width: 6,
                      height: 6,
                      borderRadius: "50%",
                      background:
                        workspace.status === "active"
                          ? "#68d391"
                          : workspace.status === "busy"
                          ? "#f6ad55"
                          : "#4a5568",
                      display: "inline-block",
                    }}
                  />
                  {workspace.name}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
