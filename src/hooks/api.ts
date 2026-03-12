/**
 * Typed wrappers around Tauri `invoke` for all backend commands.
 * Keeps component code clean and free of raw string command names.
 */
import { invoke } from "@tauri-apps/api/core";
import type {
  ClawConfig,
  DepInfo,
  ProgressSnapshot,
  ChatMessage,
} from "../types";

export const api = {
  // ── Config ──────────────────────────────────────────────────────────────
  loadConfig: () => invoke<ClawConfig>("load_config"),
  saveConfig: (config: ClawConfig) => invoke<void>("save_config", { config }),

  // ── Environment ─────────────────────────────────────────────────────────
  checkDependencies: () => invoke<DepInfo[]>("check_dependencies"),
  checkSingleDependency: (name: string) =>
    invoke<DepInfo>("check_single_dependency", { name }),
  installDependency: (name: string) =>
    invoke<void>("install_dependency", { name }),
  getDepProgress: (name: string) =>
    invoke<ProgressSnapshot>("get_dep_progress", { name }),

  // ── OpenClaw ─────────────────────────────────────────────────────────────
  detectOpenclaw: (installDir: string) =>
    invoke<boolean>("detect_openclaw", { installDir: installDir }),
  defaultOpenclawConfig: () =>
    invoke<{ install_dir: string; npm_package: string }>("default_openclaw_config"),
  installOpenclaw: (installDir: string, npmPackage: string) =>
    invoke<void>("install_openclaw", {
      request: { install_dir: installDir, npm_package: npmPackage },
    }),
  getOpenclawProgress: () =>
    invoke<ProgressSnapshot>("get_openclaw_progress"),
  launchOpenclaw: (installDir: string) =>
    invoke<number>("launch_openclaw", { installDir: installDir }),

  // ── Connection ───────────────────────────────────────────────────────────
  testConnection: (url: string, apiKey: string) =>
    invoke<string>("test_connection", { url, apiKey: apiKey }),

  // ── Chat ─────────────────────────────────────────────────────────────────
  sendChatMessage: (
    config: ClawConfig,
    history: ChatMessage[],
    message: string
  ) =>
    invoke<string>("send_chat_message", {
      config,
      history: history.map((m) => ({ role: m.role, content: m.content })),
      message,
    }),
};
