import { ClawConfig } from "../../types";

interface Props {
  config: ClawConfig;
  onConfigChange: (c: ClawConfig) => void;
  onNext: () => void;
  onBack: () => void;
}

const PRESET_TRAITS = [
  "友善", "专业", "幽默", "耐心", "简洁", "详细",
  "创意", "严谨", "温柔", "活泼", "博学", "务实",
];

function traitsToString(traits: string[]): string {
  return traits.join("、");
}

function parseTraits(input: string): string[] {
  return input
    .split(/[,，、;；]/)
    .map((s) => s.trim())
    .filter(Boolean);
}

export default function PersonalityStep({
  config,
  onConfigChange,
  onNext,
  onBack,
}: Props) {
  const traitsStr = traitsToString(config.personality_traits);

  const toggleTrait = (trait: string) => {
    const current = new Set(config.personality_traits);
    if (current.has(trait)) {
      current.delete(trait);
    } else {
      current.add(trait);
    }
    onConfigChange({ ...config, personality_traits: [...current] });
  };

  const handleTraitsInput = (val: string) => {
    onConfigChange({ ...config, personality_traits: parseTraits(val) });
  };

  return (
    <div>
      <h2 className="step-title">第五步：定义 Claw 性格</h2>
      <p className="step-desc">
        定义 Claw 的性格特征，让它更符合您的使用习惯和期望。
      </p>

      <div className="step-field">
        <label>常用性格特征（点击切换）</label>
        <div className="trait-chips">
          {PRESET_TRAITS.map((t) => (
            <button
              key={t}
              className={`trait-chip ${config.personality_traits.includes(t) ? "active" : ""}`}
              onClick={() => toggleTrait(t)}
            >
              {t}
            </button>
          ))}
        </div>
      </div>

      <div className="step-field">
        <label>自定义特征（逗号 / 顿号分隔）</label>
        <input
          value={traitsStr}
          onChange={(e) => handleTraitsInput(e.target.value)}
          placeholder="例如: 友善、专业、幽默"
        />
      </div>

      <div className="step-field">
        <label>系统提示词</label>
        <textarea
          rows={6}
          value={config.system_prompt}
          onChange={(e) =>
            onConfigChange({ ...config, system_prompt: e.target.value })
          }
          placeholder="描述 Claw 的性格和行为准则..."
        />
        <span className="hint">
          系统提示词会在每次对话开始时发送给 Claw，定义其行为方式
        </span>
      </div>

      <div className="step-nav">
        <button className="btn-ghost" onClick={onBack}>
          ← 上一步
        </button>
        <button className="btn-primary" onClick={onNext}>
          完成配置 →
        </button>
      </div>

      <style>{`
        .trait-chips { display: flex; flex-wrap: wrap; gap: 8px; }
        .trait-chip {
          background: var(--bg-card); border: 1px solid var(--border);
          border-radius: 20px; color: var(--text-secondary);
          cursor: pointer; font-size: 13px; padding: 4px 14px;
          transition: all 0.15s;
        }
        .trait-chip:hover { border-color: var(--accent); color: var(--text-primary); }
        .trait-chip.active {
          background: var(--accent); border-color: var(--accent);
          color: #fff;
        }
      `}</style>
    </div>
  );
}
