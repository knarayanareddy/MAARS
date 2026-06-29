# MAARS Phase 7

Source: maarsprompt.md

---

## 📋 PHASE 7 — DEPLOYMENT & RELEASE PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 7: DEPLOYMENT, RELEASE & GO-LIVE                      ║
║  Goal: Zero-downtime, safe, monitored production release     ║
╚══════════════════════════════════════════════════════════════╝

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
🚀 DEVOPS LEAD
   → Deployment runbook (step-by-step)
   → Feature flag strategy
   → Blue-green / canary deployment plan
   → Rollback plan (tested and verified)
   → Database migration strategy (zero-downtime)

🔧 PLATFORM ENGINEER
   → Infrastructure provisioning verification (IaC)
   → Environment parity verification
   → Secrets rotation pre-launch
   → SSL/TLS certificate verification
   → DNS configuration verification

📊 OBSERVABILITY ENGINEER
   → Pre-launch dashboard creation
   → Alert thresholds configured and tested
   → On-call runbook created
   → Synthetic monitoring configured
   → Real User Monitoring (RUM) configured

📱 MOBILE DEVOPS
   → App Store Connect submission preparation
   → Google Play Console submission preparation
   → TestFlight / Internal Testing verification
   → App signing and certificates verified
   → Phased rollout plan (1% → 5% → 25% → 100%)

🔒 SECURITY ENGINEER
   → Pre-launch security checklist
   → Secrets management verification
   → WAF rules configured and tested
   → DDoS protection verified

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 SRE CRITIC
   → Is the rollback plan actually tested?
   → What happens if deployment fails at step 7 of 10?
   → Are SLOs being monitored from minute one?
   → Is the on-call team briefed and ready?

🔴 SECURITY CRITIC
   → Last-minute secret/credential exposure check
   → Public exposure audit (Shodan/similar)
   → Are prod credentials different from staging?

🔴 DEVOPS CRITIC
   → Single points of failure in deployment pipeline?
   → What manual steps exist that shouldn't?
   → Configuration drift risks?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 7 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Deployment Runbook
→ Rollback Runbook (tested)
→ Production Monitoring Dashboard
→ On-Call Runbook
→ Go-Live Checklist (signed off by all leads)
→ App Store/Play Store Submission Docs
→ Unanimous Score Documentation
```

---

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 7.5 — Beta / Preview Program** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
