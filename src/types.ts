// ─── Studio Domain Models ─────────────────────────────────────────────────────

export type EmployeeType = "human" | "digital";
export type EmployeeKind = "individual_contributor" | "manager";
export type EmployeeStatus = "active" | "inactive" | "busy" | "idle";

export interface ModelProfile {
  id: string;
  name: string;
  provider: string;
  model: string;
  temperature?: number;
}

export interface Employee {
  id: string;
  organizationId: string;
  type: EmployeeType;
  kind: EmployeeKind;
  name: string;
  title: string;
  description: string;
  status: EmployeeStatus;
  workspaceId: string;
  reportsToEmployeeId?: string;
  modelProfileId?: string;
  avatar?: string;
}

export type WorkspaceStatus = "active" | "idle" | "busy";

export interface Workspace {
  id: string;
  organizationId: string;
  employeeId: string;
  name: string;
  status: WorkspaceStatus;
  createdAt: string;
}

export type MissionStatus = "draft" | "active" | "completed" | "paused";

export interface Mission {
  id: string;
  title: string;
  brief: string;
  status: MissionStatus;
  createdById: string;
  createdAt: string;
  updatedAt: string;
}

export type TaskStatus = "pending" | "in_progress" | "completed" | "blocked";

export interface Task {
  id: string;
  missionId: string;
  workspaceId: string;
  assignedToEmployeeId: string;
  title: string;
  description: string;
  status: TaskStatus;
  createdAt: string;
}

export type ArtifactVisibility = "private" | "shared" | "mission-shared";

export interface Artifact {
  id: string;
  workspaceId: string;
  taskId?: string;
  name: string;
  type: string;
  content: string;
  visibility: ArtifactVisibility;
  createdAt: string;
}

// ─── Message Protocol ─────────────────────────────────────────────────────────

export type MessageChannel = "human_ceo" | "mission" | "workspace" | "handoff";

export type MessageType =
  | "GoalSubmitted"
  | "TaskAssigned"
  | "ProgressReported"
  | "ArtifactPublished"
  | "HandoffRequested"
  | "TaskCompleted"
  | "SummaryReported";

export interface StudioMessage {
  id: string;
  channel: MessageChannel;
  type: MessageType;
  fromEmployeeId: string;
  toEmployeeId?: string;
  missionId?: string;
  workspaceId?: string;
  payload: Record<string, unknown>;
  timestamp: string;
}

export interface Organization {
  id: string;
  name: string;
  mode: string;
  ownerId: string;
  createdAt: string;
}

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
