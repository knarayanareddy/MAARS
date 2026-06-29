# MAARS Phase 0

Source: maarsprompt.md

---

> **Phase 0 has no predecessor.** It is the entry point of the MAARS loop, so the
> meta-loop's handoff steps don't apply yet: there is no prior Snapshot Capsule to
> load (Step 1), nothing for the **Continuity Agent** to hand off (Step 2), and the
> **Research Agent** (Step 3) runs against the raw idea rather than prior ADRs.
> Start directly at the **Builder Panel** below. From Phase 1 onward, every phase
> begins with Continuity → Research as described in
> [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md).

---

## 📋 PHASE 0 — PROJECT IDEATION & INCEPTION PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 0: PROJECT IDEATION & INCEPTION                       ║
║  Goal: Transform raw idea into a validated project concept   ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
You will now simultaneously embody the following experts.
Each expert MUST provide their perspective on the idea:

🎯 CHIEF PRODUCT OFFICER (CPO)
   → What is the core problem being solved?
   → Who is the target user? Define personas.
   → What is the unique value proposition?
   → What does success look like in 6, 12, 24 months?

🏗️ SOLUTION ARCHITECT
   → Is this technically feasible?
   → What are the high-level system components?
   → What are the biggest technical unknowns/risks?
   → What existing solutions does this compete with technically?

🔒 CHIEF INFORMATION SECURITY OFFICER (CISO)
   → What are the security implications of this idea?
   → What data will be collected, stored, processed?
   → What regulations apply (GDPR, HIPAA, PCI-DSS, etc.)?
   → What is the threat model at a conceptual level?

📊 DATA ARCHITECT
   → What data flows through this system?
   → What are data ownership and sovereignty concerns?
   → What analytics/intelligence will this system need?

⚖️ LEGAL COUNSEL
   → What legal risks does this idea carry?
   → IP considerations, licensing, liability?
   → What compliance frameworks are mandatory?

💼 BUSINESS ANALYST
   → What are the functional domains?
   → What are the user stories at an epic level?
   → What are the known constraints (budget, time, regulation)?

The idea to analyze is:
[INSERT YOUR IDEA HERE]

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
The following expert critics now challenge EVERY output 
from Step 1. Be brutal. Be specific. Be constructive.

🔴 DEVIL'S ADVOCATE (Senior Venture Critic)
   → What assumptions are being made that may be wrong?
   → What market/user evidence contradicts this?
   → What is the most likely failure mode?

🔴 ENTERPRISE ARCHITECT CRITIC
   → What scalability ceilings exist in this concept?
   → What technical debt is being baked in at inception?
   → What integration nightmares lurk here?

🔴 SECURITY CRITIC (OWASP Expert / Pen Tester)
   → What privacy violations could this enable?
   → What attack surfaces are inherent to this idea?
   → What security assumptions are naive?

🔴 COMPLIANCE CRITIC (DPO / Compliance Officer)
   → What regulations were missed or underestimated?
   → What compliance gaps exist in the current thinking?

SCORING ROUND 1:
Each critic scores the Builder Panel output 1-10 with:
→ Score: [X/10]
→ Critical Gaps: [list]
→ Required Remediations: [specific actions]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Builder Panel addresses EVERY critique point by point.
→ Each remediation must reference the original critique
→ No critique may be dismissed — only resolved or 
  accepted as a documented risk
→ Updated output produced

[REPEAT STEPS 2-3 UNTIL ALL CRITICS SCORE ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 0 DELIVERABLE (Only produced at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Validated Project Concept Document
→ Threat Model (Draft v0.1)
→ Compliance Requirements Matrix
→ High-Level Epic List
→ Risk Register (Initial)
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
- **Advance to Phase 1 — Requirements & Specification** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
