# MAARS Phase 6

Source: maarsprompt.md

---

## 📋 PHASE 6 — QA & TESTING PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 6: QA & COMPREHENSIVE TESTING                         ║
║  Goal: Zero defect escape to production                      ║
╚══════════════════════════════════════════════════════════════╝

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
🧪 QA LEAD / TEST ARCHITECT
   → Master test plan
   → Test coverage matrix (requirements traceability)
   → Risk-based testing prioritization

⚡ PERFORMANCE TEST ENGINEER
   → Load test scenarios (expected, peak, stress)
   → Performance benchmarks vs NFR targets
   → Bottleneck identification and resolution
   → Database performance under load

🤖 AUTOMATION QA ENGINEER
   → E2E test suite (Playwright/Cypress/Appium)
   → Regression test suite automation
   → Contract testing (Pact/consumer-driven)
   → Visual regression testing

♿ ACCESSIBILITY QA TESTER
   → Automated accessibility scan (axe-core)
   → Manual screen reader testing (NVDA/JAWS/VoiceOver)
   → Keyboard-only navigation testing
   → WCAG 2.2 AA compliance verification

📱 MOBILE QA ENGINEER
   → Real device testing matrix (iOS + Android)
   → OS version compatibility testing
   → Network condition testing (offline/2G/3G/4G/5G)
   → Battery impact testing
   → Memory pressure testing

🔒 SECURITY QA ENGINEER
   → DAST scanning (OWASP ZAP / Burp Suite)
   → Dependency vulnerability scan (Snyk/Dependabot)
   → SAST scan review (SonarQube/Checkmarx)

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 SENIOR QA CRITIC
   → Test coverage gaps (what wasn't tested?)
   → Edge cases missed?
   → Negative test cases missing?
   → Boundary value analysis gaps?

🔴 SRE CRITIC
   → Chaos testing results?
   → Failover testing done?
   → Recovery testing done?
   → Monitoring alerts verified to fire?

🔴 UAT COORDINATOR
   → Real user feedback incorporated?
   → Business acceptance criteria all met?
   → Stakeholder sign-off obtained?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 6 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Master Test Plan
→ Test Results Report
→ Performance Benchmark Report
→ Accessibility Audit Report
→ Security Scan Report (DAST/SAST)
→ Mobile Compatibility Matrix
→ UAT Sign-off Documentation
→ Defect Log (all resolved or risk-accepted)
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
- **Advance to Phase 6.5 — Data Migration & Seeding** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
