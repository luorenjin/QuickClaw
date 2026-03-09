import { ClawConfig } from "../../types";

interface Props {
  config: ClawConfig;
  onFinish: () => void;
  onBack: () => void;
}

export default function FinishStep({ config, onFinish, onBack }: Props) {
  return (
    <div className="finish-step">
      <div className="finish-hero">
        <span className="finish-icon">🎉</span>
        <h1>配置完成！</h1>
        <p className="text-muted">
          {config.claw_name} 已准备就绪，可以开始使用了
        </p>
      </div>

      {/* Summary */}
      <div className="finish-summary">
        <div className="finish-summary-title">配置摘要</div>
        {[
          ["服务器", config.server_url],
          ["Claw 名称", config.claw_name],
          ["角色定位", config.claw_role],
        ].map(([k, v]) => (
          <div key={k} className="finish-summary-row">
            <span className="text-muted">{k}：</span>
            <span>{v || "（未设置）"}</span>
          </div>
        ))}
        {config.personality_traits.length > 0 && (
          <div className="finish-summary-row">
            <span className="text-muted">性格特征：</span>
            <span>{config.personality_traits.join("、")}</span>
          </div>
        )}
      </div>

      <div className="finish-actions">
        <button className="btn-primary btn-lg" onClick={onFinish}>
          开始使用 QuickClaw 🚀
        </button>
        <button className="btn-ghost" onClick={onBack}>
          ← 返回修改
        </button>
      </div>

      <style>{`
        .finish-step {
          display: flex; flex-direction: column; align-items: center;
          text-align: center; gap: 28px;
        }
        .finish-hero { display: flex; flex-direction: column; align-items: center; gap: 8px; }
        .finish-icon { font-size: 64px; }
        .finish-hero h1 { font-size: 2rem; }
        .finish-summary {
          background: var(--bg-card); border: 1px solid var(--border);
          border-radius: var(--radius-md); padding: 20px 24px;
          width: 100%; max-width: 440px; text-align: left;
        }
        .finish-summary-title { font-weight: 700; margin-bottom: 12px; }
        .finish-summary-row {
          display: flex; gap: 8px; font-size: 13px;
          padding: 4px 0; border-bottom: 1px solid var(--border);
        }
        .finish-summary-row:last-child { border-bottom: none; }
        .finish-actions { display: flex; flex-direction: column; align-items: center; gap: 10px; }
      `}</style>
    </div>
  );
}
