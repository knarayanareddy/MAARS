# MAARS — Multi-Agent Adversarial Refinement System

A complete, copy-paste **prompt library** for driving an LLM through a full
software development lifecycle (SDLC) using a multi-agent **builder vs. critic**
adversarial pattern with **unanimous 9/10 consensus gates** at every phase.

Instead of one-shot prompting, MAARS simulates an **expert panel** that iterates
adversarially until quality converges: builders produce, critics tear it apart,
remediation fixes it, and nothing advances until every critic independently
signs off at 9/10 or higher.

> This is a documentation/prompt library — there is no application code to build
> or run. You use it by copying prompts into your LLM of choice.

---

## How to use

1. Prepend **`00-master-system-prompt.md`** to your session. It encodes the 7
   Core Operating Principles, the consensus scoring protocol, and the universal
   critique/iteration/scoring templates.
2. Run **Phase 0** (`00-phase-0-ideation-inception.md`) with your project idea.
3. Drive each phase through the **11-step loop** defined in
   **[`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md)**, using the
   `meta-agents/` to manage handoff, research, scoring, remediation,
   arbitration, documentation, and context compression.
4. Iterate until all critics score ≥ 9/10, lock the deliverables, then advance.
5. Repeat for every phase. Never skip. Never shortcut.

See **[`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md)** for the full pipeline
and the numbered MAARS Rules.

---

## Adapting MAARS to your project

MAARS is a **process framework, not a tech stack** — it adapts to essentially
any software project idea. There is no project-specific hardcoding anywhere:
Phase 0 starts from `[INSERT YOUR IDEA HERE]`, every later phase takes "prior
phase deliverables" as input, and builders/critics are *personas* that reason
about **your** chosen stack rather than dictating one. Concrete tech mentions
(PostgreSQL, JWT, iOS versions, GDPR/HIPAA/PCI) are illustrative examples inside
critique prompts, not requirements.

To adapt it: paste the master prompt → fill in Phase 0 with your idea and
constraints (platform, users, scale, compliance, tech preferences) → pick the
phases and critic personas that apply → run the loop. Scale the ceremony to the
project; its value grows with complexity.

### Project-size presets

Pick a preset as a starting point, then add or drop phases as your project
demands. The core spine (Phase 0 → 1 → 2 → 4) is always recommended.

| Preset | Good for | Run these phases | Meta-agents |
|--------|----------|------------------|-------------|
| **Lite** | Scripts, one-off prototypes, spikes, static sites | 0 → 1 → 4 | Scoring + Remediation only (skip Continuity/Research/Snapshot — single sitting) |
| **Standard** | Typical web/mobile app, internal tool, SaaS MVP | 0 → 1 → 1.5 → 2 → 2.5 → 3 → 4 → 5 → 6 → 7 → 8 | Full loop |
| **Full** | Regulated, security-sensitive, or multi-team platforms | All 17 phases (0 → 10, every `.5`) | Full loop, Arbitration enabled |

### Which phases to include

Drop a phase when its subject doesn't exist in your project:

| Phase | Skip when… |
|-------|-----------|
| `1.5` API / Interface Design | no external/internal API surface (e.g. a pure script) |
| `2.5` Database Design | no persistent data layer |
| `3` UX / UI Design | backend-/API-only, no human-facing UI |
| `5.5` Integration & E2E Testing | single component, no integrations |
| `6.5` Data Migration & Seeding | greenfield with no existing data to migrate |
| `7.5` Beta / Preview | no staged rollout (internal tool, hard launch) |
| `8.5` Incident Response & Runbooks | non-production / throwaway project |
| `9` Training & Change Mgmt | no end users to train or onboard |

The `.5` phases are modular by design — removing one doesn't break the
0 → 10 spine, since each phase consumes the *prior* phase's deliverables
regardless of which interstitials ran.

### Which personas to include

Each phase names a default Builder Panel and Critique Panel, but you choose the
**roster** per project:

- **Always keep:** Devil's Advocate, and the security critics (Pen Tester /
  OWASP Expert) — Shift-Left Security is a non-negotiable principle.
- **Add** domain critics your project needs: FinOps (cost-sensitive), Privacy /
  DPO (handles PII), Accessibility Expert (consumer UI), SRE (high-availability),
  ML/Data Science (model-driven features).
- **Drop** personas with no surface area: e.g. Mobile/Platform critics for a
  pure backend, Accessibility for a headless service.
- **Tune compliance:** the framework leans on GDPR/HIPAA/PCI/SOC2 examples —
  swap in (or remove) the regimes that actually apply to you.

> Rule of thumb: fewer phases and a leaner panel for small ideas; the full
> 17-phase loop with arbitration for anything regulated, security-critical, or
> long-lived.

---

## Repository layout

### Master prompt
| File | Purpose |
|------|---------|
| `00-master-system-prompt.md` | The "operating system" — prepended to every session. 7 Core Principles, scoring protocol, persona protocol, and the Universal Critique / Iteration Loop / Scoring Rubric templates. |

### Phases (the SDLC)
The 17 SDLC phases, one file each. `.5` files are interstitial phases.

| File | Phase |
|------|-------|
| `00-phase-0-ideation-inception.md` | Ideation & Inception |
| `01-phase-1-requirements.md` | Requirements & Specification |
| `01_5-phase-1_5-api-interface-design.md` | API / Interface Design |
| `02-phase-2-architecture.md` | Architecture & System Design |
| `02_5-phase-2_5-database-design.md` | Database Design & Data Modeling |
| `03-phase-3-ux-ui-design.md` | UX / UI Design |
| `04-phase-4-development.md` | Development |
| `05-phase-5-security-review.md` | Security Review & Hardening |
| `05_5-phase-5_5-integration-e2e-testing.md` | Integration & E2E Testing |
| `06-phase-6-qa-testing.md` | QA & Comprehensive Testing |
| `06_5-phase-6_5-data-migration-seeding.md` | Data Migration & Seeding |
| `07-phase-7-deployment-release.md` | Deployment, Release & Go-Live |
| `07_5-phase-7_5-beta-preview.md` | Beta / Preview Program |
| `08-phase-8-post-launch.md` | Post-Launch Monitoring |
| `08_5-phase-8_5-incident-response-runbooks.md` | Incident Response & Runbooks |
| `09-phase-9-training-documentation-change-mgmt.md` | Training, Documentation & Change Management |
| `10-phase-10-retrospective-continuous-learning.md` | Retrospective & Continuous Learning |

### Meta-agents (loop orchestration)
The orchestration layer that wraps the build/critique loop. Each is keyed to a
step in `ORDER-OF-OPERATIONS.md`.

| File | Agent | Step | Role |
|------|-------|------|------|
| `meta-agents/01-continuity-agent.md` | Continuity | 2 | Lossless phase-to-phase handoff; re-anchors principles, frozen decisions, open risks. |
| `meta-agents/02-research-agent.md` | Research | 3 | Live-web validation of tech freshness, best practices, and regulatory deltas. |
| `meta-agents/03-scoring-aggregator.md` | Scoring Aggregator | 7 | Tallies critic scores, invalidates inflated ones, issues the phase-gate verdict. |
| `meta-agents/04-remediation-agent.md` | Remediation | 8 | Turns findings into traceable tickets and produces the remediated deliverable. |
| `meta-agents/05-arbitration-agent.md` | Arbitration | 9 | Breaks deadlock after 5 iterations with a final, accountable ruling. |
| `meta-agents/06-living-document-agent.md` | Living Document | 10 | Persists the passed phase to `/docs/` (changelog, ADRs, risks, glossary, API registry). |
| `meta-agents/07-phase-snapshot-agent.md` | Phase Snapshot | 11 | Compresses the phase into a token-budgeted capsule for the next phase. |

### Cross-cutting & orchestration
| File | Purpose |
|------|---------|
| `cc-cross-cutting-concerns.md` | The 8 continuous disciplines (security, accessibility, observability, documentation, compliance, everything-as-code, cost, metrics) and how to weave each through every phase. |
| `ORDER-OF-OPERATIONS.md` | The 11-step per-phase loop and the numbered MAARS Rules. |

---

## The framework at a glance

- **7 Core Operating Principles** — applied in every output (see master prompt).
- **Builder Panel → Critique Panel → Remediation** loop per phase.
- **Unanimous 9/10 gate** — the phase score is the *lowest* valid critic score;
  no averaging, no majority rule.
- **Meta-agent pipeline** — continuity, research, scoring, remediation,
  arbitration, living docs, and snapshotting solve context rot, persona drift,
  false consensus, infinite loops, and staleness.
- **8 cross-cutting concerns** woven through every phase.

> This framework builds a self-correcting, adversarially-hardened,
> expert-consensus-driven development engine. Every phase's output is
> battle-tested before the next phase begins.
