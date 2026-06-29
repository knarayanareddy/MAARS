# MAARS Phase 5.5

Source: maarspromptpatch.md

---

## 📋 PHASE 5.5 — INTEGRATION & E2E TESTING PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 5.5: INTEGRATION & END-TO-END TESTING                 ║
║  Goal: Verify all systems work together flawlessly           ║
║  Input: All prior phase deliverables + implemented code      ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🔗 INTEGRATION TEST LEAD
Your deliverables:

→ Integration Test Strategy Document:
   • Scope of integration testing (which integrations tested?)
   • Test environment strategy (dedicated integration env?)
   • Test data management approach
   • Service dependency management (real vs mocked)
   • Test isolation strategy (parallel test execution?)

→ Service-to-Service Integration Tests:
   For each service integration:
   • Integration test cases:
     - Happy path (successful integration)
     - Error scenarios (service down, timeout, malformed response)
     - Retry behavior validation
     - Circuit breaker trigger scenarios
     - Fallback mechanism validation
   • Test implementation (JUnit, pytest, Jest, etc.)
   • Assertion criteria (response validation, side effects verified)

→ Database Integration Tests:
   • Real database testing (not in-memory):
     - Connection pool behavior under load
     - Transaction boundary validation
     - Deadlock scenario testing
     - Concurrent access patterns
     - Database constraint enforcement verification
   • Migration testing:
     - Up migration success
     - Down migration (rollback) success
     - Data integrity after migration
   • Performance testing:
     - Query execution time validation
     - Bulk insert performance
     - Index usage verification (EXPLAIN PLAN analysis)

→ Message Queue Integration Tests:
   • Producer-consumer integration (Kafka, RabbitMQ, SQS):
     - Message publishing and consumption
     - Message ordering guarantees
     - Exactly-once/at-least-once delivery verification
     - Dead letter queue behavior
     - Consumer group behavior
   • Event-driven workflow testing:
     - Event triggers correct downstream actions
     - Event failure retry behavior
     - Event schema validation

→ Caching Integration Tests:
   • Cache hit/miss scenarios
   • Cache invalidation verification
   • Cache stampede prevention
   • Distributed cache consistency (if Redis Cluster, etc.)

→ Authentication/Authorization Integration:
   • SSO integration (OAuth, SAML, OIDC)
   • JWT token validation across services
   • Permission enforcement at API layer
   • Session management across distributed system

🌐 THIRD-PARTY INTEGRATION TEST SPECIALIST
Your deliverables:

→ Third-Party API Integration Tests:
   For EACH external API:
   • Sandbox/test mode testing:
     - Authentication/API key validation
     - Request/response format verification
     - Rate limiting behavior
     - Error response handling
   • Mock server integration:
     - WireMock/MockServer configuration
     - Simulated API responses (happy + error paths)
     - Simulated timeouts and network errors
   • Contract testing (Pact):
     - Consumer contract definitions
     - Provider verification
     - Contract version management

→ Payment Gateway Integration (if applicable):
   • Test transaction processing:
     - Successful payment flow
     - Failed payment scenarios (insufficient funds, etc.)
     - Refund processing
     - Webhook callback handling
   • PCI-DSS compliance in test mode
   • Test card numbers and scenarios documented

→ Email/SMS Provider Integration:
   • Email sending verification (test mode/sandbox)
   • Template rendering validation
   • Delivery status tracking
   • Bounce/complaint handling
   • Failover to secondary provider

→ Cloud Services Integration:
   • S3/Blob Storage (file upload, download, deletion)
   • CDN integration (cache invalidation, purge)
   • Cloud queue services (SQS, Cloud Tasks)
   • Serverless function invocation

🎭 END-TO-END (E2E) TEST ENGINEER
Your deliverables:

→ E2E Test Strategy:
   • Critical user journey identification
   • E2E test framework selection (Playwright, Cypress, Selenium)
   • Test environment (staging, pre-prod)
   • Test data provisioning strategy
   • Test execution frequency (nightly, per-deploy, on-demand)

→ Critical User Journey E2E Tests:
   For EACH critical journey:
   • Test scenario definition:
     - User persona
     - Starting state
     - Step-by-step actions
     - Expected outcomes
   • Test implementation:
     - Page object model (POM) pattern
     - Assertions at each step
     - Screenshot/video on failure
     - Test execution time tracking

   Example journeys:
   • User registration → email verification → first login
   • Product search → add to cart → checkout → payment
   • Create content → publish → view as end user
   • Admin: Create user → assign role → verify permissions

→ Cross-Platform E2E Tests:
   • Web application E2E tests:
     - Desktop browsers (Chrome, Firefox, Safari, Edge)
     - Mobile browsers (iOS Safari, Android Chrome)
     - Responsive design validation
   • Mobile app E2E tests (Appium):
     - iOS app critical flows
     - Android app critical flows
     - Cross-platform consistency validation

→ Real-World Scenario Simulation:
   • Multi-user concurrency scenarios
   • Multi-tab behavior (same user, multiple tabs)
   • Session timeout and re-authentication
   • Network interruption and recovery
   • Browser back/forward navigation
   • Bookmark and direct URL access

→ E2E Performance Validation:
   • Page load time assertions (< 3s, < 5s thresholds)
   • Core Web Vitals measurement in E2E tests
   • API response time validation in real workflows
   • User-perceived performance metrics

🔄 CONTRACT TESTING ENGINEER
Your deliverables:

→ Consumer-Driven Contract Testing (Pact):
   • Consumer contracts:
     - Expected request format from consumer
     - Expected response format from provider
   • Provider verification:
     - Provider validates it can fulfill contracts
   • Contract versioning and compatibility matrix
   • Breaking change detection

→ API Schema Validation:
   • OpenAPI schema compliance tests:
     - Request payloads match schema
     - Response payloads match schema
   • JSON Schema validation in tests
   • GraphQL schema validation

→ Backward Compatibility Testing:
   • Old client + new API version
   • New client + old API version
   • Deprecation warning detection

🧪 DATA INTEGRITY TEST ENGINEER
Your deliverables:

→ Multi-Service Data Consistency Tests:
   • Distributed transaction validation (Saga pattern, etc.)
   • Eventual consistency verification
   • Data replication consistency (read replicas)
   • Cross-service data integrity

→ Data Migration Integration Tests:
   • End-to-end migration pipeline test
   • Data reconciliation between source and target
   • Data transformation validation
   • Referential integrity after migration

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 SRE CRITIC
Your critique must identify:
→ Are real production failure scenarios tested?
→ Network partition testing (what if service X can't reach Y?)
→ Cascading failure scenarios covered?
→ Monitoring/alerting tested (do alerts fire in test scenarios?)
→ Disaster recovery scenario testing missing?
→ Specific critique: "No test for when database replica lags 
   by 30 seconds - will app show stale data to users?"

🔴 SECURITY CRITIC
Your critique must identify:
→ Authentication bypass opportunities in integration?
→ Authorization enforcement tested across service boundaries?
→ Sensitive data exposure in integration logs?
→ API security headers validated in integration tests?
→ CSRF/CORS tested in real integration scenarios?
→ Specific critique: "No test validates that service A cannot 
   access service B's data without proper JWT - authz gap"

🔴 BACKEND ENGINEER CRITIC
Your critique must identify:
→ Edge cases not covered in integration tests?
→ Error propagation not tested (does error in service A bubble correctly?)
→ Database transaction rollback scenarios untested?
→ Race condition scenarios not covered?
→ Specific critique: "Concurrent order creation by same user 
   not tested - could create duplicate charges"

🔴 FRONTEND ENGINEER CRITIC
Your critique must identify:
→ E2E tests too brittle (break on minor UI changes)?
→ Real user workflows not represented?
→ Mobile-specific scenarios missing?
→ Accessibility not tested in E2E flows?
→ Specific critique: "E2E tests use CSS selectors that will 
   break when design system updates - need data-testid attributes"

🔴 QA LEAD CRITIC
Your critique must identify:
→ Test coverage gaps (critical paths not tested?)
→ Test data management issues (tests not isolated?)
→ Flaky tests (non-deterministic failures?)
→ Test execution time unreasonable (how long for full suite?)
→ Test reporting insufficient?
→ Specific critique: "Payment integration tests share same 
   test account - tests will fail when run in parallel"

🔴 DEVOPS CRITIC
Your critique must identify:
→ Integration tests not integrated into CI/CD?
→ Test environment provisioning not automated?
→ Test failures blocking deployments properly?
→ Test result reporting and history tracking missing?
→ Specific critique: "Integration tests require manual setup 
   of test database - will slow down CI/CD pipeline"

🔴 MOBILE QA CRITIC (if applicable)
Your critique must identify:
→ Real device testing vs emulator-only?
→ OS version matrix coverage gaps?
→ Network condition variations not tested (2G/3G/4G/WiFi)?
→ App backgrounding/foregrounding scenarios missing?
→ Specific critique: "E2E tests only run on iOS 17 - no 
   coverage for iOS 15/16 which 40% of users still use"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ COVERAGE: /10
   Are all critical integration points tested?

→ RELIABILITY: /10
   Are tests stable and not flaky?

→ REALISM: /10
   Do tests represent real-world scenarios?

→ MAINTAINABILITY: /10
   Can tests evolve with the codebase?

→ AUTOMATION: /10
   Are tests fully automated in CI/CD?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 5.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Integration Test Strategy Document
2. Integration Test Suite (code + documentation)
   - Service-to-service integration tests
   - Database integration tests
   - Message queue integration tests
   - Cache integration tests
3. Third-Party Integration Test Suite
   - Mock server configurations
   - Contract test definitions
   - Sandbox test results
4. E2E Test Suite
   - Critical user journey tests
   - Cross-browser test results
   - Mobile E2E test results
5. Contract Test Suite (Pact contracts)
6. Test Data Management Strategy
7. Test Execution Report
   - Pass/fail statistics
   - Code coverage from integration tests
   - Execution time metrics
8. CI/CD Integration Documentation
9. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 5.5 COMPLETE when:
   → All critics score ≥ 9/10
   → All integration tests passing
   → All E2E tests passing
   → Test coverage adequate (≥80% of critical paths)
   → Tests integrated into CI/CD
   → Flaky tests eliminated

🚫 Phase 5.5 BLOCKED if:
   → Any critic scores < 9/10
   → Critical integration paths untested
   → Tests are flaky
   → Test data management issues
   → Tests not automated

[ONLY AFTER UNANIMOUS 9/10+ → PROCEED TO NEXT PHASE]
```

---
# 📋 REMAINING SDLC PHASE PROMPTS + CROSS-CUTTING GUIDE

---

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 6 — QA & Comprehensive Testing** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
