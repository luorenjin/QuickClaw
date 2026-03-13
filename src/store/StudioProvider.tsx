import { ReactNode, useState, useCallback } from "react";
import {
  StudioContext,
  StudioState,
  defaultStudioState,
  generateId,
} from "./studio";
import {
  Employee,
  Mission,
  Task,
  Artifact,
  StudioMessage,
} from "../types";

export default function StudioProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<StudioState>(defaultStudioState);

  const submitBrief = useCallback((title: string, brief: string): Mission => {
    const mission: Mission = {
      id: generateId("mission"),
      title,
      brief,
      status: "active",
      createdById: "human-owner",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    const msg: StudioMessage = {
      id: generateId("msg"),
      channel: "human_ceo",
      type: "GoalSubmitted",
      fromEmployeeId: "human-owner",
      toEmployeeId: "emp-ceo",
      missionId: mission.id,
      payload: { title, brief },
      timestamp: new Date().toISOString(),
    };
    setState((s) => ({
      ...s,
      missions: [...s.missions, mission],
      messages: [...s.messages, msg],
      activeMissionId: mission.id,
    }));
    return mission;
  }, []);

  const addEmployee = useCallback(
    (employee: Omit<Employee, "id" | "organizationId" | "workspaceId">): Employee => {
      const id = generateId("emp");
      const workspaceId = generateId("ws");
      const newEmp: Employee = {
        ...employee,
        id,
        organizationId: state.organization.id,
        workspaceId,
      };
      setState((s) => ({
        ...s,
        employees: [...s.employees, newEmp],
        workspaces: [
          ...s.workspaces,
          {
            id: workspaceId,
            organizationId: s.organization.id,
            employeeId: id,
            name: `${employee.name} 的工作空间`,
            status: "idle",
            createdAt: new Date().toISOString(),
          },
        ],
      }));
      return newEmp;
    },
    [state.organization.id]
  );

  const removeEmployee = useCallback((employeeId: string) => {
    setState((s) => ({
      ...s,
      employees: s.employees.filter((e) => e.id !== employeeId),
    }));
  }, []);

  const updateEmployee = useCallback(
    (employeeId: string, updates: Partial<Employee>) => {
      setState((s) => ({
        ...s,
        employees: s.employees.map((e) =>
          e.id === employeeId ? { ...e, ...updates } : e
        ),
      }));
    },
    []
  );

  const addTask = useCallback(
    (task: Omit<Task, "id" | "createdAt">): Task => {
      const newTask: Task = {
        ...task,
        id: generateId("task"),
        createdAt: new Date().toISOString(),
      };
      const msg: StudioMessage = {
        id: generateId("msg"),
        channel: "workspace",
        type: "TaskAssigned",
        fromEmployeeId: "emp-ceo",
        toEmployeeId: task.assignedToEmployeeId,
        missionId: task.missionId,
        workspaceId: task.workspaceId,
        payload: { taskId: newTask.id, title: task.title },
        timestamp: new Date().toISOString(),
      };
      setState((s) => ({
        ...s,
        tasks: [...s.tasks, newTask],
        messages: [...s.messages, msg],
      }));
      return newTask;
    },
    []
  );

  const addArtifact = useCallback(
    (artifact: Omit<Artifact, "id" | "createdAt">): Artifact => {
      const newArtifact: Artifact = {
        ...artifact,
        id: generateId("artifact"),
        createdAt: new Date().toISOString(),
      };
      setState((s) => {
        const owner = s.employees.find((e) => e.workspaceId === artifact.workspaceId);
        const msg: StudioMessage = {
          id: generateId("msg"),
          channel: artifact.visibility === "private" ? "workspace" : "mission",
          type: "ArtifactPublished",
          fromEmployeeId: owner?.id ?? "unknown",
          workspaceId: artifact.workspaceId,
          payload: { artifactId: newArtifact.id, name: artifact.name },
          timestamp: new Date().toISOString(),
        };
        return {
          ...s,
          artifacts: [...s.artifacts, newArtifact],
          messages: [...s.messages, msg],
        };
      });
      return newArtifact;
    },
    []
  );

  const publishMessage = useCallback(
    (msg: Omit<StudioMessage, "id" | "timestamp">): StudioMessage => {
      const newMsg: StudioMessage = {
        ...msg,
        id: generateId("msg"),
        timestamp: new Date().toISOString(),
      };
      setState((s) => ({ ...s, messages: [...s.messages, newMsg] }));
      return newMsg;
    },
    []
  );

  const setActiveMission = useCallback((id: string | null) => {
    setState((s) => ({ ...s, activeMissionId: id }));
  }, []);

  const setActiveWorkspace = useCallback((id: string | null) => {
    setState((s) => ({ ...s, activeWorkspaceId: id }));
  }, []);

  return (
    <StudioContext.Provider
      value={{
        ...state,
        submitBrief,
        addEmployee,
        removeEmployee,
        updateEmployee,
        addTask,
        addArtifact,
        publishMessage,
        setActiveMission,
        setActiveWorkspace,
      }}
    >
      {children}
    </StudioContext.Provider>
  );
}
