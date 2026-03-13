import { useStudio } from "../../store/studio";
import { StudioPage } from "./AppShell";

interface SidebarProps {
  activePage: StudioPage;
  onNavigate: (page: StudioPage, extra?: string) => void;
  selectedWorkspaceId?: string;
}

const NAV_ITEMS: { page: StudioPage; icon: string; label: string }[] = [
  { page: "dashboard", icon: "🏠", label: "仪表盘" },
  { page: "brief", icon: "📝", label: "向 CEO 汇报" },
  { page: "ceo-workspace", icon: "🤖", label: "CEO 工作台" },
  { page: "workspaces", icon: "🗂️", label: "团队工作空间" },
  { page: "mission-feed", icon: "📡", label: "任务动态" },
];

export default function Sidebar({ activePage, onNavigate }: SidebarProps) {
  const { organization, missions, employees } = useStudio();
  const activeMission = missions.find((m) => m.status === "active");

  return (
    <nav className="sidebar">
      <div className="sidebar-brand">
        <span className="sidebar-brand-icon">🦀</span>
        <div className="sidebar-brand-text">
          <span className="sidebar-brand-name">Studio</span>
          <span className="sidebar-brand-org">{organization.name}</span>
        </div>
      </div>

      <div className="sidebar-section">
        {NAV_ITEMS.map(({ page, icon, label }) => (
          <button
            key={page}
            className={`sidebar-item ${activePage === page ? "active" : ""}`}
            onClick={() => onNavigate(page)}
          >
            <span className="sidebar-item-icon">{icon}</span>
            <span className="sidebar-item-label">{label}</span>
          </button>
        ))}
      </div>

      {activeMission && (
        <div className="sidebar-section">
          <div className="sidebar-section-title">当前任务</div>
          <button
            className={`sidebar-item mission-item ${activePage === "mission-feed" ? "active" : ""}`}
            onClick={() => onNavigate("mission-feed")}
          >
            <span className="sidebar-item-icon">🎯</span>
            <span className="sidebar-item-label sidebar-item-truncate">
              {activeMission.title}
            </span>
          </button>
        </div>
      )}

      <div className="sidebar-footer">
        <div className="sidebar-team-summary">
          👥 {employees.filter((e) => e.type === "digital").length} 位数字员工
        </div>
        <button
          className={`sidebar-item ${activePage === "settings" ? "active" : ""}`}
          onClick={() => onNavigate("settings")}
        >
          <span className="sidebar-item-icon">⚙</span>
          <span className="sidebar-item-label">设置</span>
        </button>
      </div>
    </nav>
  );
}
