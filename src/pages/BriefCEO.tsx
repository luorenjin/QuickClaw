import { useState } from "react";
import { useStudio } from "../store/studio";
import { StudioPage } from "../components/shell/AppShell";

interface BriefCEOProps {
  onNavigate: (page: StudioPage) => void;
}

export default function BriefCEO({ onNavigate }: BriefCEOProps) {
  const { submitBrief, missions } = useStudio();
  const [title, setTitle] = useState("");
  const [brief, setBrief] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const activeMission = missions.find((m) => m.status === "active");

  const handleSubmit = () => {
    if (!title.trim() || !brief.trim()) return;
    submitBrief(title.trim(), brief.trim());
    setSubmitted(true);
  };

  if (submitted) {
    return (
      <div className="page">
        <div className="page-header">
          <h1 className="page-title">📝 汇报目标</h1>
        </div>
        <div className="card" style={{ textAlign: "center", padding: "40px 20px" }}>
          <div style={{ fontSize: 48, marginBottom: 16 }}>✅</div>
          <h2 style={{ margin: "0 0 8px", color: "#e2e8f0" }}>目标已提交给 CEO！</h2>
          <p style={{ color: "#718096", marginBottom: 24 }}>
            CEO 将制定计划并分配任务给团队成员。
          </p>
          <div style={{ display: "flex", gap: 10, justifyContent: "center" }}>
            <button className="btn btn-primary" onClick={() => onNavigate("ceo-workspace")}>
              查看 CEO 工作台
            </button>
            <button className="btn btn-secondary" onClick={() => onNavigate("mission-feed")}>
              任务动态
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">📝 向 CEO 汇报目标</h1>
        <p className="page-subtitle">描述您的目标，CEO 将制定计划并组织团队执行。</p>
      </div>

      {activeMission && (
        <div className="card" style={{ borderColor: "#f6ad55", marginBottom: 20 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
            <span>⚠️</span>
            <span style={{ fontSize: 13, color: "#f6ad55" }}>
              当前已有进行中任务：<strong>{activeMission.title}</strong>。新提交将创建另一个任务。
            </span>
          </div>
        </div>
      )}

      <div className="card">
        <div className="form-group">
          <label className="form-label">任务名称</label>
          <input
            className="form-input"
            placeholder="例如：开发一个简洁的落地页"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
          />
        </div>
        <div className="form-group">
          <label className="form-label">目标描述</label>
          <textarea
            className="form-textarea"
            style={{ minHeight: 140 }}
            placeholder="详细描述您的目标、期望成果、约束条件等..."
            value={brief}
            onChange={(e) => setBrief(e.target.value)}
          />
        </div>
        <div style={{ display: "flex", gap: 10 }}>
          <button
            className="btn btn-primary"
            disabled={!title.trim() || !brief.trim()}
            onClick={handleSubmit}
          >
            🚀 提交给 CEO
          </button>
          <button className="btn btn-secondary" onClick={() => onNavigate("dashboard")}>
            取消
          </button>
        </div>
      </div>

      {/* Guide */}
      <div className="card" style={{ background: "rgba(43,108,176,0.08)", borderColor: "#2b6cb0" }}>
        <div className="card-title" style={{ color: "#63b3ed" }}>💡 如何写好目标描述</div>
        <ul style={{ margin: 0, paddingLeft: 18, color: "#718096", fontSize: 13, lineHeight: 1.8 }}>
          <li>清楚描述期望结果，而不仅仅是任务</li>
          <li>说明截止时间或优先级</li>
          <li>提及约束条件（预算、技术栈等）</li>
          <li>描述成功标准</li>
        </ul>
      </div>
    </div>
  );
}
