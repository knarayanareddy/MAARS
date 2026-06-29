import { useMemo, useState } from "react";
import { useDashboardStore } from "./stores/useDashboardStore";
import { BuilderCard, ScoreCard } from "./features/workspace/stream/Cards";

export function App() {
  const [idea, setIdea] = useState("Build a MAARS dashboard");
  const { phase, score, stream, runPhase0 } = useDashboardStore();
  const canRun = idea.trim().length > 0;

  const status = useMemo(() => {
    if (score?.decision === "PASS") return ["PASS", "pass"] as const;
    if (score?.decision === "FAIL") return ["FAIL", "fail"] as const;
    if (score?.decision === "ARBITRATION") return ["ARBITRATION", "arb"] as const;
    return ["IDLE", "arb"] as const;
  }, [score]);

  return (
    <div className="shell">
      <aside className="panel">
        <div className="card">
          <strong>MAARS</strong>
          <div className="muted">Phase A walking skeleton</div>
        </div>
        <div className="card">
          <div className="muted">Phase</div>
          <div>{phase}</div>
        </div>
        <div className="card">
          <div className="muted">Gate</div>
          <div className={`badge ${status[1]}`}>{status[0]}</div>
        </div>
      </aside>
      <main className="main">
        <div className="card">
          <h1>Intake Wizard — Express Path</h1>
          <p className="muted">Single model for Phase A. No routing table yet.</p>
          <textarea
            rows={4}
            style={{ width: "100%", boxSizing: "border-box" }}
            value={idea}
            onChange={(e) => setIdea(e.target.value)}
          />
          <div className="row" style={{ marginTop: 8 }}>
            <button disabled={!canRun} onClick={() => runPhase0(idea)}>
              Run Phase 0
            </button>
          </div>
        </div>
        <BuilderCard text={stream.builder} />
        <ScoreCard score={score} />
      </main>
      <aside className="panel right">
        <div className="card">
          <strong>Context</strong>
          <div className="muted">Tri-panel shell, right sidebar stub</div>
        </div>
        <div className="card">
          <strong>Stream</strong>
          <div className="stream" aria-live="off" aria-busy="true">{stream.builder || "Awaiting run..."}</div>
        </div>
      </aside>
    </div>
  );
}
