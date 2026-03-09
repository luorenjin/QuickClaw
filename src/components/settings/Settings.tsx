import { useState } from "react";
import { api } from "../../hooks/api";
import { ClawConfig } from "../../types";
import "./Settings.css";

const PRESET_TRAITS = [
  "友善", "专业", "幽默", "耐心", "简洁", "详细",
  "创意", "严谨", "温柔", "活泼", "博学", "务实",
];

interface Props {
  config: ClawConfig;
  onSave: (config: ClawConfig) => Promise<void>;
  onRerunWizard: () => void;
}

export default function Settings({ config, onSave, onRerunWizard }: Props) {
  const [draft, setDraft] = useState<ClawConfig>({ ...config });
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [testStatus, setTestStatus] = useState<{ ok: boolean; msg: string } | null>(null);
  const [testing, setTesting] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      await onSave(draft);
      setSaved(true);
      setTimeout(() => setSaved(false), 2500);
    } finally {
      setSaving(false);
    }
  };

  const testConn = async () => {
    setTesting(true);
    setTestStatus(null);
    try {
      const msg = await api.testConnection(draft.server_url, draft.api_key);
      setTestStatus({ ok: true, msg });
    } catch (err) {
      setTestStatus({ ok: false, msg: String(err) });
    } finally {
      setTesting(false);
    }
  };

  const traitsStr = draft.personality_traits.join("、");
  const parseTraits = (v: string) =>
    v.split(/[,，、;；]/).map((s) => s.trim()).filter(Boolean);

  const toggleTrait = (t: string) => {
    const s = new Set(draft.personality_traits);
    s.has(t) ? s.delete(t) : s.add(t);
    setDraft({ ...draft, personality_traits: [...s] });
  };

  return (
    <div className="settings">
      <div className="settings-header">
        <h2>⚙ 设置</h2>
        <div className="settings-header-actions">
          <button className="btn-ghost" onClick={onRerunWizard}>
            重新运行向导
          </button>
          <button
            className="btn-primary"
            onClick={handleSave}
            disabled={saving}
          >
            {saving ? "保存中..." : saved ? "✓ 已保存" : "保存设置"}
          </button>
        </div>
      </div>

      <div className="settings-body">
        {/* Server */}
        <section className="settings-section">
          <h3>服务器配置</h3>
          <div className="step-field">
            <label>服务器地址</label>
            <input
              value={draft.server_url}
              onChange={(e) => setDraft({ ...draft, server_url: e.target.value })}
              placeholder="http://localhost:8080"
            />
          </div>
          <div className="step-field">
            <label>API 密钥（可选）</label>
            <input
              type="password"
              value={draft.api_key}
              onChange={(e) => setDraft({ ...draft, api_key: e.target.value })}
              placeholder="如不需要认证，留空即可"
            />
          </div>
          <div className="settings-test-row">
            <button className="btn-secondary" onClick={testConn} disabled={testing}>
              {testing ? "测试中..." : "测试连接"}
            </button>
            {testStatus && (
              <span style={{ color: testStatus.ok ? "var(--success)" : "var(--danger)", fontSize: 13 }}>
                {testStatus.ok ? "✓" : "✗"} {testStatus.msg}
              </span>
            )}
          </div>
        </section>

        {/* Identity */}
        <section className="settings-section">
          <h3>Claw 身份</h3>
          <div className="step-field">
            <label>名称</label>
            <input
              value={draft.claw_name}
              onChange={(e) => setDraft({ ...draft, claw_name: e.target.value })}
              placeholder="Claw"
            />
          </div>
          <div className="step-field">
            <label>角色定位</label>
            <input
              value={draft.claw_role}
              onChange={(e) => setDraft({ ...draft, claw_role: e.target.value })}
              placeholder="AI 助手"
            />
          </div>
        </section>

        {/* Personality */}
        <section className="settings-section">
          <h3>Claw 性格</h3>
          <div className="step-field">
            <label>性格特征</label>
            <div className="trait-chips">
              {PRESET_TRAITS.map((t) => (
                <button
                  key={t}
                  className={`trait-chip ${draft.personality_traits.includes(t) ? "active" : ""}`}
                  onClick={() => toggleTrait(t)}
                >
                  {t}
                </button>
              ))}
            </div>
            <input
              style={{ marginTop: 8 }}
              value={traitsStr}
              onChange={(e) =>
                setDraft({ ...draft, personality_traits: parseTraits(e.target.value) })
              }
              placeholder="友善、专业、幽默"
            />
          </div>
          <div className="step-field">
            <label>系统提示词</label>
            <textarea
              rows={5}
              value={draft.system_prompt}
              onChange={(e) => setDraft({ ...draft, system_prompt: e.target.value })}
              placeholder="描述 Claw 的性格和行为准则..."
            />
          </div>
        </section>
      </div>
    </div>
  );
}
