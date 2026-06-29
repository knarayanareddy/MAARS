import type { ScoreReport } from "../../../lib/types/contracts";

export function BuilderCard({ text }: { text: string }) {
  return (
    <section className="card">
      <h2>Builder Card</h2>
      <pre className="stream">{text || "No builder output yet."}</pre>
    </section>
  );
}

export function ScoreCard({ score }: { score: ScoreReport | null }) {
  return (
    <section className="card">
      <h2>Score Card</h2>
      {!score ? (
        <div className="muted">Awaiting score...</div>
      ) : (
        <>
          <div className={`badge ${score.decision === "PASS" ? "pass" : score.decision === "FAIL" ? "fail" : "arb"}`}>
            {score.decision} — {score.phase_score}/10
          </div>
          <pre className="stream">{JSON.stringify(score, null, 2)}</pre>
        </>
      )}
    </section>
  );
}
