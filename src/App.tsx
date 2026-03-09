import { useEffect, useState } from "react";
import { api } from "./hooks/api";
import { ClawConfig, defaultConfig, WizardStep } from "./types";
import Wizard from "./components/wizard/Wizard";
import ChatView from "./components/chat/ChatView";
import Settings from "./components/settings/Settings";
import "./App.css";

type Screen = "wizard" | "chat" | "settings";

export default function App() {
  const [config, setConfig] = useState<ClawConfig>(defaultConfig());
  const [screen, setScreen] = useState<Screen>("wizard");
  const [loading, setLoading] = useState(true);

  // Load config on mount
  useEffect(() => {
    api
      .loadConfig()
      .then((cfg) => {
        setConfig(cfg);
        setScreen(cfg.configured ? "chat" : "wizard");
      })
      .catch(() => setScreen("wizard"))
      .finally(() => setLoading(false));
  }, []);

  const handleWizardComplete = (cfg: ClawConfig) => {
    setConfig(cfg);
    setScreen("chat");
  };

  const handleSaveConfig = async (cfg: ClawConfig) => {
    await api.saveConfig(cfg);
    setConfig(cfg);
  };

  if (loading) {
    return (
      <div className="app-loading">
        <span className="app-loading-icon">🦀</span>
        <p>加载中...</p>
      </div>
    );
  }

  return (
    <div className="app">
      {screen !== "wizard" && (
        <nav className="app-nav">
          <div className="app-nav-brand">🦀 QuickClaw</div>
          <div className="app-nav-links">
            <button
              className={`app-nav-btn ${screen === "chat" ? "active" : ""}`}
              onClick={() => setScreen("chat")}
            >
              💬 对话
            </button>
            <button
              className={`app-nav-btn ${screen === "settings" ? "active" : ""}`}
              onClick={() => {
                setScreen("settings");
              }}
            >
              ⚙ 设置
            </button>
          </div>
        </nav>
      )}

      <main className="app-main">
        {screen === "wizard" && (
          <Wizard
            initialConfig={config}
            onComplete={handleWizardComplete}
          />
        )}
        {screen === "chat" && (
          <ChatView config={config} />
        )}
        {screen === "settings" && (
          <Settings
            config={config}
            onSave={handleSaveConfig}
            onRerunWizard={() => {
              setConfig({ ...defaultConfig(), configured: false });
              setScreen("wizard");
            }}
          />
        )}
      </main>
    </div>
  );
}
