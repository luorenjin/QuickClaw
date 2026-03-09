import { useState } from "react";
import { api } from "../../hooks/api";
import { ClawConfig } from "../../types";

interface Props {
  config: ClawConfig;
  onConfigChange: (c: ClawConfig) => void;
  onNext: () => void;
  onBack: () => void;
}

export default function ServerConfigStep({
  config,
  onConfigChange,
  onNext,
  onBack,
}: Props) {
  const [connStatus, setConnStatus] = useState<{
    ok: boolean;
    msg: string;
  } | null>(null);
  const [testing, setTesting] = useState(false);

  const testConn = async () => {
    setTesting(true);
    setConnStatus(null);
    try {
      const msg = await api.testConnection(config.server_url, config.api_key);
      setConnStatus({ ok: true, msg });
    } catch (err) {
      setConnStatus({ ok: false, msg: String(err) });
    } finally {
      setTesting(false);
    }
  };

  return (
    <div>
      <h2 className="step-title">第三步：配置 OpenClaw 服务器</h2>
      <p className="step-desc">
        确认 OpenClaw 服务器地址。如果您使用了 QuickClaw 自动安装，地址已自动填好。
      </p>

      <div className="step-field">
        <label>服务器地址</label>
        <input
          value={config.server_url}
          onChange={(e) =>
            onConfigChange({ ...config, server_url: e.target.value })
          }
          placeholder="http://localhost:8080"
        />
        <span className="hint">支持 http:// 和 https:// 协议</span>
      </div>

      <div className="step-field">
        <label>API 密钥（可选）</label>
        <input
          type="password"
          value={config.api_key}
          onChange={(e) =>
            onConfigChange({ ...config, api_key: e.target.value })
          }
          placeholder="如果服务器需要认证，请输入 API 密钥"
        />
        <span className="hint">如果您的 OpenClaw 服务器不需要认证，可以留空</span>
      </div>

      <div className="server-test-row">
        <button
          className="btn-secondary"
          onClick={testConn}
          disabled={testing || !config.server_url}
        >
          {testing ? <><span className="spinner" /> 测试中...</> : "测试连接"}
        </button>
        {connStatus && (
          <span
            className="server-test-result"
            style={{ color: connStatus.ok ? "var(--success)" : "var(--danger)" }}
          >
            {connStatus.ok ? "✓" : "✗"} {connStatus.msg}
          </span>
        )}
      </div>

      <div className="step-nav">
        <button className="btn-ghost" onClick={onBack}>
          ← 上一步
        </button>
        <button
          className="btn-primary"
          onClick={onNext}
          disabled={!config.server_url}
        >
          下一步 →
        </button>
      </div>

      <style>{`
        .server-test-row { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
        .server-test-result { font-size: 13px; }
        .spinner {
          display: inline-block; width: 13px; height: 13px;
          border: 2px solid currentColor; border-top-color: transparent;
          border-radius: 50%; animation: spin 0.7s linear infinite; vertical-align: middle;
        }
        @keyframes spin { to { transform: rotate(360deg); } }
      `}</style>
    </div>
  );
}
