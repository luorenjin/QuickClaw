import { ClawConfig } from "../../types";

interface Props {
  config: ClawConfig;
  onConfigChange: (c: ClawConfig) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function IdentityStep({
  config,
  onConfigChange,
  onNext,
  onBack,
}: Props) {
  const canProceed = config.claw_name.trim().length > 0;

  return (
    <div>
      <h2 className="step-title">第四步：定义 Claw 身份</h2>
      <p className="step-desc">
        为您的 Claw 助手定义一个独特的身份，包括名字和角色定位。
      </p>

      <div className="step-field">
        <label>Claw 名称</label>
        <input
          value={config.claw_name}
          onChange={(e) =>
            onConfigChange({ ...config, claw_name: e.target.value })
          }
          placeholder="例如: 小助、ARIA、Max..."
          autoFocus
        />
        <span className="hint">给您的 AI 助手一个专属名字</span>
      </div>

      <div className="step-field">
        <label>角色定位</label>
        <input
          value={config.claw_role}
          onChange={(e) =>
            onConfigChange({ ...config, claw_role: e.target.value })
          }
          placeholder="例如: 工作助手、编程专家、知识百科..."
        />
        <span className="hint">描述 Claw 的主要职责和专长领域</span>
      </div>

      {/* Preview */}
      <div className="identity-preview">
        <div className="identity-preview-label text-muted text-sm">预览</div>
        <div className="identity-preview-card">
          <span className="identity-preview-avatar">🦀</span>
          <div>
            <div className="identity-preview-name">
              {config.claw_name || "Claw"}{" "}
              <span className="text-muted text-sm">
                {config.claw_role ? `· ${config.claw_role}` : ""}
              </span>
            </div>
            <div className="text-muted text-sm">你好！有什么我可以帮助你的吗？</div>
          </div>
        </div>
      </div>

      <div className="step-nav">
        <button className="btn-ghost" onClick={onBack}>
          ← 上一步
        </button>
        <button
          className="btn-primary"
          onClick={onNext}
          disabled={!canProceed}
        >
          下一步 →
        </button>
      </div>

      <style>{`
        .identity-preview { margin: 20px 0; }
        .identity-preview-label { margin-bottom: 6px; }
        .identity-preview-card {
          display: flex; align-items: flex-start; gap: 12px;
          background: var(--bg-card); border: 1px solid var(--border);
          border-radius: var(--radius-md); padding: 14px 16px;
        }
        .identity-preview-avatar { font-size: 28px; }
        .identity-preview-name { font-weight: 600; }
      `}</style>
    </div>
  );
}
