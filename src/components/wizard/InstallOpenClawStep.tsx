import { useEffect, useRef, useState } from "react";
import { api } from "../../hooks/api";
import { ClawConfig } from "../../types";

interface Props {
  config: ClawConfig;
  onConfigChange: (c: ClawConfig) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function InstallOpenClawStep({
  config,
  onConfigChange,
  onNext,
  onBack,
}: Props) {
  const [installDir, setInstallDir] = useState("");
  const [npmPackage, setNpmPackage] = useState("");
  const [ready, setReady] = useState(false);
  const [running, setRunning] = useState(false);
  const [log, setLog] = useState<string[]>([]);
  const [finished, setFinished] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const logRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Load defaults from backend
    api.defaultOpenclawConfig().then((cfg) => {
      setInstallDir(cfg.install_dir);
      setNpmPackage(cfg.npm_package);
      // Check if already installed
      api.detectOpenclaw(cfg.install_dir).then(setReady);
    });
  }, []);

  // Auto-scroll log to bottom
  useEffect(() => {
    logRef.current?.scrollTo(0, logRef.current.scrollHeight);
  }, [log]);

  const startInstall = async () => {
    setRunning(true);
    setLog([]);
    setFinished(false);
    setError(null);
    try {
      await api.installOpenclaw(installDir, npmPackage);
      // Poll for progress
      const poll = setInterval(async () => {
        const progress = await api.getOpenclawProgress();
        setLog(progress.log);
        if (progress.finished) {
          clearInterval(poll);
          setRunning(false);
          setFinished(true);
          if (progress.error) {
            setError(progress.error);
          } else {
            setReady(true);
            // Auto-fill server URL
            onConfigChange({ ...config, server_url: "http://localhost:8080" });
          }
        }
      }, 500);
    } catch (err) {
      setRunning(false);
      setError(String(err));
    }
  };

  const retry = () => {
    setFinished(false);
    setError(null);
    setLog([]);
  };

  return (
    <div>
      <h2 className="step-title">第二步：安装 OpenClaw 本地服务</h2>
      <p className="step-desc">
        OpenClaw 是驱动 Claw 智能能力的本地服务内核。QuickClaw
        将通过 npm 自动安装 OpenClaw 到您的计算机。
      </p>

      {/* Already installed banner */}
      {ready && !running && (
        <div className="install-banner install-banner-ok">
          <span className="install-banner-icon">✓</span>
          <div>
            <strong>OpenClaw 已安装</strong>
            <div className="text-muted text-sm">安装目录：{installDir}</div>
          </div>
          <button
            className="btn-ghost"
            style={{ marginLeft: "auto" }}
            onClick={() => { setReady(false); setFinished(false); setLog([]); }}
          >
            重新安装 / 更新
          </button>
        </div>
      )}

      {/* Install form */}
      {!ready && !running && !finished && (
        <>
          <div className="step-field">
            <label>安装目录</label>
            <input
              value={installDir}
              onChange={(e) => setInstallDir(e.target.value)}
              placeholder="~/.quickclaw"
            />
            <span className="hint">OpenClaw 将被安装至该目录下的 openclaw/ 子目录</span>
          </div>
          <div className="step-field">
            <label>npm 包名</label>
            <input
              value={npmPackage}
              onChange={(e) => setNpmPackage(e.target.value)}
              placeholder="openclaw"
            />
            <span className="hint">OpenClaw 的 npm 包名称，通常为 "openclaw"</span>
          </div>
          <button className="btn-primary" onClick={startInstall}>
            🚀 开始安装 OpenClaw
          </button>
        </>
      )}

      {/* Running */}
      {running && (
        <div className="install-running">
          <span className="spinner" /> <span className="text-muted">安装中，请稍候...</span>
        </div>
      )}

      {/* Success */}
      {finished && !error && (
        <div className="install-banner install-banner-ok">
          <span className="install-banner-icon">✓</span>
          <strong>OpenClaw 安装成功！</strong>
        </div>
      )}

      {/* Error */}
      {finished && error && (
        <div className="install-banner install-banner-err">
          <span className="install-banner-icon">✗</span>
          <div>
            <strong>安装失败</strong>
            <div className="text-sm">{error}</div>
          </div>
          <button className="btn-ghost" style={{ marginLeft: "auto" }} onClick={retry}>
            重试
          </button>
        </div>
      )}

      {/* Log output */}
      {log.length > 0 && (
        <div className="install-log" ref={logRef}>
          {log.map((line, i) => (
            <div key={i}>{line}</div>
          ))}
        </div>
      )}

      <div className="step-nav">
        <button className="btn-ghost" onClick={onBack} disabled={running}>
          ← 上一步
        </button>
        <div className="step-nav-right">
          {!running && !ready && (
            <button className="btn-ghost" onClick={onNext}>
              跳过（已有自托管服务）
            </button>
          )}
          <button
            className="btn-primary"
            onClick={onNext}
            disabled={running || (!ready && !finished && log.length === 0)}
          >
            下一步 →
          </button>
        </div>
      </div>

      <style>{`
        .install-banner {
          display: flex; align-items: center; gap: 12px;
          border-radius: var(--radius-md); padding: 14px 16px;
          margin-bottom: 16px;
        }
        .install-banner-ok  { background: rgba(0,200,100,0.1); border: 1px solid rgba(0,200,100,0.3); }
        .install-banner-err { background: var(--danger-dim); border: 1px solid rgba(224,82,82,0.3); }
        .install-banner-icon { font-size: 20px; }
        .install-running { display: flex; align-items: center; gap: 8px; margin-bottom: 16px; }
        .install-log {
          background: var(--bg-code); border-radius: var(--radius-sm);
          padding: 10px 12px; font-family: var(--font-mono); font-size: 11.5px;
          color: #b0d0b0; max-height: 180px; overflow-y: auto;
          margin-bottom: 16px; line-height: 1.5;
        }
        .spinner {
          display: inline-block; width: 16px; height: 16px;
          border: 2px solid var(--accent); border-top-color: transparent;
          border-radius: 50%; animation: spin 0.7s linear infinite; vertical-align: middle;
        }
        @keyframes spin { to { transform: rotate(360deg); } }
      `}</style>
    </div>
  );
}
