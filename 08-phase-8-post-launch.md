# MAARS Phase 8

Source: maarsprompt.md

---

## 📋 PHASE 8 — POST-LAUNCH & CONTINUOUS IMPROVEMENT PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 8: POST-LAUNCH MONITORING & CONTINUOUS IMPROVEMENT    ║
║  Goal: Sustain excellence, grow, and continuously improve    ║
╚══════════════════════════════════════════════════════════════╝

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
📊 AIOPS ENGINEER
   → Anomaly detection configuration
   → Predictive alerting models
   → Automated incident correlation
   → Capacity planning based on ML predictions

🔬 DATA SCIENTIST / ANALYST
   → KPI dashboard and baseline establishment
   → User behavior analysis framework
   → A/B testing framework
   → Retention and engagement metrics

🔒 SOC ANALYST
   → Continuous security monitoring
   → Threat intelligence feeds integrated
   → Incident response playbooks active

💰 FINOPS ENGINEER
   → Cost monitoring and anomaly detection
   → Reserved instance/savings plan optimization
   → Cost allocation tagging audit

🌱 GROWTH ENGINEER
   → Funnel analysis setup
   → Experimentation platform operational
   → Feature flag governance

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 SRE CRITIC (Post-Launch)
   → Are SLOs being met?
   → Error budget burn rate?
   → MTTR analysis — is it acceptable?

🔴 SECURITY CRITIC (Post-Launch)
   → Active threat monitoring operational?
   → Patch cadence defined and active?
   → Vulnerability disclosure process in place?

🔴 PRODUCT CRITIC
   → Are users actually succeeding?
   → What do early metrics tell us?
   → What's the first iteration priority?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]
[This phase repeats on a sprint/release cadence]
```

---

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 8.5 — Incident Response & Runbooks** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
