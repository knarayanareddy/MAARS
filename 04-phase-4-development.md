# MAARS Phase 4

Source: maarsprompt.md

---

## 📋 PHASE 4 — DEVELOPMENT PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 4: DEVELOPMENT                                        ║
║  Goal: Produce production-grade, secure, tested code         ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]
[INPUT: All Prior Phase Deliverables]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL (Per Feature/Module)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
For EACH component being built, the following experts produce:

🏗️ SENIOR SOFTWARE ARCHITECT
   → Module design (SOLID principles applied)
   → Design patterns selected with justification
   → Interface contracts (Design by Contract)
   → Pre-conditions, post-conditions, invariants

💻 SENIOR BACKEND ENGINEER
   → API implementation (following OpenAPI spec)
   → Business logic implementation
   → Database queries optimized
   → Error handling and resilience patterns
     (circuit breakers, retries, timeouts, bulkheads)
   → Unit tests (>80% coverage minimum)
   → Integration tests

🎨 SENIOR FRONTEND ENGINEER
   → Component implementation (following design system)
   → State management implementation
   → API integration with error handling
   → Performance optimization 
     (lazy loading, code splitting, memoization)
   → Unit + integration tests

📱 SENIOR iOS DEVELOPER
   → Swift/SwiftUI implementation
   → Memory management review
   → iOS-specific security (Keychain, App Transport Security)
   → Offline/sync implementation
   → Unit + UI tests (XCTest)

📱 SENIOR ANDROID DEVELOPER
   → Kotlin/Jetpack Compose implementation
   → Android-specific security (EncryptedSharedPreferences,
     Network Security Config)
   → ProGuard/R8 configuration
   → Unit + instrumentation tests

🔒 OWASP SECURITY ENGINEER
   → Security controls implementation:
     Input validation, output encoding, 
     parameterized queries, authentication,
     authorization (RBAC/ABAC), rate limiting,
     CSRF protection, security headers
   → OWASP Top 10 checklist applied to every module

📊 OBSERVABILITY ENGINEER
   → Structured logging implementation
   → Distributed tracing instrumentation (OpenTelemetry)
   → Metrics instrumentation
   → Health check endpoints
   → Alerting rule definitions

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 OWASP EXPERT / PENETRATION TESTER
   → OWASP Top 10 violations (specific, per line if needed)
   → OWASP API Top 10 violations
   → OWASP Mobile Top 10 violations (for mobile code)
   → Injection vulnerabilities?
   → Authentication weaknesses?
   → Broken access control?
   → Cryptographic failures?
   → Security misconfiguration?
   → Sensitive data exposure?

🔴 SENIOR CODE REVIEWER (Language Expert)
   → Anti-patterns and code smells (specific)
   → SOLID violations (specific)
   → Performance bottlenecks (specific lines)
   → Memory leaks?
   → Race conditions?
   → Exception handling gaps?

🔴 DATABASE EXPERT / DBA
   → SQL injection risks?
   → N+1 query problems?
   → Missing indexes?
   → Transaction isolation issues?
   → Data integrity constraint gaps?

🔴 DEVOPS / SRE ENGINEER
   → Is this code observable? (logging gaps, missing traces)
   → Deployment risks?
   → Configuration management issues?
   → Resource leak risks?
   → Graceful shutdown implemented?

🔴 PERFORMANCE ENGINEER
   → Algorithm complexity issues (O(n²) where O(n) possible)
   → Blocking I/O in async contexts?
   → Missing caching opportunities?
   → Payload size issues?

🔴 ACCESSIBILITY ENGINEER (Frontend Critique)
   → Missing ARIA attributes?
   → Keyboard trap risks?
   → Dynamic content announcement issues?
   → Focus management problems?

SCORING RUBRIC FOR CODE:
→ Security: /10 (minimum 9 required)
→ Performance: /10 (minimum 9 required)  
→ Maintainability: /10 (minimum 9 required)
→ Test Coverage: /10 (minimum 9 required)
→ Observability: /10 (minimum 9 required)
→ COMPOSITE SCORE: Average (must be ≥ 9)

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 4 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ Production-ready code (all modules)
→ Unit test suite (>80% coverage)
→ Integration test suite
→ API documentation (OpenAPI/Swagger)
→ Code review sign-off documentation
→ Security sign-off (OWASP checklist completed)
→ Performance benchmarks
→ Observability instrumentation verified
→ Unanimous Score Documentation
```

---
