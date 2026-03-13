import { ReactNode } from "react";
import Sidebar from "./Sidebar";
import "./AppShell.css";

export type StudioPage =
  | "dashboard"
  | "brief"
  | "ceo-workspace"
  | "workspaces"
  | "workspace-detail"
  | "mission-feed"
  | "settings";

interface AppShellProps {
  page: StudioPage;
  onNavigate: (page: StudioPage, extra?: string) => void;
  selectedWorkspaceId?: string;
  children: ReactNode;
}

export default function AppShell({
  page,
  onNavigate,
  selectedWorkspaceId,
  children,
}: AppShellProps) {
  return (
    <div className="app-shell">
      <Sidebar
        activePage={page}
        onNavigate={onNavigate}
        selectedWorkspaceId={selectedWorkspaceId}
      />
      <main className="app-shell-content">{children}</main>
    </div>
  );
}
