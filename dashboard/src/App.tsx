import { useEffect, useMemo, useState } from "react";
import { useDashboardStore } from "./stores/useDashboardStore";
import { BuilderCard, ScoreCard } from "./features/workspace/stream/Cards";
import type { LoopMode, Preset } from "./lib/types/contracts";

const PRESETS: Preset[] = ["Lite", "Standard", "Full"];
const MODES: { value: LoopMode; label: string }[] = [
  { value: "full-auto", label: "Full-Auto" },
  { value: "semi-auto", label: "Semi-Auto" },
  { value: "supervised", label: "Supervised" }
];

export function App() {
  const [idea, setIdea] = useState("Build a MAARS dashboard");
  const {
    score,
    stream,
    preset,
    mode,
    phases,
    routeOverview,
    running,
    halted,
    needsHuman,
    setPreset,
    setMode,
    runProject,
    refreshRoutes
  } = useDashboardStore();
  const canRun = idea.trim().length > 0 && !running;

  useEffect(() => {
    void refreshRoutes(idea);
  }, [idea, preset, refreshRoutes]);

  const status = useMemo(() => {
    if (halted) return ["HALTED", "fail"] as const;
    if (score?.decision === "PASS") return ["PASS", "pass"] as const;
    if (score?.decision === "FAIL") return ["FAIL", "fail"] as const;
    if (score?.decision === "ARBITRATION") return ["ARBITRATION", "arb"] as const;
    return ["IDLE", "arb"] as const;
  }, [score, halted]);

  return (
    <div className="shell">
      <aside className="panel">
        <div className="card">
          <strong>MAARS</strong>
          <div className="muted">Phase B — full loop</div>
        </div>
        <div className="card">
          <div className="muted">Gate</div>
          <div className={`badge ${status[1]}`}>{status[0]}</div>
        </div>
        <div className="card">
          <div className="muted">Pipeline ({preset})</div>
          <ol className="pipeline" aria-label="Phase pipeline">
            {phases.map((p) => (
              <li key={p.phaseId} className={`pipeline-step ${p.status}`}>
                <span className="dot" aria-hidden="true" />
                Phase {p.phaseId}
                <span className="muted"> · {p.status}</span>
              </li>
            ))}
          </ol>
        </div>
        <div className="card">
          <div className="muted">Route plan</div>
          {routeOverview ? (
            <ul className="route-list">
              {routeOverview.routes.map((route) => (
                <li key={route.role} className="route-row">
                  <strong>{route.role}</strong>
                  <span className="muted">{route.providerId} · {route.model}</span>
                  <span className="muted">{route.budget.usedTokens}/{route.budget.limitTokens} · {Math.round(route.budget.ratio * 100)}%</span>
                  <span className={`badge ${route.budget.band.toLowerCase()}`}>{route.budget.recommendation}</span>
                </li>
              ))}
            </ul>
          ) : (
            <div className="muted">No route overview yet.</div>
          )}
        </div>
      </aside>
      <main className="main">
        <div className="card">
          <h1>Intake — Run a Project</h1>
          <p className="muted">
            Runs the full 11-step loop across every phase of the selected preset, in the selected mode.
          </p>
          <textarea
            rows={4}
            style={{ width: "100%", boxSizing: "border-box" }}
            value={idea}
            onChange={(e) => setIdea(e.target.value)}
          />
          <div className="row" style={{ marginTop: 8, gap: 12 }}>
            <label>
              Preset{" "}
              <select value={preset} onChange={(e) => setPreset(e.target.value as Preset)}>
                {PRESETS.map((p) => (
                  <option key={p} value={p}>
                    {p}
                  </option>
                ))}
              </select>
            </label>
            <label>
              Mode{" "}
              <select value={mode} onChange={(e) => setMode(e.target.value as LoopMode)}>
                {MODES.map((m) => (
                  <option key={m.value} value={m.value}>
                    {m.label}
                  </option>
                ))}
              </select>
            </label>
            <button disabled={!canRun} onClick={() => runProject(idea)}>
              {running ? "Running…" : "Run Project"}
            </button>
          </div>
        </div>
        {needsHuman && (
          <div className="card" role="alert">
            <strong>Needs human input</strong>
            <div className="muted">{needsHuman}</div>
          </div>
        )}
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
          <div className="stream" aria-live="off" aria-busy={running}>
            {stream.builder || "Awaiting run..."}
          </div>
        </div>
      </aside>
    </div>
  );
}
