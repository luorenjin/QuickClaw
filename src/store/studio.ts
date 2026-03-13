import { createContext, useContext } from "react";
import {
  Artifact,
  Employee,
  Mission,
  ModelProfile,
  Organization,
  StudioMessage,
  Task,
  Workspace,
} from "../types";

// ─── Seed / mock data ─────────────────────────────────────────────────────────

const ORG_ID = "org-01";
const NOW = new Date().toISOString();

export const DEFAULT_MODEL_PROFILES: ModelProfile[] = [
  { id: "mp-default", name: "Default", provider: "local", model: "default" },
  { id: "mp-gpt4", name: "GPT-4o", provider: "openai", model: "gpt-4o" },
  { id: "mp-claude", name: "Claude 3.5", provider: "anthropic", model: "claude-3-5-sonnet-20241022" },
];

export const DEFAULT_EMPLOYEES: Employee[] = [
  {
    id: "emp-ceo",
    organizationId: ORG_ID,
    type: "digital",
    kind: "manager",
    name: "Alex",
    title: "CEO",
    description: "数字首席执行官，统筹团队任务，制定计划，分派工作",
    status: "active",
    workspaceId: "ws-ceo",
    modelProfileId: "mp-default",
    avatar: "🤖",
  },
  {
    id: "emp-researcher",
    organizationId: ORG_ID,
    type: "digital",
    kind: "individual_contributor",
    name: "Riley",
    title: "研究员",
    description: "收集信息、分析竞品、输出研究摘要",
    status: "idle",
    workspaceId: "ws-researcher",
    reportsToEmployeeId: "emp-ceo",
    modelProfileId: "mp-default",
    avatar: "🔬",
  },
  {
    id: "emp-builder",
    organizationId: ORG_ID,
    type: "digital",
    kind: "individual_contributor",
    name: "Jordan",
    title: "工程师",
    description: "负责代码开发与技术实现",
    status: "idle",
    workspaceId: "ws-builder",
    reportsToEmployeeId: "emp-ceo",
    modelProfileId: "mp-default",
    avatar: "💻",
  },
  {
    id: "emp-marketer",
    organizationId: ORG_ID,
    type: "digital",
    kind: "individual_contributor",
    name: "Sam",
    title: "市场专员",
    description: "品牌宣传、内容创作、增长策略",
    status: "idle",
    workspaceId: "ws-marketer",
    reportsToEmployeeId: "emp-ceo",
    modelProfileId: "mp-default",
    avatar: "📣",
  },
];

export const DEFAULT_WORKSPACES: Workspace[] = DEFAULT_EMPLOYEES.map((e) => ({
  id: e.workspaceId,
  organizationId: ORG_ID,
  employeeId: e.id,
  name: `${e.name} 的工作空间`,
  status: e.status === "active" ? "active" : "idle",
  createdAt: NOW,
}));

export const DEFAULT_ORGANIZATION: Organization = {
  id: ORG_ID,
  name: "我的一人公司",
  mode: "general",
  ownerId: "human-owner",
  createdAt: NOW,
};

// ─── Studio state shape ───────────────────────────────────────────────────────

export interface StudioState {
  organization: Organization;
  employees: Employee[];
  workspaces: Workspace[];
  missions: Mission[];
  tasks: Task[];
  artifacts: Artifact[];
  messages: StudioMessage[];
  modelProfiles: ModelProfile[];
  activeMissionId: string | null;
  activeWorkspaceId: string | null;
}

export interface StudioActions {
  submitBrief: (title: string, brief: string) => Mission;
  addEmployee: (employee: Omit<Employee, "id" | "organizationId" | "workspaceId">) => Employee;
  removeEmployee: (employeeId: string) => void;
  updateEmployee: (employeeId: string, updates: Partial<Employee>) => void;
  addTask: (task: Omit<Task, "id" | "createdAt">) => Task;
  addArtifact: (artifact: Omit<Artifact, "id" | "createdAt">) => Artifact;
  publishMessage: (msg: Omit<StudioMessage, "id" | "timestamp">) => StudioMessage;
  setActiveMission: (id: string | null) => void;
  setActiveWorkspace: (id: string | null) => void;
}

export const defaultStudioState = (): StudioState => ({
  organization: DEFAULT_ORGANIZATION,
  employees: DEFAULT_EMPLOYEES,
  workspaces: DEFAULT_WORKSPACES,
  missions: [],
  tasks: [],
  artifacts: [],
  messages: [],
  modelProfiles: DEFAULT_MODEL_PROFILES,
  activeMissionId: null,
  activeWorkspaceId: null,
});

// ─── Context ──────────────────────────────────────────────────────────────────

export type StudioContextValue = StudioState & StudioActions;

export const StudioContext = createContext<StudioContextValue | null>(null);

export function useStudio(): StudioContextValue {
  const ctx = useContext(StudioContext);
  if (!ctx) throw new Error("useStudio must be used inside StudioProvider");
  return ctx;
}

// ─── Utilities ────────────────────────────────────────────────────────────────

export function generateId(prefix: string): string {
  return `${prefix}-${Math.random().toString(36).slice(2, 9)}`;
}
