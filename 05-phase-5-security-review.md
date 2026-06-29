# MAARS Phase 5

Source: maarsprompt.md

---

## 📋 PHASE 5 — SECURITY REVIEW PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 5: SECURITY REVIEW & HARDENING                        ║
║  Goal: Zero critical/high vulnerabilities before release     ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
🔒 LEAD SECURITY ARCHITECT
   → Final threat model review (STRIDE + PASTA)
   → Security architecture sign-off
   → Defense-in-depth verification

🕵️ SENIOR PENETRATION TESTER
   → Application layer pen test simulation
   → Authentication/session attack scenarios
   → Business logic abuse scenarios
   → OWASP Top 10 full sweep
   → OWASP API Top 10 full sweep
   → OWASP Mobile Top 10 full sweep

☁️ CLOUD SECURITY ENGINEER
   → IAM policy review (principle of least privilege)
   → S3/storage misconfiguration check
   → Network security group rules review
   → Public exposure audit
   → Secret scanning (no hardcoded credentials)
   → Cloud Security Posture Management (CSPM) review

🔑 IAM ENGINEER
   → Authentication flows security review
   → Token security (JWT claims, expiry, rotation)
   → OAuth 2.0 / OIDC implementation review
   → MFA implementation review
   → Privilege escalation paths?

🔐 CRYPTOGRAPHY ENGINEER
   → Encryption implementation review
   → Key management review
   → TLS configuration (ciphers, versions, certificates)
   → Password hashing review (bcrypt/Argon2/scrypt)
   → Random number generation review

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — RED TEAM CRITIQUE
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 RED TEAM LEAD
   → Adversarial attack simulation
   → What would a nation-state attacker do?
   → What would an insider threat do?
   → What would a script kiddie do?
   → Social engineering attack surfaces?

🔴 OWASP EXPERT CRITIC
   → Any OWASP checklist item not fully addressed?
   → False negatives in security testing?
   → Security controls that can be bypassed?

🔴 COMPLIANCE CRITIC (CISO)
   → Are all regulatory security requirements met?
   → Audit trail completeness?
   → Incident response readiness?

VULNERABILITY SCORING:
→ Critical findings: BUILD MUST STOP. Fix before any scoring.
→ High findings: Must be resolved before 9/10 achievable.
→ Medium findings: Must have remediation plan to score 9.
→ Low/Informational: Documented and accepted or resolved.

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Security Assessment Report
→ Penetration Test Report
→ OWASP Compliance Checklist (signed off)
→ Vulnerability Register (all items addressed)
→ Cloud Security Posture Report
→ Cryptography Review Sign-off
→ Security Hardening Checklist
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
- **Advance to Phase 5.5 — Integration & E2E Testing** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
