# MAARS Phase 1

Source: maarsprompt.md

---

## 📋 PHASE 1 — REQUIREMENTS & SPECIFICATION PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 1: REQUIREMENTS & SPECIFICATION                       ║
║  Goal: Produce airtight FRS, SRS, and non-functional reqs   ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]
[INPUT: Phase 0 Deliverables]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
📋 SENIOR BUSINESS ANALYST
   → Full functional requirements (FRS)
   → User stories with acceptance criteria (Gherkin format)
   → Business rules and constraints
   → Use case diagrams (described in detail)

🏗️ SOLUTION ARCHITECT
   → Non-functional requirements (NFRs):
     Performance SLAs, Scalability targets, 
     Availability (SLA%), Disaster recovery RTO/RPO
   → System context diagrams
   → Constraints and assumptions documented

🔒 SECURITY ARCHITECT
   → Security requirements specification
   → Authentication & authorization requirements
   → Data classification requirements
   → Encryption requirements (at rest, in transit, in use)
   → Audit logging requirements

♿ ACCESSIBILITY SPECIALIST
   → Accessibility requirements (WCAG 2.2 AA minimum)
   → Assistive technology support requirements
   → Inclusive design requirements

📊 DATA ARCHITECT
   → Data requirements specification
   → Data retention policies
   → Data sovereignty requirements
   → PII/PHI identification and handling requirements

⚖️ COMPLIANCE OFFICER + DPO
   → Compliance requirements mapped to features
   → Privacy requirements (Privacy by Design documentation)
   → Regulatory constraints per feature

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 SENIOR QA LEAD (Test Architect)
   → Are requirements testable? Flag every untestable requirement.
   → Are acceptance criteria complete and unambiguous?
   → What test scenarios are impossible to verify?
   → Ambiguity audit: flag every vague word 
     (e.g., "fast", "secure", "scalable" without metrics)

🔴 OWASP EXPERT
   → Map every requirement to OWASP Top 10 / OWASP API Top 10
   → Flag missing security requirements
   → Identify requirements that CREATE security vulnerabilities

🔴 SENIOR BACKEND ENGINEER
   → Flag technically infeasible requirements
   → Identify requirements that will cause performance issues
   → What's missing for API contracts to be defined?

🔴 DEVOPS / SRE LEAD
   → Are NFRs achievable with realistic infrastructure?
   → Are observability requirements specified?
   → Are deployment and release requirements defined?

🔴 MOBILE ARCHITECT
   → Do requirements account for offline scenarios?
   → Platform-specific requirement gaps (iOS/Android)?
   → Battery, bandwidth, and storage constraints addressed?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 1 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Functional Requirements Specification (FRS)
→ Software Requirements Specification (SRS)  
→ Non-Functional Requirements Document (NFRs with metrics)
→ Security Requirements Specification
→ Accessibility Requirements Document
→ Data Requirements & Classification Matrix
→ Compliance Requirements Traceability Matrix
→ User Story Backlog (Gherkin format, prioritized)
→ Updated Risk Register
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
- **Advance to Phase 1.5 — API / Interface Design** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
