interface Props {
  onNext: () => void;
}

const FEATURES = [
  {
    icon: "🔍",
    title: "环境自动检测",
    desc: "自动检测并安装 Node.js、Git、Claude Code",
  },
  {
    icon: "📦",
    title: "OpenClaw 一键安装",
    desc: "自动下载并启动本地 OpenClaw 服务内核",
  },
  {
    icon: "🎭",
    title: "Claw 身份定义",
    desc: "为您的 AI 助手赋予独特的名字和角色定位",
  },
  {
    icon: "✨",
    title: "性格定制",
    desc: "定义 Claw 的性格特征和系统提示词",
  },
  {
    icon: "💬",
    title: "即开即用",
    desc: "配置完成后立即开始与 Claw 对话",
  },
];

export default function WelcomeStep({ onNext }: Props) {
  return (
    <div className="welcome-step">
      <div className="welcome-hero">
        <span className="welcome-icon">🦀</span>
        <h1>欢迎使用 QuickClaw</h1>
        <p className="text-muted">OpenClaw 的 Rust 全平台一站式桌面客户端</p>
      </div>

      <div className="welcome-features">
        {FEATURES.map((f) => (
          <div key={f.title} className="welcome-feature-card">
            <span className="welcome-feature-icon">{f.icon}</span>
            <div>
              <div className="welcome-feature-title">{f.title}</div>
              <div className="welcome-feature-desc text-muted text-sm">
                {f.desc}
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="welcome-action">
        <button className="btn-primary btn-lg" onClick={onNext}>
          开始配置 →
        </button>
      </div>

      <style>{`
        .welcome-step {
          display: flex;
          flex-direction: column;
          align-items: center;
          text-align: center;
          gap: 32px;
        }
        .welcome-hero {
          display: flex;
          flex-direction: column;
          align-items: center;
          gap: 8px;
        }
        .welcome-icon { font-size: 64px; }
        .welcome-hero h1 { font-size: 2rem; }
        .welcome-features {
          display: flex;
          flex-direction: column;
          gap: 10px;
          width: 100%;
          max-width: 480px;
        }
        .welcome-feature-card {
          display: flex;
          align-items: flex-start;
          gap: 14px;
          background: var(--bg-card);
          border: 1px solid var(--border);
          border-radius: var(--radius-md);
          padding: 14px 16px;
          text-align: left;
        }
        .welcome-feature-icon { font-size: 22px; flex-shrink: 0; }
        .welcome-feature-title { font-weight: 600; font-size: 14px; }
        .welcome-feature-desc { margin-top: 2px; }
        .welcome-action { padding-top: 8px; }
      `}</style>
    </div>
  );
}
