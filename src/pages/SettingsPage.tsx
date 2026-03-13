import Settings from "../components/settings/Settings";
import { ClawConfig } from "../types";

interface SettingsPageProps {
  config: ClawConfig;
  onSave: (config: ClawConfig) => Promise<void>;
  onRerunWizard: () => void;
}

export default function SettingsPage({ config, onSave, onRerunWizard }: SettingsPageProps) {
  return (
    <div className="page" style={{ maxWidth: 700 }}>
      <div className="page-header">
        <h1 className="page-title">⚙ 设置</h1>
        <p className="page-subtitle">服务器连接与 Claw 配置</p>
      </div>
      <Settings config={config} onSave={onSave} onRerunWizard={onRerunWizard} />
    </div>
  );
}
