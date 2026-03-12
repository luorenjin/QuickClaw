import { useEffect, useState } from "react";
import { api } from "./hooks/api";
import { ClawConfig, defaultConfig } from "./types";
import Wizard from "./components/wizard/Wizard";
import AppShell, { StudioPage } from "./components/shell/AppShell";
import StudioProvider from "./store/StudioProvider";
import Dashboard from "./pages/Dashboard";
import BriefCEO from "./pages/BriefCEO";
import CEOWorkspace from "./pages/CEOWorkspace";
import Workspaces from "./pages/Workspaces";
import WorkspaceDetail from "./pages/WorkspaceDetail";
import MissionFeed from "./pages/MissionFeed";
import SettingsPage from "./pages/SettingsPage";
import "./App.css";
import "./components/shell/AppShell.css";

type AppScreen = "wizard" | "studio";

export default function App() {
  const [config, setConfig] = useState<ClawConfig>(defaultConfig());
  const [screen, setScreen] = useState<AppScreen>("wizard");
  const [loading, setLoading] = useState(true);
  const [studioPage, setStudioPage] = useState<StudioPage>("dashboard");
  const [selectedWorkspaceId, setSelectedWorkspaceId] = useState<string | undefined>();

  // Load config on mount
  useEffect(() => {
    api
      .loadConfig()
      .then((cfg) => {
        setConfig(cfg);
        setScreen(cfg.configured ? "studio" : "wizard");
      })
      .catch(() => setScreen("wizard"))
      .finally(() => setLoading(false));
  }, []);

  const handleWizardComplete = (cfg: ClawConfig) => {
    setConfig(cfg);
    setScreen("studio");
  };

  const handleSaveConfig = async (cfg: ClawConfig) => {
    await api.saveConfig(cfg);
    setConfig(cfg);
  };

  const handleNavigate = (page: StudioPage, extra?: string) => {
    setStudioPage(page);
    if (page === "workspace-detail" && extra) {
      setSelectedWorkspaceId(extra);
    }
  };

  if (loading) {
    return (
      <div className="app-loading">
        <span className="app-loading-icon">🦀</span>
        <p>加载中...</p>
      </div>
    );
  }

  if (screen === "wizard") {
    return (
      <div className="app">
        <main className="app-main">
          <Wizard initialConfig={config} onComplete={handleWizardComplete} />
        </main>
      </div>
    );
  }

  return (
    <StudioProvider>
      <AppShell
        page={studioPage}
        onNavigate={handleNavigate}
        selectedWorkspaceId={selectedWorkspaceId}
      >
        {studioPage === "dashboard" && <Dashboard onNavigate={handleNavigate} />}
        {studioPage === "brief" && <BriefCEO onNavigate={handleNavigate} />}
        {studioPage === "ceo-workspace" && <CEOWorkspace onNavigate={handleNavigate} />}
        {studioPage === "workspaces" && <Workspaces onNavigate={handleNavigate} />}
        {studioPage === "workspace-detail" && selectedWorkspaceId && (
          <WorkspaceDetail workspaceId={selectedWorkspaceId} onNavigate={handleNavigate} />
        )}
        {studioPage === "mission-feed" && <MissionFeed />}
        {studioPage === "settings" && (
          <SettingsPage
            config={config}
            onSave={handleSaveConfig}
            onRerunWizard={() => {
              setConfig({ ...defaultConfig(), configured: false });
              setScreen("wizard");
            }}
          />
        )}
      </AppShell>
    </StudioProvider>
  );
}
