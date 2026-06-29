import type { ScoreReport } from "../../../lib/types/contracts";

type CardState = {
  loading?: boolean;
  emptyLabel?: string;
  error?: string | null;
};

export function BuilderCard({ text, loading, error, emptyLabel = "No builder output yet." }: { text: string } & CardState) {
  return (
    <section className="card">
      <h2>Builder Card</h2>
      {error ? (
        <div className="error-card" role="alert">
          <strong>Error</strong>
          <div className="muted">{error}</div>
        </div>
      ) : loading ? (
        <div className="muted">Compiling prompt…</div>
      ) : (
        <pre className="stream">{text || emptyLabel}</pre>
      )}
    </section>
  );
}

export function ScoreCard({
  score,
  loading,
  error,
  emptyLabel = "Awaiting score..."
}: { score: ScoreReport | null } & CardState) {
  return (
    <section className="card">
      <h2>Score Card</h2>
      {error ? (
        <div className="error-card" role="alert">
          <strong>Error</strong>
          <div className="muted">{error}</div>
        </div>
      ) : loading ? (
        <div className="muted">Scoring in progress…</div>
      ) : !score ? (
        <div className="muted">{emptyLabel}</div>
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
