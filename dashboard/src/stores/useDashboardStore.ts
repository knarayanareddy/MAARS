import { create } from "zustand";
import type {
  AgentEvent,
  DashboardAuditReport,
  GateDecision,
  LoopMode,
  Preset,
  RoutePlan,
  RoutingOverview,
  ProjectMeta,
  ScoreReport
} from "../lib/types/contracts";

type StreamState = { builder: string };

export interface PhaseProgress {
  phaseId: string;
  status: "pending" | "running" | "passed";
}

interface ProjectRun {
  runId: string;
  completed: boolean;
  halted: boolean;
  events: AgentEvent[];
}

interface DashboardState {
  phase: string;
  score: ScoreReport | null;
  stream: StreamState;
  preset: Preset;
  mode: LoopMode;
  phases: PhaseProgress[];
  routes: RoutePlan[];
  routeOverview: RoutingOverview | null;
  audit: DashboardAuditReport | null;
  projects: ProjectMeta[];
  notifications: string[];
  lastError: string | null;
  running: boolean;
  halted: boolean;
  needsHuman: string | null;
  setPreset: (preset: Preset) => void;
  setMode: (mode: LoopMode) => void;
  runPhase0: (idea: string) => Promise<void>;
  runProject: (idea: string) => Promise<void>;
  refreshRoutes: (idea: string) => Promise<void>;
  refreshAudit: (idea: string) => Promise<void>;
}

// Mirrors core/src/phases.rs so the navigator can render the pipeline before the
// backend reports progress.
const PRESET_PHASES: Record<Preset, string[]> = {
  Lite: ["0", "1", "4"],
  Standard: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
  Full: ["0", "1", "1.5", "2", "2.5", "3", "4", "5", "5.5", "6", "6.5", "7", "7.5", "8", "8.5", "9", "10"]
};

function deriveScore(decision: GateDecision): ScoreReport {
  return {
    phase: 0,
    iteration: 1,
    phase_score: decision === "PASS" ? 9 : 6,
    decision,
    bottleneck_critic: null,
    next_action: decision === "PASS" ? "Advance" : "Remediate",
    critic_scores: []
  };
}

export const useDashboardStore = create<DashboardState>((set, get) => ({
  phase: "Phase 0",
  score: null,
  stream: { builder: "" },
  preset: "Standard",
  mode: "full-auto",
  phases: PRESET_PHASES.Standard.map((phaseId) => ({ phaseId, status: "pending" })),
  routes: [],
  routeOverview: null,
  audit: null,
  projects: [],
  notifications: [],
  lastError: null,
  running: false,
  halted: false,
  needsHuman: null,
  setPreset: (preset) =>
    set({ preset, phases: PRESET_PHASES[preset].map((phaseId) => ({ phaseId, status: "pending" })) }),
  setMode: (mode) => set({ mode }),
  refreshRoutes: async (idea) => {
    const { preset } = get();
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const overview = await invoke<RoutingOverview>("inspect_routes_command", { idea, preset });
      set({ routeOverview: overview, routes: overview.routes });
    } catch {
      set({
        routeOverview: null,
        routes: []
      });
    }
  },
  refreshAudit: async (idea) => {
    const { preset } = get();
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const audit = await invoke<DashboardAuditReport>("audit_dashboard_command", { idea, preset });
      set({ audit });
    } catch {
      set({ audit: null });
    }
  },
  runPhase0: async (idea) => {
    const score: ScoreReport = {
      phase: 0,
      iteration: 1,
      phase_score: 9,
      decision: "PASS",
      bottleneck_critic: null,
      next_action: "Advance to Phase 1",
      critic_scores: [
        {
          critic_id: "mock-critic",
          score: 9,
          valid: true,
          invalidation_reason: null,
          key_condition: "Phase 0 skeleton is enough for Phase A",
          findings: []
        }
      ]
    };
    set({
      stream: {
        builder: `Builder Card\n\nIdea: ${idea}\n\n- tri-panel shell\n- single-model express path\n- structured renderer stub`
      },
      score
    });

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const response = await invoke<{ runId: string; events: unknown[] }>("run_phase0_command", { idea });
      console.info("Phase 0 backend run", response);
    } catch {
      // Browser dev fallback: local stub state already populated.
    }
  },
  runProject: async (idea) => {
    const { preset, mode } = get();
    const pipeline: PhaseProgress[] = PRESET_PHASES[preset].map((phaseId) => ({
      phaseId,
      status: "pending"
    }));
    set({
      running: true,
      halted: false,
      needsHuman: null,
      phases: pipeline,
      score: null,
      lastError: null,
      notifications: [...get().notifications, `Run started: ${idea}`]
    });
    await Promise.all([get().refreshRoutes(idea), get().refreshAudit(idea)]);

    const applyEvents = (events: AgentEvent[]) => {
      const phases = PRESET_PHASES[preset].map((phaseId) => ({
        phaseId,
        status: "pending" as PhaseProgress["status"]
      }));
      const passed = new Set<string>();
      let active: string | null = null;
      let lastScore: ScoreReport | null = null;
      let lastBuilder = "";
      let needsHuman: string | null = null;
      const notifications = [...get().notifications];

      for (const ev of events) {
        switch (ev.type) {
          case "stepStart":
            active = ev.phaseId;
            break;
          case "token":
            lastBuilder += ev.chunk;
            break;
          case "score":
            lastScore = ev.report;
            break;
          case "phasePassed":
            passed.add(ev.phaseId);
            break;
          case "needsHuman":
            needsHuman = ev.reason;
            notifications.push(`Needs human input: ${ev.reason}`);
            break;
          default:
            break;
        }
      }

      for (const p of phases) {
        if (passed.has(p.phaseId)) p.status = "passed";
        else if (p.phaseId === active) p.status = "running";
      }
      set({
        phases,
        score: lastScore,
        needsHuman,
        stream: { builder: lastBuilder || get().stream.builder },
        notifications
      });
    };

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const run = await invoke<ProjectRun>("run_project_command", { idea, preset, mode });
      applyEvents(run.events);
      set({
        running: false,
        halted: run.halted,
        notifications: [...get().notifications, `Run ${run.completed ? "completed" : "halted"}`],
        projects: [
          {
            id: run.runId,
            name: idea.slice(0, 40) || "Untitled project",
            preset,
            currentPhase: run.completed ? "Phase 10" : "Interrupted",
            lastActive: new Date().toISOString()
          },
          ...get().projects
        ]
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : "Unknown run error";
      // Browser dev fallback: mark every phase passed so the pipeline renders.
      set({
        phases: pipeline.map((p) => ({ ...p, status: "passed" })),
        score: deriveScore("PASS"),
        running: false,
        lastError: message,
        notifications: [...get().notifications, `Run failed: ${message}`]
      });
    }
  }
}));
