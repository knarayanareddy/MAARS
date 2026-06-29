# MAARS Order of Operations & Rules

This document defines the **canonical execution pipeline** for the Multi-Agent
Adversarial Refinement System (MAARS) and the **numbered MAARS Rules** that the
phase prompts and meta-agents reference (e.g. "Step 7", "MAARS Rule 6",
"MAARS Rule 8").

It is the orchestration spec: the phase prompts (`00`–`10`) describe *what* each
phase produces, and the meta-agents (`meta-agents/`) describe *how* the loop is
driven. This file ties the two together.

---

## The 11-Step Phase Loop

Every phase — Phase 0 through Phase 10, including the `.5` interstitial phases —
runs the **same 11-step loop**. The loop is deterministic: each step has a single
trigger condition and a single next action.

```
┌────────────────────────────────────────────────────────────────────┐
│                     MAARS PER-PHASE LOOP                             │
└────────────────────────────────────────────────────────────────────┘

  ┌─ Step 1 ─ PHASE INITIALIZATION
  │   Load the Master System Prompt + the prior phase's Snapshot
  │   Capsule. Re-inject the 7 Core Operating Principles. Confirm the
  │   Critic Panel roster and Scoring Rubric for this phase.
  │
  ├─ Step 2 ─ CONTINUITY AGENT            (meta-agents/01)
  │   Compress prior-phase outputs into a lossless handoff package;
  │   list frozen decisions, open risks; GO / NO-GO kickoff checklist.
  │
  ├─ Step 3 ─ RESEARCH AGENT              (meta-agents/02)
  │   Live-web validation of tech freshness, best-practice deltas, and
  │   regulatory updates. Emits the Builder Briefing Card.
  │   → 🛑 RESEARCH BLOCKER halts the phase until resolved.
  │
  ├─ Step 4 ─ BUILDER PANEL               (phase prompt — Step 1 "Builder Panel")
  │   The phase-specific expert builders produce the phase deliverables,
  │   with the Builder Briefing Card attached at the top.
  │
  ├─ Step 5 ─ CRITIQUE PANEL              (phase prompt — Step 2 "Critique Panel")
  │   The phase-specific adversarial critics review the Builder output
  │   using the Universal Critique Prompt; classify every finding
  │   (Critical / Major / Minor / Advisory) and score 1–10.
  │
  ├─ Step 6 ─ DEVIL'S ADVOCATE            (cross-phase critic)
  │   A dedicated contrarian critic that must surface ≥ 3 substantive
  │   risks/gaps the panel missed (see Rule 6).
  │
  ├─ Step 7 ─ SCORING AGGREGATOR          (meta-agents/03)
  │   Collects all critic + Devil's Advocate scores, invalidates
  │   unjustified/inflated scores, consolidates findings, and issues the
  │   phase-gate verdict: 🟢 PASS / 🔴 FAIL / 🟡 ARBITRATION.
  │
  │        ┌──────────────── verdict routing ────────────────┐
  │        │                                                  │
  ├─ Step 8 ─ REMEDIATION AGENT (meta-agents/04)              │
  │   Fires on 🔴 FAIL when iteration ≤ 5. Turns findings into       │
  │   traceable tickets, produces a fully remediated deliverable,     │
  │   then loops back to Step 5 (Critique Round N+1).                 │
  │        │                                                  │
  ├─ Step 9 ─ ARBITRATION AGENT (meta-agents/05)              │
  │   Fires on 🟡 ARBITRATION when iteration > 5 and score < 9.       │
  │   CTO-level final ruling that breaks the deadlock (see Rule 8).   │
  │        │                                                  │
  │        └──────────────────────────────────────────────────┘
  │
  ├─ Step 10 ─ LIVING DOCUMENT AGENT      (meta-agents/06)
  │   Fires on 🟢 PASS (unanimous ≥ 9). Persists the phase to /docs/
  │   (changelog, ADRs, risk register, glossary, API contract registry).
  │
  └─ Step 11 ─ PHASE SNAPSHOT AGENT       (meta-agents/07)
      Compresses everything into a token-budgeted (≤ 8k, hard cap 12k)
      Snapshot Capsule with checksum-referenced deliverables, and feeds
      it into the NEXT phase's Continuity Agent (Step 2).

  ──▶ Advance to Phase N+1, return to Step 1.
```

### Trigger summary

| Step | Agent / Activity | Fires when |
|------|------------------|------------|
| 1 | Phase Initialization | Start of every phase |
| 2 | Continuity Agent | Before each new phase |
| 3 | Research Agent | After Continuity, before Builder Panel |
| 4 | Builder Panel | After Research clears |
| 5 | Critique Panel | After Builder output exists |
| 6 | Devil's Advocate | With the Critique Panel, each round |
| 7 | Scoring Aggregator | After Critique Panel (incl. Devil's Advocate) |
| 8 | Remediation Agent | Verdict = FAIL **and** iteration ≤ 5 |
| 9 | Arbitration Agent | Iteration > 5 **and** score still < 9 |
| 10 | Living Document Agent | Verdict = PASS (unanimous ≥ 9) |
| 11 | Phase Snapshot Agent | After Living Document, before advancing |

### Phase 0 exception

Phase 0 is the entry point and has **no predecessor**, so its handoff steps are
no-ops: there is no prior Snapshot Capsule (Step 1) and nothing for the
Continuity Agent to hand off (Step 2). The Research Agent (Step 3) still runs,
but against the raw idea rather than prior ADRs. Phase 0 effectively begins at
the Builder Panel (Step 4). Every phase from 1 onward runs the full 11-step loop.

### A note on "Step" numbering

Each phase prompt (`00`–`10`) has its own internal `STEP 1 / 2 / 3`
(Builder Panel → Critique Panel → Remediation). Those are **local** to the phase
and are not the same as the 11 meta-steps above. The mapping is: phase
`STEP 1` = meta Step 4, phase `STEP 2` = meta Steps 5–6, phase `STEP 3` = meta
Step 8. The meta-steps wrap the phase prompt; the phase prompt does not replace
them.

---

## The MAARS Rules

These are the **non-negotiable rules** of the framework. The phase prompts and
meta-agents cite them by number.

**Rule 1 — Principles are invariant.**
The 7 Core Operating Principles (Shift-Left Security, Design by Contract,
Observability-First, Accessibility by Default, Documentation as Code, Compliance
as Architecture, Everything as Code) must be applied in **every** output of
**every** step. They are re-injected at Step 1 of each phase.

**Rule 2 — Unanimous 9/10 gate.**
A phase may advance only when **all** valid critics independently score the phase
output **≥ 9/10** with documented justification.

**Rule 3 — The phase score is the lowest valid score.**
There is no averaging and no majority rule. One valid 6 fails the phase,
regardless of how many 10s the other critics give.

**Rule 4 — No critique is silently dropped.**
Every Critical and Major finding must end in one of two states: ✅ RESOLVED, or
⚠ ACCEPTED RISK (with a risk ID, owner, mitigation, and review date). Nothing is
ignored.

**Rule 5 — Scores must be earned, not inflated.**
The Scoring Aggregator invalidates any score that is unjustified, generic,
contradicts the critic's own findings, was influenced by other critics, or sits
≥ 9 while Critical/Major issues remain open. An invalid score becomes an ABSTAIN
and the critic must re-score.

**Rule 6 — The Devil's Advocate must bite.**
The Devil's Advocate critic must surface **at least 3 substantive** risks or
gaps per round. Fewer than 3 is flagged by the Scoring Aggregator as inadequate
adversarial review and the round does not count toward the iteration budget.

**Rule 7 — Maximum 5 iterations before arbitration.**
The Critique ↔ Remediation loop may run at most **5 iterations**. If the phase
has not reached unanimous 9/10 by then, the Arbitration Agent is mandatory — the
loop may not continue indefinitely.

**Rule 8 — Arbitration is final.**
The Arbitration Agent's ruling constitutes phase passage. Constraints on that
authority:
- Maximum **2 critic overrules** per phase; needing more implies a fundamental
  flaw → recommend rollback.
- Security Criticals — authentication bypass, SQL injection, plaintext PII —
  may **never** be "accepted risk". They are MUST FIX or ROLLBACK.
- "Pass with conditions" is preferred over infinite iteration; perfect is the
  enemy of shipped.

**Rule 9 — Every passed phase is persisted, then compressed.**
On passage, the Living Document Agent (Step 10) versions all artifacts to
`/docs/`, and the Phase Snapshot Agent (Step 11) compresses them into the
Snapshot Capsule **before** the next phase begins. No phase advances on
un-persisted knowledge.

**Rule 10 — Decisions and contracts are lossless.**
Context compression may drop prose, examples, and closed risks, but must
**never** drop ADRs/decisions, open Critical/Major risks, API endpoint auth
requirements, compliance constraints, interface-contract field semantics, the 7
Core Principles, or sign-off evidence. The Snapshot Capsule hard cap is 12,000
tokens.

---

## How this maps to the files

| Concept | File(s) |
|---------|---------|
| Principles, scoring rubric, universal templates | `00-master-system-prompt.md` |
| Phase deliverables (what to build/critique) | `00-phase-0-*.md` … `10-phase-10-*.md` |
| Continuous disciplines woven through phases | `cc-cross-cutting-concerns.md` |
| Loop orchestration (Steps 2, 3, 7–11) | `meta-agents/01`–`07` |
| Loop sequence + numbered rules (this file) | `ORDER-OF-OPERATIONS.md` |
