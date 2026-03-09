import { useState } from "react";
import { api } from "../../hooks/api";
import { DepInfo } from "../../types";

interface Props {
  onNext: () => void;
  onBack: () => void;
}

const STATUS_ICON: Record<string, string> = {
  unknown: "◌",
  installed: "✓",
  missing: "✗",
  installing: "⟳",
  failed: "!",
};

const STATUS_COLOR: Record<string, string> = {
  unknown: "var(--text-muted)",
  installed: "var(--success)",
  missing: "var(--danger)",
  installing: "var(--accent)",
  failed: "var(--warning)",
};

export default function EnvCheckStep({ onNext, onBack }: Props) {
  const [deps, setDeps] = useState<DepInfo[]>([]);
  const [checked, setChecked] = useState(false);
  const [installing, setInstalling] = useState<Set<string>>(new Set());
  const [logs, setLogs] = useState<Record<string, string[]>>({});

  const checkAll = async () => {
    const result = await api.checkDependencies();
    setDeps(result);
    setChecked(true);
  };

  const installDep = async (name: string) => {
    setInstalling((prev) => new Set(prev).add(name));
    setDeps((prev) =>
      prev.map((d) => (d.name === name ? { ...d, status: "installing" } : d))
    );

    try {
      await api.installDependency(name);
      // Poll for completion
      const poll = setInterval(async () => {
        const progress = await api.getDepProgress(name);
        if (progress.log.length > 0) {
          setLogs((prev) => ({ ...prev, [name]: progress.log }));
        }
        if (progress.finished) {
          clearInterval(poll);
          setInstalling((prev) => {
            const next = new Set(prev);
            next.delete(name);
            return next;
          });
          // Re-check this dependency
          const info = await api.checkSingleDependency(name);
          setDeps((prev) => prev.map((d) => (d.name === name ? info : d)));
        }
      }, 500);
    } catch (err) {
      setDeps((prev) =>
        prev.map((d) =>
          d.name === name
            ? { ...d, status: "failed", version: String(err) }
            : d
        )
      );
      setInstalling((prev) => {
        const next = new Set(prev);
        next.delete(name);
        return next;
      });
    }
  };

  const installAllMissing = () => {
    deps
      .filter((d) => d.required && d.status === "missing")
      .forEach((d) => installDep(d.name));
  };

  const installAllOptional = () => {
    deps
      .filter((d) => !d.required && d.status === "missing")
      .forEach((d) => installDep(d.name));
  };

  const requiredOk = deps
    .filter((d) => d.required)
    .every((d) => d.status === "installed");

  const missingRequired = deps.filter(
    (d) => d.required && d.status === "missing"
  );
  const missingOptional = deps.filter(
    (d) => !d.required && d.status === "missing"
  );
  const anyInstalling = installing.size > 0;

  const canProceed = !anyInstalling && (!checked || requiredOk);

  return (
    <div>
      <h2 className="step-title">第一步：系统环境检查</h2>
      <p className="step-desc">
        QuickClaw 需要以下组件才能运行 OpenClaw。点击「检查环境」扫描您的系统，
        缺失的组件可以由 QuickClaw 自动安装。
      </p>

      {/* Dep list */}
      {deps.length > 0 && (
        <div className="dep-list">
          {deps.map((dep) => (
            <div key={dep.name} className="dep-card">
              <div className="dep-card-left">
                <span
                  className="dep-status-icon"
                  style={{ color: STATUS_COLOR[dep.status] }}
                >
                  {dep.status === "installing" ? (
                    <span className="spinner" />
                  ) : (
                    STATUS_ICON[dep.status]
                  )}
                </span>
                <div className="dep-info">
                  <div className="dep-header">
                    <strong>{dep.name}</strong>
                    {!dep.required && (
                      <span className="dep-badge">可选插件</span>
                    )}
                    <span
                      className="dep-status-text"
                      style={{ color: STATUS_COLOR[dep.status] }}
                    >
                      {dep.status === "installed"
                        ? dep.version
                        : dep.status === "failed"
                        ? `安装失败`
                        : dep.status === "installing"
                        ? "安装中..."
                        : dep.status === "missing"
                        ? "未安装"
                        : "未检测"}
                    </span>
                  </div>
                  <div className="dep-desc text-muted text-sm">
                    {dep.description}
                  </div>
                  {/* Log tail */}
                  {(dep.status === "installing" ||
                    dep.status === "failed") &&
                    logs[dep.name] && (
                      <pre className="dep-log">
                        {logs[dep.name].slice(-4).join("\n")}
                      </pre>
                    )}
                </div>
              </div>
              {dep.status === "missing" && !installing.has(dep.name) && (
                <button
                  className="btn-secondary"
                  style={{ fontSize: 12, padding: "4px 10px" }}
                  onClick={() => installDep(dep.name)}
                >
                  安装
                </button>
              )}
            </div>
          ))}
        </div>
      )}

      {/* Action row */}
      <div className="env-actions">
        <button
          className="btn-secondary"
          onClick={checkAll}
          disabled={anyInstalling}
        >
          🔍 检查环境
        </button>
        {checked && missingRequired.length > 0 && (
          <button
            className="btn-danger"
            onClick={installAllMissing}
            disabled={anyInstalling}
          >
            🚀 自动安装缺失必需组件 ({missingRequired.length})
          </button>
        )}
        {checked && missingOptional.length > 0 && (
          <button
            className="btn-secondary"
            onClick={installAllOptional}
            disabled={anyInstalling}
          >
            📦 安装可选插件 ({missingOptional.length})
          </button>
        )}
      </div>

      {checked && !requiredOk && !anyInstalling && (
        <p className="env-warning">
          ⚠ 请先安装所有必需组件，或跳过继续（不推荐）。
        </p>
      )}

      <div className="step-nav">
        <button className="btn-ghost" onClick={onBack}>
          ← 上一步
        </button>
        <div className="step-nav-right">
          {checked && !requiredOk && !anyInstalling && (
            <button className="btn-ghost" onClick={onNext}>
              跳过检查 →
            </button>
          )}
          <button
            className="btn-primary"
            onClick={onNext}
            disabled={!canProceed}
          >
            下一步 →
          </button>
        </div>
      </div>

      <style>{`
        .dep-list { display: flex; flex-direction: column; gap: 8px; margin-bottom: 20px; }
        .dep-card {
          display: flex; align-items: center; justify-content: space-between;
          background: var(--bg-card); border: 1px solid var(--border);
          border-radius: var(--radius-md); padding: 12px 16px;
        }
        .dep-card-left { display: flex; align-items: flex-start; gap: 12px; flex: 1; }
        .dep-status-icon { font-size: 18px; min-width: 22px; text-align: center; margin-top: 2px; }
        .dep-info { flex: 1; }
        .dep-header { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
        .dep-badge {
          font-size: 11px; padding: 1px 6px;
          background: rgba(100,150,200,0.15); color: #6496c8;
          border-radius: 10px;
        }
        .dep-status-text { margin-left: auto; font-size: 12px; }
        .dep-desc { margin-top: 2px; }
        .dep-log {
          margin-top: 6px; font-size: 11px; font-family: var(--font-mono);
          background: var(--bg-code); border-radius: 4px; padding: 6px 8px;
          color: #a0c0a0; white-space: pre-wrap; word-break: break-all;
          max-height: 80px; overflow-y: auto;
        }
        .env-actions { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 16px; }
        .env-warning {
          font-size: 12px; color: var(--warning);
          background: rgba(245,166,35,0.1); border-radius: var(--radius-sm);
          padding: 8px 12px; margin-bottom: 16px;
        }
        .spinner {
          display: inline-block; width: 14px; height: 14px;
          border: 2px solid var(--accent); border-top-color: transparent;
          border-radius: 50%; animation: spin 0.7s linear infinite;
          vertical-align: middle;
        }
        @keyframes spin { to { transform: rotate(360deg); } }
      `}</style>
    </div>
  );
}
