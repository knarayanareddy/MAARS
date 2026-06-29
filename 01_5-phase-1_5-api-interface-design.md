# MAARS Phase 1.5

Source: maarspromptpatch.md

---

## 📋 PHASE 1.5 — API/INTERFACE DESIGN PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 1.5: API & INTERFACE DESIGN                           ║
║  Goal: Design bulletproof API contracts before any code      ║
║  Input: Phase 0 + Phase 1 Deliverables                      ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🔌 LEAD API ARCHITECT
Your deliverables:
→ API Architecture Style Decision (REST/GraphQL/gRPC/Hybrid)
   • Justification with trade-off analysis
   • When to use which style (per service/domain)

→ RESTful API Design (if applicable):
   • Resource naming conventions (plural nouns, hierarchies)
   • HTTP verb mapping (GET/POST/PUT/PATCH/DELETE/OPTIONS)
   • HTTP status code strategy (2xx/3xx/4xx/5xx usage)
   • URL structure standards (/api/v1/resources/{id}/subresources)
   • Query parameter conventions (filtering, sorting, pagination)
   • Request/Response envelope format (if any)

→ GraphQL Schema Design (if applicable):
   • Type system design
   • Query/Mutation/Subscription organization
   • Field-level authorization strategy
   • N+1 query prevention (DataLoader strategy)
   • Schema federation approach (if microservices)
   • Pagination approach (cursor vs offset)

→ gRPC Service Design (if applicable):
   • .proto file organization
   • Service definition and RPC methods
   • Message types and field numbering strategy
   • Streaming vs unary RPC decisions
   • Error handling with status codes

→ API Versioning Strategy:
   • Versioning approach (URL path /v1/ vs header vs content negotiation)
   • Deprecation policy and timeline
   • Backward compatibility guarantees
   • Breaking change definition and process
   • Sunset notification mechanism

→ OpenAPI/Swagger Specification:
   • Complete OpenAPI 3.0 or 3.1 spec
   • Every endpoint documented with:
     - Description and use case
     - Request parameters (path, query, header, body)
     - Request body schema (with examples)
     - Response schemas for all status codes (with examples)
     - Authentication/authorization requirements
     - Rate limiting information
   • Comprehensive schema definitions ($ref usage)
   • Example requests and responses (happy path + error cases)

→ API Rate Limiting & Throttling:
   • Rate limit strategy (per user/IP/API key/endpoint)
   • Rate limit algorithms (token bucket, leaky bucket, fixed window)
   • Rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, etc.)
   • Throttling response behavior (429 Too Many Requests)
   • Quota management (daily/monthly limits)

→ API Authentication & Authorization:
   • Authentication scheme (OAuth 2.0, JWT, API Keys, mTLS)
   • Authorization model (RBAC, ABAC, scope-based)
   • Token format and claims
   • Token expiry and refresh strategy
   • Public vs authenticated endpoint designation

📊 DATA CONTRACT SPECIALIST
Your deliverables:
→ Request/Response Schema Design:
   • JSON Schema definitions for all payloads
   • Field naming conventions (camelCase, snake_case, PascalCase)
   • Required vs optional fields policy
   • Null handling strategy (explicit nulls vs omission)
   • Default values policy
   • Data type standards (date formats ISO 8601, currency, locales)

→ Data Validation Rules:
   • Input validation requirements per field
     - String length limits
     - Numeric ranges
     - Regex patterns (email, phone, URLs)
     - Enum allowed values
     - Array size limits
   • Business rule validations
   • Cross-field validation requirements

→ Error Response Format:
   • Standardized error response structure
     {
       "error": {
         "code": "VALIDATION_ERROR",
         "message": "Human-readable message",
         "details": [...],
         "timestamp": "...",
         "requestId": "..."
       }
     }
   • Error code taxonomy
   • Error message guidelines (user-friendly, actionable, no sensitive data)
   • Field-level error reporting for validation failures

→ Pagination Strategy:
   • Pagination approach (offset-based vs cursor-based)
   • Default and maximum page sizes
   • Pagination metadata in responses
     - total_count, page, page_size, next/previous links
   • HATEOAS links (if applicable)

→ Idempotency Design:
   • Idempotency-Key header usage
   • Idempotent operations identification
   • Duplicate request detection window
   • Idempotency storage strategy

→ Webhook Design (if applicable):
   • Webhook event types
   • Webhook payload format
   • Webhook signature/authentication (HMAC)
   • Retry policy for failed webhooks
   • Webhook endpoint registration API

🔄 INTEGRATION ARCHITECT
Your deliverables:
→ Third-Party API Integration Strategy:
   • External APIs to be consumed (list with purpose)
   • Integration patterns per API
     - Direct calls vs queue-based
     - Synchronous vs asynchronous
   • Circuit breaker configuration
   • Timeout and retry policies
   • Fallback/degradation strategies
   • API client library selection

→ BFF (Backend for Frontend) Design:
   • BFF necessity assessment per client type
   • Web BFF API design (if needed)
   • Mobile BFF API design (if needed)
   • Data aggregation and transformation logic
   • BFF-to-microservice communication patterns

→ API Gateway Strategy:
   • API Gateway selection (Kong, AWS API Gateway, Apigee, etc.)
   • Gateway responsibilities:
     - Request routing
     - Authentication/authorization
     - Rate limiting
     - Request/response transformation
     - Caching
     - Logging/monitoring
   • Gateway high availability and failover

→ Inter-Service Communication:
   • Synchronous communication (REST/gRPC) guidelines
   • Asynchronous communication (message queues) guidelines
   • Service mesh consideration (Istio, Linkerd)
   • Service discovery mechanism
   • Load balancing strategy

📱 MOBILE API SPECIALIST
Your deliverables:
→ Mobile-Optimized API Design:
   • Payload size optimization strategies
     - Field filtering (sparse fieldsets)
     - Compression (gzip, brotli)
     - Binary formats consideration (Protocol Buffers)
   • Request batching endpoints (if beneficial)
   • GraphQL benefits for mobile (selective field fetching)

→ Offline & Sync API Design:
   • Optimistic update patterns
   • Conflict resolution strategy (last-write-wins, vector clocks, CRDTs)
   • Delta sync endpoints (fetch only changes since timestamp)
   • Sync status API
   • Offline queue and retry mechanism

→ Mobile-Specific Considerations:
   • Network condition handling (2G/3G/4G/5G)
   • Battery-efficient polling strategies (or push alternatives)
   • Background refresh API constraints
   • App backgrounding state handling
   • Certificate pinning requirements

→ Push Notification API:
   • Device token registration endpoint
   • Push notification preferences API
   • Notification history API
   • Push notification payload standards

🔒 API SECURITY ARCHITECT
Your deliverables:
→ OWASP API Security Top 10 Mitigation:
   • API1: Broken Object Level Authorization
     - Authorization check on EVERY resource access
   • API2: Broken Authentication
     - Strong authentication mechanism
   • API3: Broken Object Property Level Authorization
     - Field-level access control
   • API4: Unrestricted Resource Consumption
     - Rate limiting + pagination + timeouts
   • API5: Broken Function Level Authorization
     - Role checks on every operation
   • API6: Unrestricted Access to Sensitive Business Flows
     - Business logic abuse prevention
   • API7: Server Side Request Forgery (SSRF)
     - URL validation and allowlisting
   • API8: Security Misconfiguration
     - Secure defaults, minimal exposure
   • API9: Improper Inventory Management
     - API inventory and documentation
   • API10: Unsafe Consumption of APIs
     - Third-party API validation

→ API Security Headers:
   • Content-Security-Policy
   • X-Content-Type-Options: nosniff
   • X-Frame-Options: DENY
   • Strict-Transport-Security
   • X-XSS-Protection (deprecated but document decision)

→ Input Sanitization & Validation:
   • Server-side validation requirements (NEVER trust client)
   • SQL injection prevention (parameterized queries mandatory)
   • XSS prevention (output encoding)
   • Command injection prevention
   • XML/JSON injection prevention

→ Sensitive Data Handling in APIs:
   • PII/PHI never in URLs or logs
   • Response filtering (don't return password hashes, tokens)
   • Encryption in transit (TLS 1.3 minimum)
   • Field-level encryption for highly sensitive data

📖 API DOCUMENTATION SPECIALIST
Your deliverables:
→ API Documentation Portal:
   • Interactive API documentation (Swagger UI, Redoc, Stoplight)
   • Getting Started guide
   • Authentication guide
   • Code examples in multiple languages (curl, JavaScript, Python, etc.)
   • Common use case tutorials
   • Rate limiting and quota information
   • Error handling guide
   • Changelog and migration guides

→ API Developer Experience (DX):
   • SDK/Client library availability (or roadmap)
   • Sandbox/playground environment
   • Postman/Insomnia collection
   • API mocking capability for development

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 SENIOR BACKEND ENGINEER CRITIC
Your critique must identify:
→ Are these APIs implementable without heroic effort?
→ Database query implications (N+1 risks, overfetching?)
→ Performance bottlenecks baked into API design?
→ Missing endpoints that will definitely be needed?
→ Endpoints that expose too much or too little?
→ Caching strategy feasibility?
→ Are error scenarios adequately covered?
→ Transaction boundary issues?
→ Specific critique: "Endpoint X will require Y database joins 
   which will kill performance at scale"

🔴 SENIOR FRONTEND ENGINEER CRITIC
Your critique must identify:
→ Developer experience issues (too many API calls needed?)
→ Over-fetching or under-fetching data problems?
→ Missing fields that UI definitely needs?
→ Pagination UX concerns?
→ Real-time update mechanism missing?
→ Error messages - are they actionable from UI perspective?
→ API versioning - how will frontend handle breaking changes?
→ Specific critique: "To render Screen X, frontend must make 
   7 API calls - this should be 1 aggregated endpoint"

🔴 SENIOR MOBILE ENGINEER CRITIC (iOS + Android)
Your critique must identify:
→ Payload size going to murder mobile data plans?
→ Chatty API pattern causing battery drain?
→ Offline-first scenarios inadequately addressed?
→ Background refresh limitations not considered?
→ Network resilience patterns missing?
→ Platform-specific constraints violated?
→ Specific critique: "This polling approach will drain battery 
   on iOS - needs push or efficient long-polling"

🔴 OWASP API SECURITY EXPERT CRITIC
Your critique must identify:
→ OWASP API Top 10 violations (be specific - which one, where?)
→ Authorization bypass opportunities
→ Rate limiting too generous or missing on critical endpoints?
→ Sensitive data exposure in responses?
→ IDOR (Insecure Direct Object Reference) vulnerabilities?
→ Mass assignment vulnerabilities?
→ Authentication weaknesses?
→ Specific critique: "Endpoint GET /users/{id} doesn't verify 
   requesting user has permission to access user {id} - IDOR vulnerability"

🔴 PERFORMANCE ENGINEER CRITIC
Your critique must identify:
→ Endpoints that will be slow (> 200ms) at scale?
→ Missing caching opportunities (HTTP cache headers)?
→ Database indexes required but not specified?
→ Pagination missing on potentially large result sets?
→ Synchronous operations that should be async?
→ Request/response payload bloat?
→ Specific critique: "Endpoint /products requires full-text 
   search across 3 tables without specifying index strategy"

🔴 DEVOPS/SRE CRITIC
Your critique must identify:
→ APIs that are unmonitorable (missing observability hooks)?
→ Deployment and versioning challenges?
→ Breaking change impact analysis missing?
→ Health check and readiness probe endpoints defined?
→ Graceful degradation strategy absent?
→ Documentation generation integrated into CI/CD?
→ Specific critique: "No /health or /ready endpoint specified 
   for Kubernetes liveness/readiness probes"

🔴 QA/TEST ENGINEER CRITIC
Your critique must identify:
→ Untestable API behaviors (non-deterministic responses)?
→ Test data management challenges?
→ Missing test environments or sandbox mode?
→ Contract testing strategy absent?
→ API mocking challenges?
→ Idempotency not verifiable?
→ Specific critique: "DELETE operations have no safe test mode - 
   will destroy real data in testing"

🔴 API CONSUMER (DEVELOPER EXPERIENCE) CRITIC
Your critique must identify:
→ Confusing or inconsistent API patterns?
→ Documentation gaps that will cause support tickets?
→ Missing SDK or client libraries?
→ Poor error messages (vague, not actionable)?
→ Difficult authentication flow?
→ Versioning and migration path unclear?
→ Specific critique: "Error code E_VAL_001 is meaningless - 
   developers won't know what's wrong or how to fix it"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Each critic scores across these dimensions:

→ SECURITY: /10
   Criteria: OWASP API Top 10 coverage, auth/authz robustness,
   data exposure prevention, injection attack prevention

→ PERFORMANCE: /10
   Criteria: Response time viability, scalability, payload efficiency,
   caching strategy, database query efficiency

→ DEVELOPER EXPERIENCE: /10
   Criteria: Consistency, intuitiveness, documentation quality,
   error message quality, ease of integration

→ COMPLETENESS: /10
   Criteria: All required endpoints present, edge cases covered,
   error scenarios handled, versioning strategy solid

→ MAINTAINABILITY: /10
   Criteria: Evolvability, backward compatibility approach,
   deprecation strategy, documentation as code

→ COMPOSITE SCORE: Average of all dimensions (must be ≥ 9.0)

Each score MUST include:
   Score: [X/10]
   Justification: [Why this specific score]
   Critical Issues: [Blockers - must be resolved]
   Major Issues: [Significant concerns - should be resolved]
   Minor Issues: [Improvements - nice to have]
   Conditions for 9/10: [Exactly what must change]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Builder Panel addresses EVERY critique point systematically:

For each Critical Issue:
→ Issue ID: [Critic Name - Issue #]
→ Original Critique: [Exact quote]
→ Remediation Action: [Specific change made]
→ Verification: [How this resolves the issue]

For each Major Issue:
→ [Same structure as Critical]

For each Minor Issue:
→ Action: [Resolved / Documented as accepted risk / Deferred to v2]
→ Justification: [If not resolved, why]

[REPEAT STEPS 2-3 UNTIL ALL CRITICS SCORE ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 1.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. OpenAPI 3.0/3.1 Specification (Complete)
   - All endpoints documented with examples
   - Schema definitions for all request/response types
   - Authentication/authorization documented
   - Error responses documented

2. GraphQL Schema Definition (if applicable)
   - Complete SDL (Schema Definition Language)
   - Resolver documentation
   - Authorization directive documentation

3. gRPC Proto Files (if applicable)
   - All .proto files with comments
   - Service definitions
   - Message type definitions

4. API Design Guidelines Document
   - Naming conventions
   - Versioning strategy
   - Pagination standards
   - Error handling standards
   - Rate limiting policy

5. API Security Review Document
   - OWASP API Top 10 checklist (completed)
   - Authentication/authorization architecture
   - Security headers configuration
   - Input validation standards

6. API Developer Portal Content
   - Getting Started guide
   - Authentication guide
   - Code examples (multiple languages)
   - Error handling guide
   - Postman/Insomnia collection

7. API Versioning & Deprecation Policy
   - Versioning scheme
   - Breaking change process
   - Deprecation timeline
   - Migration guides

8. Mobile API Optimization Strategy
   - Payload optimization techniques
   - Offline/sync design
   - Battery/bandwidth considerations

9. Integration Architecture Document
   - Third-party API integration patterns
   - BFF design (if applicable)
   - API Gateway configuration
   - Circuit breaker policies

10. Unanimous Score Documentation
    - All critic scores ≥ 9/10
    - All critique points addressed
    - Sign-off from all experts

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 1.5 COMPLETE when:
   → All critics score ≥ 9/10 with documented justification
   → All Critical and Major issues resolved
   → All deliverables produced and reviewed
   → OpenAPI spec validates (no schema errors)
   → Security review sign-off obtained
   → API contracts locked and versioned

🚫 Phase 1.5 BLOCKED if:
   → Any critic scores < 9/10
   → Critical security issues remain
   → API contracts incomplete or ambiguous
   → Performance concerns unresolved
   → Documentation gaps exist

[ONLY AFTER UNANIMOUS 9/10+ → PROCEED TO NEXT PHASE]
```

---

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 2 — Architecture & System Design** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
