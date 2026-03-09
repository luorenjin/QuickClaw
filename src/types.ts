// ─── Config ───────────────────────────────────────────────────────────────────

export interface ClawConfig {
  server_url: string;
  api_key: string;
  claw_name: string;
  claw_role: string;
  personality_traits: string[];
  system_prompt: string;
  configured: boolean;
}

export const defaultConfig = (): ClawConfig => ({
  server_url: "http://localhost:8080",
  api_key: "",
  claw_name: "Claw",
  claw_role: "智能助手",
  personality_traits: ["友善", "专业", "耐心"],
  system_prompt:
    "你是一个友善、专业、耐心的智能助手。你会用简洁清晰的语言回答用户的问题，提供准确有用的信息，并在需要时给出详细解释。",
  configured: false,
});

// ─── Dependencies ─────────────────────────────────────────────────────────────

export type DepStatusKey =
  | "unknown"
  | "installed"
  | "missing"
  | "installing"
  | "failed";

export interface DepInfo {
  name: string;
  description: string;
  required: boolean;
  status: DepStatusKey;
  version?: string;
}

// ─── Install progress ─────────────────────────────────────────────────────────

export interface ProgressSnapshot {
  current_step: string;
  log: string[];
  finished: boolean;
  error?: string;
}

// ─── Chat ─────────────────────────────────────────────────────────────────────

export type MessageRole = "user" | "assistant" | "system";

export interface ChatMessage {
  role: MessageRole;
  content: string;
  id: string;
}

// ─── Wizard steps ─────────────────────────────────────────────────────────────

export type WizardStep =
  | "welcome"
  | "env-check"
  | "install-openclaw"
  | "server-config"
  | "identity"
  | "personality"
  | "finish";

export const WIZARD_STEPS: WizardStep[] = [
  "welcome",
  "env-check",
  "install-openclaw",
  "server-config",
  "identity",
  "personality",
  "finish",
];

export const PROGRESS_STEPS: WizardStep[] = [
  "env-check",
  "install-openclaw",
  "server-config",
  "identity",
  "personality",
];

export const STEP_LABELS: Record<WizardStep, string> = {
  welcome: "欢迎",
  "env-check": "环境检查",
  "install-openclaw": "安装 OpenClaw",
  "server-config": "服务器配置",
  identity: "身份定义",
  personality: "性格定义",
  finish: "完成",
};
