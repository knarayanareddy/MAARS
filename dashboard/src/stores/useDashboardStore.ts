import { create } from "zustand";
import type { ScoreReport } from "../lib/types/contracts";

type StreamState = { builder: string };

interface DashboardState {
  phase: string;
  score: ScoreReport | null;
  stream: StreamState;
  runPhase0: (idea: string) => Promise<void>;
}

export const useDashboardStore = create<DashboardState>((set) => ({
  phase: "Phase 0",
  score: null,
  stream: { builder: "" },
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
        builder: `Builder Card\\n\\nIdea: ${idea}\\n\\n- tri-panel shell\\n- single-model express path\\n- structured renderer stub`
      },
      score
    });

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const response = await invoke<{ runId: string; events: unknown[] }>("run_phase0", { idea });
      console.info("Phase 0 backend run", response);
    } catch {
      // Browser dev fallback: local stub state already populated.
    }
  }
}));
