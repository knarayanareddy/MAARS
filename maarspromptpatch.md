# 📋 COMPLETE CONSOLIDATED PROMPT TEMPLATES
## For All Missing SDLC Phases

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

## 📋 PHASE 2.5 — DATABASE DESIGN & DATA MODELING PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 2.5: DATABASE DESIGN & DATA MODELING                  ║
║  Goal: Design a database that won't crumble under load       ║
║  Input: Phase 0, 1, 1.5, 2 Deliverables                     ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

📊 SENIOR DATABASE ARCHITECT
Your deliverables:

→ Database Technology Selection (Per Service/Domain):
   • Relational (PostgreSQL, MySQL, SQL Server, etc.)
   • Document Store (MongoDB, Couchbase)
   • Key-Value (Redis, DynamoDB)
   • Column-Family (Cassandra, HBase)
   • Graph (Neo4j, Amazon Neptune)
   • Time-Series (TimescaleDB, InfluxDB)
   • Search (Elasticsearch, OpenSearch)
   • Selection justification for each (polyglot persistence rationale)

→ Normalized Entity-Relationship Diagram (ERD):
   • All entities with attributes
   • Primary keys designated
   • Foreign keys and relationships (1:1, 1:M, M:M)
   • Cardinality and optionality notation
   • Normalization level (3NF/BCNF) with justification
   • Denormalization decisions with specific rationale

→ Complete Table Specifications:
   For EACH table:
   • Table name (with naming convention rationale)
   • Column definitions:
     - Column name
     - Data type (with size/precision)
     - NOT NULL constraints
     - DEFAULT values
     - CHECK constraints
     - UNIQUE constraints
     - Comments/descriptions
   • Primary key definition (simple or composite)
   • Foreign key constraints:
     - Referenced table and column
     - ON DELETE behavior (CASCADE, SET NULL, RESTRICT)
     - ON UPDATE behavior
   • Unique constraints (beyond PK)
   • Table-level CHECK constraints

→ Indexing Strategy:
   For EACH table:
   • Clustered index (if applicable)
   • Non-clustered indexes with:
     - Index name
     - Indexed columns (order matters!)
     - Included columns (covering index strategy)
     - Filtered index conditions (if partial index)
     - Justification (which queries does this optimize?)
   • Full-text search indexes (if needed)
   • Spatial indexes (if geodata)
   • Index maintenance strategy (rebuild/reorganize thresholds)

→ Data Type Standards:
   • GUID/UUID usage policy
   • DATETIME vs TIMESTAMP decisions
   • VARCHAR size standards
   • DECIMAL precision for currency
   • ENUM vs lookup table decisions
   • JSON/JSONB column usage policy
   • Binary data storage approach (BLOB vs file storage)

→ Schema Versioning & Migration:
   • Migration tool selection (Flyway, Liquibase, Alembic, etc.)
   • Migration file organization
   • Rollback strategy for each migration
   • Zero-downtime migration patterns
     - Expand-contract pattern
     - Blue-green database approach
     - Shadow tables/triggers for live migrations

🗄️ DATABASE ADMINISTRATOR (DBA)
Your deliverables:

→ Partitioning Strategy:
   • Horizontal partitioning (sharding):
     - Shard key selection with rationale
     - Sharding algorithm (hash, range, list)
     - Number of shards (current and growth plan)
     - Cross-shard query strategy
     - Shard rebalancing approach
   • Vertical partitioning:
     - Table column splitting rationale
     - Hot/cold data separation
   • Table partitioning (within database):
     - Partition key (date ranges, hash ranges)
     - Partition maintenance (auto-creation, archival)

→ Replication Topology:
   • Replication type:
     - Primary-Replica (single write leader)
     - Multi-Primary (conflict resolution needed)
     - Chain replication
   • Number and location of replicas
   • Read replica usage strategy (read load distribution)
   • Replication lag monitoring and alerting thresholds
   • Failover procedure:
     - Automatic vs manual
     - Promotion criteria
     - Downtime estimate
     - Data loss exposure (RPO)

→ Backup and Restore Strategy:
   • Backup types:
     - Full backups (frequency: daily, weekly?)
     - Incremental backups (frequency: hourly?)
     - Differential backups
     - Point-in-time recovery (PITR) capability
   • Backup retention policy (7 days, 30 days, 90 days, etc.)
   • Backup storage location (on-site, off-site, multi-region)
   • Backup encryption requirements
   • Backup validation (automated restore testing frequency)
   • Recovery Time Objective (RTO): [X minutes/hours]
   • Recovery Point Objective (RPO): [X minutes/hours]
   • Disaster recovery runbook

→ Archive and Purge Strategy:
   • Data retention policy per table/entity
   • Archival triggers (age-based, size-based)
   • Archive storage destination (cold storage, data lake)
   • Purge schedule for truly deletable data
   • Compliance considerations (GDPR right to deletion)
   • Archive query capability (how to access old data if needed)

→ Database Security Hardening:
   • Encryption at rest:
     - TDE (Transparent Data Encryption) configuration
     - Key management (KMS integration)
   • Encryption in transit:
     - SSL/TLS enforcement
     - Certificate management
   • Column-level encryption (for PII/PHI)
   • Database user management:
     - Principle of least privilege
     - Service account policies
     - Password rotation policy
   • Network security:
     - Private subnet placement
     - Security group/firewall rules
     - IP allowlisting
   • Audit logging:
     - What gets logged (DDL, DML, login attempts)
     - Audit log retention
     - SIEM integration

→ Database Configuration & Tuning:
   • Connection pool settings:
     - Min/max connections
     - Connection timeout
     - Idle connection timeout
   • Memory allocation:
     - Buffer pool size
     - Query cache size
     - Sort buffer size
   • Transaction isolation level defaults
   • Query timeout settings
   • Slow query logging threshold
   • Autovacuum settings (PostgreSQL)
   • Read/write split configuration

⚡ DATABASE PERFORMANCE ENGINEER
Your deliverables:

→ Query Optimization Strategy:
   • Slow query identification process
   • Explain/execution plan analysis standards
   • Query anti-pattern identification:
     - SELECT * usage policy
     - N+1 query prevention
     - Missing WHERE clause on large tables
     - Cartesian products
     - Correlated subqueries
   • Query rewriting guidelines
   • Stored procedure vs application logic decisions

→ Transaction Design:
   • Transaction boundary guidelines
   • Long-running transaction prevention
   • Deadlock prevention strategies:
     - Lock ordering conventions
     - Lock timeout settings
   • Optimistic vs pessimistic locking decisions
   • Transaction isolation level per use case:
     - READ UNCOMMITTED (rarely acceptable)
     - READ COMMITTED
     - REPEATABLE READ
     - SERIALIZABLE
   • ACID guarantees vs eventual consistency trade-offs

→ Database Performance Monitoring:
   • Metrics to track:
     - Query latency (p50, p95, p99)
     - Connection pool utilization
     - Cache hit ratio
     - Replication lag
     - Disk I/O utilization
     - CPU and memory usage
     - Lock contention
     - Transaction throughput
   • Alerting thresholds for each metric
   • Performance baseline establishment

→ Caching Strategy (Database Level):
   • Query result caching
   • Materialized views:
     - Which aggregations/joins get materialized
     - Refresh strategy (on-demand, scheduled, incremental)
   • Database-level caching (Redis/Memcached integration)
   • Cache invalidation strategy
   • Cache warming approach

🔄 DATA MIGRATION ARCHITECT (if migrating from existing system)
Your deliverables:

→ Source System Analysis:
   • Source database inventory
   • Source schema documentation
   • Data volume assessment (row counts per table)
   • Data quality assessment (nulls, duplicates, orphans)
   • Legacy data model analysis

→ Data Mapping Document:
   • Source-to-target table mapping
   • Source-to-target column mapping
   • Data type conversions
   • Data transformation rules:
     - Denormalization/normalization changes
     - Calculated fields
     - Data cleansing rules
   • Data merge/split logic
   • Orphaned record handling

→ Migration Strategy:
   • Migration approach:
     - Big Bang (cut-over weekend)
     - Phased migration (module by module)
     - Parallel run (dual write during transition)
   • Downtime window requirement
   • Rollback criteria and procedure
   • Data validation checkpoints
   • Go/no-go decision criteria

→ ETL Pipeline Design (covered more in Phase 6.5):
   • Extraction approach
   • Transformation logic
   • Load strategy (bulk insert, upsert)
   • Error handling and logging

📐 DATA MODELER (Conceptual & Logical Modeling)
Your deliverables:

→ Conceptual Data Model:
   • High-level entities and relationships
   • Business rules captured
   • Stakeholder-friendly visualization

→ Logical Data Model:
   • Technology-agnostic detailed model
   • All attributes defined
   • Normalization applied
   • Referential integrity rules

→ Physical Data Model:
   • Technology-specific implementation
   • Performance optimizations applied
   • Storage considerations included

→ Data Dictionary:
   • Every table documented:
     - Purpose and business context
     - Ownership (which team/service)
   • Every column documented:
     - Business definition
     - Allowed values/ranges
     - Source of truth
     - PII/PHI classification

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 SENIOR BACKEND ENGINEER CRITIC
Your critique must identify:
→ ORM compatibility issues (impedance mismatch risks?)
→ Schema changes that will break existing code?
→ Missing tables/columns that application layer needs?
→ Foreign key constraints too restrictive for business logic?
→ Transaction boundary issues with application design?
→ Missing database functions/procedures needed?
→ Specific critique: "User table missing 'email_verified_at' 
   column which authentication flow requires"

🔴 DBA CRITIC (Peer Review)
Your critique must identify:
→ Scalability ceiling (when will this design break?)
→ Index bloat risks (over-indexing?)
→ Missing indexes (queries that will be slow?)
→ Partition strategy flaws?
→ Replication lag risks?
→ Backup/restore time exceeding RTO?
→ Migration complexity underestimated?
→ Specific critique: "Orders table will hit 100M rows in 6 months 
   but no partitioning strategy - query performance will degrade"

🔴 SECURITY ARCHITECT CRITIC
Your critique must identify:
→ PII/PHI columns not encrypted?
→ Audit logging gaps (missing who/when for sensitive changes?)
→ Overly permissive database user privileges?
→ Password/secret storage inadequate (plain text, weak hashing)?
→ SQL injection opportunities in schema design?
→ Data exposure in logs (sensitive columns in debug logs?)
→ Specific critique: "SSN column has no column-level encryption 
   - violates compliance requirements"

🔴 COMPLIANCE OFFICER CRITIC
Your critique must identify:
→ GDPR right-to-deletion implementation missing?
→ Data retention policy not enforceable in schema?
→ PII identification incomplete?
→ Audit trail inadequate for compliance?
→ Data residency constraints not addressed?
→ Consent tracking missing from schema?
→ Specific critique: "No 'deleted_at' or soft-delete mechanism 
   to comply with GDPR right to deletion while maintaining referential integrity"

🔴 PERFORMANCE ENGINEER CRITIC
Your critique must identify:
→ Queries that will require full table scans?
→ JOIN operations that will be expensive at scale?
→ Missing covering indexes?
→ Hot partition risks (uneven data distribution)?
→ Lock contention scenarios?
→ N+1 query patterns enabled by schema design?
→ Specific critique: "JOIN between Users and Orders will scan 
   100M rows without composite index on (user_id, created_at)"

🔴 DATA SCIENTIST CRITIC (if applicable)
Your critique must identify:
→ Analytics query patterns not optimized?
→ Missing materialized views for reporting?
→ Data warehouse ETL challenges?
→ Historical data tracking inadequate?
→ Slowly Changing Dimension (SCD) handling missing?
→ Specific critique: "No created_at/updated_at timestamps on 
   Product table - impossible to do time-series analysis"

🔴 SITE RELIABILITY ENGINEER (SRE) CRITIC
Your critique must identify:
→ Database monitoring gaps?
→ Disaster recovery testing not planned?
→ Failover procedure untested?
→ Backup validation missing?
→ Runbook for database incidents incomplete?
→ Observability instrumentation missing?
→ Specific critique: "RTO is 1 hour but restore from backup 
   takes 3 hours with 500GB database - unrealistic target"

🔴 APPLICATION ARCHITECT CRITIC
Your critique must identify:
→ Schema doesn't support required application features?
→ Multi-tenancy isolation inadequate?
→ Eventual consistency challenges?
→ Caching strategy gaps?
→ Microservice data ownership boundaries unclear?
→ Specific critique: "Multi-tenant design uses tenant_id column 
   but no Row-Level Security policies - tenant data isolation at risk"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ SCALABILITY: /10
   Will this handle 10x, 100x, 1000x growth?

→ PERFORMANCE: /10
   Will queries be fast? Are indexes optimal?

→ SECURITY: /10
   Is sensitive data protected? Audit logging adequate?

→ RELIABILITY: /10
   Can we restore from disasters? Is replication solid?

→ MAINTAINABILITY: /10
   Can we evolve schema? Are migrations safe?

→ COMPLIANCE: /10
   Does this meet regulatory requirements?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

[Standard scoring format with justifications]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 2.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Database Technology Selection Document
   - Chosen database per domain/service
   - Justification and trade-off analysis

2. Entity-Relationship Diagrams (ERD)
   - Conceptual ERD
   - Logical ERD (normalized)
   - Physical ERD (with denormalizations)

3. Complete DDL Scripts
   - CREATE TABLE statements (all tables)
   - CREATE INDEX statements (all indexes)
   - CONSTRAINT definitions
   - Comments and documentation

4. Database Schema Documentation
   - Data dictionary (all tables and columns)
   - Business rules captured
   - PII/PHI classification

5. Indexing Strategy Document
   - All indexes with justifications
   - Index maintenance plan

6. Partitioning & Sharding Design
   - Partition strategy (if applicable)
   - Shard key and algorithm (if applicable)

7. Replication & HA Architecture
   - Replication topology diagram
   - Failover procedures
   - Replication lag monitoring plan

8. Backup & Disaster Recovery Plan
   - Backup schedule and retention
   - Restore procedures (tested)
   - RTO/RPO targets with validation

9. Database Security Hardening Checklist
   - Encryption configuration
   - User privilege matrix
   - Audit logging configuration

10. Performance Monitoring Plan
    - Metrics to track
    - Alert thresholds
    - Performance baseline

11. Migration Plan (if applicable)
    - Source-to-target mapping
    - Migration strategy
    - Rollback procedure

12. Unanimous Score Documentation
    - All critic scores ≥ 9/10
    - All issues addressed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 2.5 COMPLETE when:
   → All critics score ≥ 9/10
   → DDL scripts validated (no syntax errors)
   → ERDs reviewed and approved
   → Security review sign-off
   → Performance analysis complete
   → Backup/restore tested in non-prod

🚫 Phase 2.5 BLOCKED if:
   → Any critic scores < 9/10
   → Scalability concerns unresolved
   → Security vulnerabilities present
   → Compliance gaps exist
   → RTO/RPO targets unachievable

[ONLY AFTER UNANIMOUS 9/10+ → PROCEED TO NEXT PHASE]
```

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

## 📋 PHASE 6.5 — DATA MIGRATION & SEEDING PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 6.5: DATA MIGRATION & SEEDING                         ║
║  Goal: Move/seed data safely without loss or corruption      ║
║  Input: Phase 2.5 (DB Design) + Source System Analysis       ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🗄️ DATA MIGRATION LEAD
Your deliverables:

→ Migration Strategy Document:
   • Migration type selection with justification:
     
     Option A: BIG BANG MIGRATION
     - Complete cutover in single maintenance window
     - Downtime duration: [X hours]
     - Rollback window: [X hours]
     - Risk assessment: [High/Medium/Low]
     - Use case: Suitable when total data size allows migration 
       within acceptable downtime window
     
     Option B: PHASED MIGRATION
     - Module/domain-by-module migration
     - Phase breakdown (e.g., Phase 1: Users, Phase 2: Orders, etc.)
     - Each phase timeline and success criteria
     - Inter-phase dependencies
     - Use case: Large systems where big bang is too risky
     
     Option C: PARALLEL RUN (Dual Write)
     - Write to both old and new systems simultaneously
     - Shadow mode duration: [X weeks]
     - Reconciliation frequency: [Hourly/Daily]
     - Cutover decision criteria
     - Use case: Zero-downtime requirement, need validation period
     
     Option D: TRICKLE MIGRATION (Lazy Migration)
     - Migrate data on-demand as accessed
     - Background batch migration for unaccessed data
     - Hybrid source (old DB + new DB) strategy
     - Use case: Massive datasets where full migration upfront impractical

   • Chosen strategy: [A/B/C/D]
   • Justification: [Why this approach for this project]
   • Risk mitigation for chosen strategy

→ Pre-Migration Assessment:
   • Source data volume analysis:
     - Total row count per table
     - Total data size (GB/TB)
     - Data growth rate (rows/day)
     - Largest tables identified
   
   • Source data quality analysis:
     - NULL value percentage per column
     - Duplicate record analysis
     - Orphaned record count (FK violations)
     - Data format inconsistencies
     - Character encoding issues
     - Date/time format variations
     - Invalid data patterns
   
   • Data quality score: [X/10]
   • Data cleansing requirements identified

→ Migration Timeline:
   • Preparation phase:
     - Start date: [Date]
     - Activities: Schema creation, test data migration, validation
     - Duration: [X weeks]
   
   • Dry run phase:
     - Test migration executions: [Count]
     - Issues discovered and resolved
     - Performance tuning
     - Duration: [X weeks]
   
   • Production migration phase:
     - Maintenance window: [Date + Time + Duration]
     - Migration execution: [X hours]
     - Validation: [X hours]
     - Contingency buffer: [X hours]
   
   • Post-migration stabilization:
     - Monitoring period: [X days]
     - Rollback window: [X hours/days]
     - Decommission old system: [Date]

→ Rollback Plan:
   • Rollback decision criteria (Go/No-Go):
     - Data loss detected: [threshold]
     - Data corruption percentage: [threshold]
     - Migration time exceeds: [X hours]
     - Critical validation failures: [count threshold]
     - Application errors post-migration: [threshold]
   
   • Rollback procedure (step-by-step):
     1. [Stop new system]
     2. [Restore database from pre-migration backup]
     3. [Revert application deployment]
     4. [Restart old system]
     5. [Verify old system operational]
     6. [Customer communication]
   
   • Rollback time estimate: [X hours]
   • Data loss exposure during rollback: [X minutes of transactions]
   • Rollback testing results: [Tested on: Date, Result: Pass/Fail]

→ Stakeholder Communication Plan:
   • Pre-migration notifications:
     - Internal teams: [X days before]
     - Customers: [X days before]
     - Communication channels: [Email, in-app, status page]
   
   • During migration:
     - Status updates frequency: [Every X hours]
     - Escalation contacts
     - War room/bridge line setup
   
   • Post-migration:
     - Success announcement
     - Issue reporting channel
     - Support team briefing

⚙️ ETL ENGINEER
Your deliverables:

→ Extraction Strategy:
   • Source database connection strategy:
     - Read replica usage (avoid production load)
     - Connection pooling configuration
     - Query timeout settings
   
   • Extraction approach per table:
     - Full table scan vs incremental (WHERE created_at > ?)
     - Batch size (10K rows, 100K rows, etc.)
     - Pagination strategy
     - Extraction order (respect FK dependencies)
   
   • Extraction performance optimization:
     - Parallel extraction (multiple tables simultaneously)
     - Compression during transfer
     - Network bandwidth considerations
   
   • Extraction logging:
     - Progress tracking (% complete per table)
     - Row count checkpoints
     - Error logging

→ Transformation Logic:
   For EACH table/entity transformation:
   
   • Data mapping specification:
     - Source table → Target table
     - Source column → Target column (with data type conversion)
     - Derived/calculated fields (with formulas)
     - Constant values inserted
     - Lookup/reference data resolution
   
   • Data cleansing transformations:
     - NULL handling (default values, skip row, error)
     - Duplicate resolution (keep first, keep last, merge)
     - Orphaned record handling (create placeholder, skip, error)
     - Data format standardization:
       * Phone number formats: (555) 123-4567 → +15551234567
       * Date formats: MM/DD/YYYY → YYYY-MM-DD
       * Case normalization: Mixed Case → lowercase
     - Invalid data handling (log and skip, use default, error)
   
   • Business rule transformations:
     - Status code mapping (old values → new values)
     - Category/type mapping
     - Unit conversions
     - Currency conversions
   
   • Data enrichment:
     - Geocoding addresses
     - Deriving fields (full_name from first + last)
     - Adding metadata (migration_date, source_system_id)
   
   • Transformation code/scripts:
     - Language/tool: [Python/Pandas, SQL, Apache Spark, etc.]
     - Version controlled in: [Git repository path]
     - Code review completed: [Yes/No, Reviewer: Name]

→ Load Strategy:
   • Target database load approach:
     - Bulk insert (COPY, LOAD DATA INFILE, bulk API)
     - Batch inserts (INSERT multiple rows)
     - Individual inserts
     - Upsert (INSERT ... ON CONFLICT UPDATE)
   
   • Load performance optimization:
     - Disable indexes during load, rebuild after
     - Disable foreign key constraints during load, re-enable after
     - Disable triggers during load
     - Increase database write buffer size
     - Parallel loading (multiple tables/partitions)
   
   • Load order:
     - Parent tables before child tables (FK dependencies)
     - Reference/lookup data first
     - Transactional data last
   
   • Load validation checkpoints:
     - Row count validation after each table
     - Data integrity checks (FK constraints re-enabled successfully)
     - Sample data spot checks

→ Incremental Load Strategy (for ongoing sync if parallel run):
   • Change data capture (CDC):
     - CDC mechanism (DB triggers, transaction log, timestamp column)
     - Change identification query
     - CDC polling frequency
   
   • Delta load process:
     - Identify new/updated/deleted records since last sync
     - Conflict resolution (if same record modified in both systems)
     - Delta apply strategy (merge, overwrite)

→ Error Handling & Recovery:
   • Error classification:
     - Fatal errors (stop entire migration):
       * Database connection lost
       * Disk space exhausted
       * Data corruption detected
     - Non-fatal errors (log and continue):
       * Individual row transformation failure
       * Optional field validation failure
     - Warning (log only):
       * Data quality issues (unexpected NULL, etc.)
   
   • Error logging:
     - Structured error log format
     - Error details: timestamp, table, row ID, error type, message
     - Failed row data captured (for manual review)
   
   • Recovery mechanism:
     - Checkpointing (save progress every X rows)
     - Resume from last checkpoint
     - Retry logic (transient errors: network timeout, etc.)
     - Manual intervention queue (for errors requiring human decision)

→ ETL Pipeline Code:
   • Complete pipeline implementation
   • Configuration files (source DB, target DB, credentials, etc.)
   • Parameterized execution (dry-run vs production mode)
   • Logging and monitoring instrumentation
   • Performance metrics tracking (rows/sec, GB/hour)

✅ DATA QUALITY ENGINEER
Your deliverables:

→ Pre-Migration Data Quality Assessment:
   • Data profiling report (source system):
     - Column completeness (% non-NULL)
     - Value distribution analysis
     - Outlier detection
     - Format consistency
     - Referential integrity violations
   
   • Data quality issues catalog:
     - Issue ID, Table, Column, Issue Type, Count, Severity, Remediation
     - Example: DQ-001, Users, email, Invalid format, 234 rows, Medium, Apply regex fix
   
   • Data cleansing plan:
     - Which issues will be fixed during migration
     - Which issues accepted as-is (with justification)
     - Which issues require pre-migration cleanup in source

→ Data Validation & Reconciliation Plan:
   • Validation checkpoints:
     
     CHECKPOINT 1: Row Count Validation
     - Compare row counts: source vs target (per table)
     - Acceptance criteria: 100% match or documented exceptions
     
     CHECKPOINT 2: Aggregate Validation
     - Compare aggregates (SUM, COUNT, AVG) on key metrics
     - Example: SUM(order_total) in source = SUM(order_total) in target
     
     CHECKPOINT 3: Sample Data Validation
     - Random sample (e.g., 1000 rows per table)
     - Field-by-field comparison
     - Acceptance criteria: 100% match
     
     CHECKPOINT 4: Referential Integrity Validation
     - All foreign keys resolve (no orphaned records)
     - All constraints pass
     
     CHECKPOINT 5: Business Rule Validation
     - Business logic assertions hold
     - Example: All active users have at least one order
     
     CHECKPOINT 6: Application Smoke Test
     - Critical user journeys work with migrated data
     - Reports generate correctly

   • Reconciliation reports:
     - Summary report (overall migration success metrics)
     - Detailed discrepancy report (row-level differences)
     - Reconciliation sign-off criteria

→ Post-Migration Data Quality Monitoring:
   • Data quality metrics tracked for [X days] post-migration:
     - Data completeness trends
     - Duplicate record creation rate
     - Referential integrity violation rate
     - User-reported data issues
   
   • Monitoring dashboard
   • Alerting thresholds for data quality degradation

🧪 MIGRATION TEST ENGINEER
Your deliverables:

→ Test Migration Execution Plan:
   • Test environment setup:
     - Source database: [Production snapshot or anonymized copy]
     - Target database: [Test instance]
     - Data volume: [Full dataset or representative sample]
   
   • Test migration execution schedule:
     - Test run 1: [Date] - Focus: End-to-end process validation
     - Test run 2: [Date] - Focus: Performance tuning
     - Test run 3: [Date] - Focus: Rollback procedure validation
     - Test run 4: [Date] - Focus: Production dress rehearsal
   
   • Success criteria per test run:
     - All data migrated successfully
     - Validation checkpoints pass
     - Performance targets met (migration completes in X hours)
     - Rollback procedure works

→ Test Migration Results Documentation:
   For EACH test run:
   • Execution date and duration
   • Data volume migrated
   • Issues encountered (with severity)
   • Performance metrics (rows/sec, bottlenecks)
   • Validation results
   • Lessons learned and improvements made
   • Go/No-Go recommendation for production

→ Rollback Testing:
   • Rollback procedure executed in test: [Date]
   • Rollback duration: [X hours]
   • Data loss during rollback: [X minutes of transactions]
   • Issues during rollback: [List]
   • Rollback procedure refined: [Yes/No]

🌱 DATABASE SEEDING SPECIALIST
Your deliverables:

→ Reference Data Seeding Strategy:
   • Reference data identification:
     - Country/state/city lists
     - Product categories
     - User roles and permissions
     - Configuration settings
     - Lookup tables (status codes, types, etc.)
   
   • Seeding approach:
     - SQL seed scripts (INSERT statements)
     - Seed data version control (Git)
     - Idempotent seeding (can run multiple times safely)
     - Environment-specific seeds (dev/staging/prod differences)
   
   • Seed data maintenance:
     - Update process when reference data changes
     - Seed script testing in CI/CD

→ Test Data Generation (Non-Production Environments):
   • Synthetic test data generation:
     - Realistic but fake user data (Faker library, etc.)
     - Realistic transaction patterns
     - Edge case data (boundary values, special characters)
     - Performance test data (large volumes)
   
   • Test data anonymization (from production):
     - PII anonymization strategy:
       * Email: real@domain.com → user123@example.com
       * Name: John Doe → User 123
       * Phone: 555-1234 → randomized
       * SSN/Credit Card: fully masked or tokenized
     - Preserve referential integrity during anonymization
     - Preserve statistical properties (for testing)
   
   • Test data refresh strategy:
     - Frequency: [Weekly, Monthly]
     - Automated refresh pipeline

→ Production Initial Data Seeding:
   • Initial users/accounts (if applicable):
     - Admin users
     - System accounts
     - Test accounts (if in production)
   
   • Initial configuration
   • Initial content (if CMS/content platform)

🔒 SECURITY & COMPLIANCE OFFICER (Migration Context)
Your deliverables:

→ Data Security During Migration:
   • Data in transit encryption:
     - SSL/TLS for database connections
     - Encrypted file transfer (if file-based migration)
     - VPN/private network for migration traffic
   
   • Data at rest encryption:
     - Source backup encryption
     - Target database encryption
     - Temporary storage encryption (if ETL uses intermediate storage)
   
   • Access control during migration:
     - Who has access to migration tools
     - Credentials management (vaulted, rotated)
     - Audit logging of all access

→ Compliance Considerations:
   • PII/PHI handling:
     - Data minimization (only migrate necessary data)
     - Data retention compliance (don't migrate expired data)
     - Cross-border data transfer (if applicable)
   
   • Regulatory requirements:
     - GDPR: Right to deletion honored before migration
     - HIPAA: Audit trail of PHI access during migration
     - PCI-DSS: Cardholder data handling
   
   • Data breach notification prep:
     - If migration exposes data, notification plan ready

→ Data Disposal (Old System):
   • Decommissioning plan for old system:
     - Decommission date: [Date]
     - Data retention period in old system: [X days post-migration]
     - Secure data deletion procedure:
       * Backup deletion
       * Database secure wipe
       * File system secure deletion
     - Compliance with data retention laws
   
   • Certification of destruction: [Yes/No]

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 DATABASE ADMINISTRATOR (DBA) CRITIC
Your critique must identify:
→ Migration duration realistic? (Can it complete in maintenance window?)
→ Database performance during migration (will it lock tables, block reads?)
→ Index/constraint disabling strategy risky?
→ Backup strategy before migration adequate?
→ Disk space requirements calculated correctly?
→ Network bandwidth sufficient for data transfer?
→ Specific critique: "Migration script disables all indexes but doesn't 
   account for 8 hours to rebuild B-tree indexes on 500M row table - 
   will exceed maintenance window"

🔴 SRE CRITIC
Your critique must identify:
→ Monitoring during migration - how will we know if it's failing?
→ Rollback procedure tested under realistic conditions?
→ Rollback time estimate realistic?
→ Data loss during rollback acceptable?
→ Communication plan during migration adequate?
→ Runbook for migration execution detailed enough?
→ Specific critique: "Rollback requires restoring 2TB database from backup 
   but restore testing shows 6 hour restore time - exceeds stated 2 hour 
   rollback window"

🔴 SECURITY CRITIC
Your critique must identify:
→ PII/PHI exposure during migration?
→ Migration credentials overly permissive?
→ Logging captures sensitive data?
→ Temporary data storage insecure?
→ Access audit trail complete?
→ Specific critique: "ETL pipeline logs entire row data including SSN 
   and credit cards to plaintext log file - security violation"

🔴 COMPLIANCE OFFICER CRITIC
Your critique must identify:
→ Data retention policy violations?
→ Cross-border data transfer compliance gaps?
→ Right to deletion honored before migration?
→ Audit trail for compliance requirements?
→ Data destruction procedure compliant?
→ Specific critique: "Migration includes user data from EU users who 
   requested deletion 90 days ago - GDPR violation"

🔴 APPLICATION ARCHITECT CRITIC
Your critique must identify:
→ Application compatibility with migrated data?
→ Data schema changes breaking existing code?
→ Dual-write strategy (if parallel run) creating data inconsistencies?
→ Application deployment synchronized with migration?
→ Specific critique: "Migrated data uses new ENUM values but application 
   code still expects old values - will cause runtime errors"

🔴 DATA QUALITY CRITIC
Your critique must identify:
→ Data validation insufficient?
→ Reconciliation thresholds too lenient?
→ Data cleansing rules introducing new errors?
→ Post-migration data quality monitoring missing?
→ Specific critique: "Only validating row counts - not detecting data 
   corruption within rows (e.g., swapped columns)"

🔴 BUSINESS STAKEHOLDER CRITIC
Your critique must identify:
→ Downtime impact on business operations?
→ Customer communication adequate?
→ Business continuity during migration?
→ Rollback impact on customers?
→ Specific critique: "Migration scheduled during peak business hours in 
   APAC region - should be moved to APAC off-hours"

🔴 PERFORMANCE ENGINEER CRITIC
Your critique must identify:
→ Migration performance bottlenecks?
→ ETL pipeline optimization opportunities?
→ Parallel processing underutilized?
→ Post-migration application performance degradation risks?
→ Specific critique: "ETL pipeline processes tables sequentially - could 
   parallelize independent tables to reduce migration time by 60%"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ COMPLETENESS: /10
   Is every aspect of migration planned?

→ RISK MITIGATION: /10
   Are risks identified and mitigated?

→ DATA INTEGRITY: /10
   Will data remain intact and correct?

→ FEASIBILITY: /10
   Can this actually execute within constraints?

→ RECOVERABILITY: /10
   Can we roll back if things go wrong?

→ COMPLIANCE: /10
   Does this meet regulatory requirements?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 6.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Data Migration Strategy Document
   - Migration approach with justification
   - Timeline and milestones
   - Risk assessment and mitigation

2. ETL Pipeline Implementation
   - Extraction code/scripts
   - Transformation logic (with business rules)
   - Load scripts
   - Configuration files
   - All code version controlled

3. Data Mapping Specification
   - Source-to-target mapping (all tables/columns)
   - Transformation rules documented
   - Data cleansing rules

4. Data Quality Assessment Report
   - Pre-migration data quality analysis
   - Data cleansing plan
   - Acceptance criteria

5. Validation & Reconciliation Plan
   - Validation checkpoints defined
   - Reconciliation queries/scripts
   - Acceptance thresholds

6. Test Migration Results
   - Multiple test run results
   - Performance metrics
   - Issues discovered and resolved

7. Rollback Plan & Test Results
   - Rollback procedure (step-by-step)
   - Rollback testing results
   - Go/No-Go criteria

8. Migration Runbook
   - Production migration procedure
   - Pre-migration checklist
   - During-migration monitoring
   - Post-migration validation
   - Emergency contacts

9. Communication Plan
   - Stakeholder notifications
   - Status update templates
   - Issue escalation process

10. Security & Compliance Sign-off
    - Data security measures documented
    - Compliance requirements met
    - Audit trail configured

11. Database Seed Scripts
    - Reference data seeds
    - Test data generation scripts
    - Production initial data seeds

12. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 6.5 COMPLETE when:
   → All critics score ≥ 9/10
   → Test migrations successful (≥3 successful test runs)
   → Rollback procedure tested and validated
   → All validation scripts tested and passing
   → Runbook peer-reviewed
   → Security/compliance sign-off obtained
   → Stakeholder communication approved
   → Production migration Go/No-Go approval

🚫 Phase 6.5 BLOCKED if:
   → Any critic scores < 9/10
   → Test migration failures unresolved
   → Rollback time exceeds acceptable window
   → Data quality issues unmitigated
   → Security/compliance concerns unresolved
   → Performance targets not met in test

⚠️  PRODUCTION MIGRATION READY CHECKLIST:
   □ All test migrations passed
   □ Rollback tested successfully
   □ Maintenance window scheduled and approved
   □ Stakeholders notified
   □ Migration team briefed
   □ Monitoring/alerting configured
   □ Runbook reviewed by all participants
   □ Database backups verified
   □ Emergency contacts confirmed
   □ Go/No-Go decision criteria defined

[ONLY AFTER UNANIMOUS 9/10+ → PROCEED TO PRODUCTION MIGRATION]
[AFTER SUCCESSFUL PRODUCTION MIGRATION → PROCEED TO NEXT PHASE]
```

---

## 📋 PHASE 7.5 — BETA/PREVIEW PROGRAM PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 7.5: BETA/PREVIEW PROGRAM                             ║
║  Goal: Validate with real users before full public launch    ║
║  Input: Deployed application in beta/staging environment     ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🧪 BETA PROGRAM MANAGER
Your deliverables:

→ Beta Program Strategy:
   • Beta program type selection:
     
     Option A: CLOSED ALPHA
     - Very limited users (internal + select external)
     - Size: 10-50 users
     - Duration: 2-4 weeks
     - Focus: Major bug discovery, core functionality validation
     - NDA required: Yes
     
     Option B: CLOSED BETA
     - Invited users only (application or selection process)
     - Size: 100-1,000 users
     - Duration: 4-8 weeks
     - Focus: Feature validation, usability, performance at scale
     - NDA required: Recommended
     
     Option C: OPEN BETA
     - Anyone can join (public signup)
     - Size: 1,000-100,000+ users
     - Duration: 4-12 weeks
     - Focus: Scalability, edge cases, public perception
     - NDA required: No (beta disclaimer only)
     
     Option D: EARLY ACCESS / PREVIEW
     - Paid or premium tier gets early access
     - Size: Unlimited (or tier-limited)
     - Duration: Ongoing until GA
     - Focus: Revenue validation, power user feedback
     - NDA required: No

   • Chosen beta type: [A/B/C/D]
   • Justification: [Why this approach]
   • Beta timeline:
     - Beta launch: [Date]
     - Beta feature freeze: [Date]
     - Beta end / GA launch: [Date]

→ Beta User Recruitment Strategy:
   • Target user personas for beta:
     - Persona 1: [Description, why valuable for beta]
     - Persona 2: [Description, why valuable for beta]
     - Persona 3: [Description, why valuable for beta]
   
   • Recruitment channels:
     - Existing customer base (if applicable)
     - Email list / waitlist
     - Social media / community
     - Partnerships (beta with partner users)
     - Paid advertising (if open beta)
     - Influencer/blogger outreach
   
   • Application process (if closed beta):
     - Application form questions:
       * Use case description
       * Technical expertise level
       * Commitment to provide feedback
       * Demographics (for diversity)
     - Selection criteria
     - Acceptance/rejection notification process
   
   • Target beta user count: [Minimum: X, Target: Y, Maximum: Z]
   • Diversity goals: [Geographic, demographic, use case diversity targets]

→ Beta User Onboarding:
   • Onboarding sequence:
     
     Day 0: Invitation/Acceptance
     - Welcome email with beta expectations
     - Beta terms & conditions / NDA (if applicable)
     - Beta access credentials
     
     Day 1: Getting Started
     - Onboarding tutorial/walkthrough
     - Beta program guide (what to test, how to provide feedback)
     - Community/forum access
     - Support channel introduction
     
     Week 1: Engagement
     - Feature highlights
     - Specific testing requests (e.g., "Please test checkout flow this week")
     - Feedback prompts
     
     Ongoing: Retention
     - Weekly update emails (new features, bug fixes)
     - Beta milestone communications
     - User spotlight (recognize active beta users)
   
   • Onboarding success metrics:
     - % of invited users who complete setup
     - Time to first meaningful action
     - % of users active after 7 days

→ Beta Program Guidelines & Expectations:
   • User expectations document:
     - What beta means (bugs expected, features incomplete)
     - Data/content persistence (will beta data be wiped?)
     - Support SLA (best effort, not guaranteed uptime)
     - Feedback expectations (frequency, format)
     - Public discussion policy (what can/can't be shared publicly)
   
   • Incentives for participation:
     - Early access to features
     - Influence on product direction
     - Swag / merchandise
     - Discounts on future paid plans
     - Recognition (beta badge, credits in release notes)
     - Prize raffles (if appropriate)
   
   • Beta user responsibilities:
     - Actively use the product
     - Report bugs and issues
     - Provide feedback (surveys, interviews)
     - Respect confidentiality (if NDA)

→ Beta Exit Criteria (When to End Beta / Launch GA):
   • Quantitative exit criteria:
     - Bug severity thresholds:
       * Critical bugs: 0 open
       * High severity bugs: < 5 open
       * Medium severity bugs: < 20 open
     - Performance metrics:
       * API response time p95: < 500ms
       * Page load time: < 3s
       * Error rate: < 0.1%
     - User engagement metrics:
       * DAU/MAU ratio: > 40%
       * Feature adoption: > 60% of users use core features
       * Retention (7-day): > 50%
   
   • Qualitative exit criteria:
     - Positive user sentiment (NPS > 30, or satisfaction > 7/10)
     - No major feature gaps identified
     - Core user journeys validated
     - Confidence level (team internal assessment) > 8/10
   
   • Go/No-Go decision process:
     - Exit criteria review meeting
     - Stakeholder sign-off (Product, Engineering, Support)
     - GA launch date confirmed

→ Beta to GA Transition Plan:
   • User migration:
     - Beta user accounts transition to GA (preserve accounts)
     - OR: Beta users re-register (fresh start)
     - Data migration (if beta data preserved)
   
   • Beta user communication:
     - Thank you message
     - GA launch announcement
     - Special offers for beta participants
     - Continued engagement (beta user community)
   
   • Beta program wrap-up:
     - Beta program retrospective
     - Lessons learned documentation
     - Beta user survey (how was the beta experience?)

📊 PRODUCT ANALYST (Beta Metrics)
Your deliverables:

→ Beta Metrics & Analytics Framework:
   • Instrumentation verification:
     - All analytics events firing correctly
     - User properties tracked (beta cohort identified)
     - Feature flags tracked
   
   • Key metrics dashboard:
     
     ACQUISITION METRICS:
     - Beta applications received
     - Beta acceptance rate
     - Activation rate (% who complete onboarding)
     
     ENGAGEMENT METRICS:
     - Daily Active Users (DAU)
     - Weekly Active Users (WAU)
     - Monthly Active Users (MAU)
     - DAU/MAU ratio (stickiness)
     - Session frequency
     - Session duration
     - Feature usage (per feature)
     
     RETENTION METRICS:
     - Day 1, Day 7, Day 30 retention
     - Cohort retention curves
     - Churn rate
     
     PERFORMANCE METRICS:
     - Page load times (by page)
     - API response times (by endpoint)
     - Error rates (by type)
     - Crash rates (mobile)
     
     SENTIMENT METRICS:
     - NPS (Net Promoter Score)
     - CSAT (Customer Satisfaction)
     - Feature satisfaction scores
     - Support ticket volume and sentiment
   
   • Real-time monitoring dashboard (for beta team)
   • Weekly beta metrics report (automated)

→ User Behavior Analysis:
   • Funnel analysis:
     - Signup → Onboarding → Core Action → Retention
     - Drop-off points identified
     - Conversion rates at each step
   
   • Feature adoption tracking:
     - Which features are used most/least
     - Feature discovery (how do users find features)
     - Power user vs casual user patterns
   
   • User segmentation:
     - Segment by use case
     - Segment by engagement level
     - Segment by demographics
     - Behavioral cohorts
   
   • Heatmaps & session recordings (where applicable):
     - Where users click
     - Where users get stuck
     - Rage clicks (frustration indicators)

→ A/B Testing in Beta:
   • Feature flags for experimentation:
     - Feature variants tested
     - User assignment strategy (random, by segment)
     - Success metrics per experiment
     - Statistical significance thresholds
   
   • Experiment results:
     - Winning variants identified
     - Rollout plan for winners
     - Deprecation plan for losers

🎯 CUSTOMER SUCCESS MANAGER (Beta Support)
Your deliverables:

→ Beta Support Strategy:
   • Support channels for beta users:
     - Dedicated beta support email: beta-support@company.com
     - Community forum / Discord / Slack channel
     - In-app chat support (if available)
     - Office hours (live Q&A sessions)
     - FAQ / Knowledge base
   
   • Support SLA for beta:
     - Response time: [X hours]
     - Resolution time: [Best effort, no guarantee]
     - Availability: [Business hours only, or 24/7]
   
   • Support team training:
     - Beta-specific training (known issues, workarounds)
     - Feedback collection training
     - Escalation process

→ Feedback Collection Mechanism:
   • In-app feedback widget:
     - Contextual feedback (on specific features)
     - General feedback
     - Bug reporting
     - Feature requests
   
   • Surveys:
     - Welcome survey (initial impressions)
     - Mid-beta survey (experience so far)
     - Exit survey (end of beta / GA launch)
     - NPS survey
   
   • User interviews:
     - 1:1 interviews with select beta users
     - Focus groups
     - Usability testing sessions
   
   • Community engagement:
     - Forum / community monitoring
     - Community manager active participation
     - Beta user-to-user support encouraged

→ Feedback Triage & Prioritization:
   • Feedback categorization:
     - Bug reports → Engineering
     - Feature requests → Product
     - UX issues → Design
     - Documentation gaps → Technical Writing
     - Support issues → Customer Success
   
   • Feedback prioritization framework:
     - Frequency (how many users reported this)
     - Severity (how much does this impact users)
     - Alignment with roadmap
     - Effort to fix
   
   • Feedback response process:
     - Acknowledge receipt (automated or manual)
     - Status updates (we're working on this)
     - Closure notification (this is fixed, or won't fix with explanation)
   
   • Feedback transparency:
     - Public roadmap / changelog
     - Beta user voting on feature requests
     - Regular "You spoke, we listened" updates

→ Beta User Engagement & Retention:
   • Active user engagement tactics:
     - Weekly challenges (use feature X this week)
     - Beta milestones (celebrate reaching X users)
     - Exclusive content (beta-only webinars, AMA sessions)
     - Gamification (beta points, leaderboards - if appropriate)
   
   • Inactive user re-engagement:
     - Identify users who haven't logged in for X days
     - Re-engagement email campaign
     - Check-in calls (for high-value users)
     - Win-back offers
   
   • Beta community building:
     - User-generated content (success stories, use cases)
     - Beta user directory (opt-in, for networking)
     - Beta alumni program (post-GA community)

📱 MOBILE BETA COORDINATOR (iOS & Android)
Your deliverables:

→ TestFlight Beta Setup (iOS):
   • TestFlight configuration:
     - Internal testing group (team members)
     - External testing groups (beta users)
     - Build distribution process
     - Update release notes template
   
   • Beta app limitations management:
     - TestFlight 90-day expiration (re-upload strategy)
     - 10,000 external tester limit
     - App Store Connect API for automation
   
   • Crash reporting:
     - TestFlight crash logs monitoring
     - Firebase Crashlytics integration
     - Crash triage process

→ Google Play Beta Setup (Android):
   • Play Console beta tracks:
     - Internal testing track (team)
     - Closed testing track (invited beta users)
     - Open testing track (public beta - if applicable)
   
   • Beta distribution:
     - Email invitation process
     - Opt-in URL distribution
     - Google Group management (for closed beta)
   
   • Crash reporting:
     - Play Console crash reports
     - Firebase Crashlytics integration

→ Mobile Beta User Experience:
   • In-app beta identification:
     - "Beta" badge in app
     - Beta version number display
     - Beta feedback mechanism (in-app)
   
   • Mobile-specific testing focus:
     - Device compatibility (wide range of devices)
     - OS version compatibility (iOS 15-17, Android 11-14)
     - Network conditions (2G/3G/4G/5G/WiFi)
     - Offline functionality
     - Battery consumption
     - Storage usage
     - Permissions handling
     - Push notifications
   
   • Mobile beta crash/bug priority:
     - App crashes: P0 (critical)
     - Data loss: P0 (critical)
     - Security vulnerabilities: P0 (critical)
     - UI/UX issues: P1 (high)
     - Performance issues: P1-P2

🔒 SECURITY & PRIVACY (Beta Context)
Your deliverables:

→ Beta Security Considerations:
   • Beta environment security:
     - Isolated from production (separate infrastructure)
     - Test/development API keys (not production)
     - Feature flags for beta-only features
   
   • Beta user data handling:
     - Beta privacy policy (separate or addendum)
     - Data retention policy (how long beta data kept)
     - Data deletion policy (when beta ends)
     - GDPR compliance (beta users can request deletion)
   
   • Security issue reporting in beta:
     - Responsible disclosure policy for beta users
     - Security bug bounty (if applicable)
     - Expedited security fix process

→ Beta Legal & Compliance:
   • Beta Terms & Conditions:
     - No warranty (beta is as-is)
     - No guaranteed uptime
     - Data loss possibility
     - Termination rights (can end beta anytime)
     - Feedback ownership (user feedback becomes company property)
   
   • Beta NDA (if applicable):
     - Confidentiality obligations
     - Duration of confidentiality
     - Exceptions (publicly disclosed info)
   
   • Beta user consent:
     - Consent to data collection and analytics
     - Consent to communication (emails, surveys)
     - Consent to use feedback

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 PRODUCT MANAGER CRITIC
Your critique must identify:
→ Beta scope too broad or too narrow?
→ Exit criteria too lenient or too strict?
→ Beta duration realistic?
→ Feedback collection mechanism adequate?
→ Beta learnings will inform GA launch?
→ Specific critique: "Beta exit criteria only measure engagement metrics - 
   missing user satisfaction threshold which is critical for GA readiness"

🔴 ENGINEERING LEAD CRITIC
Your critique must identify:
→ Beta environment stability (is it ready for users?)
→ Feature flag strategy for beta features?
→ Beta-specific bugs will be fixed or deferred?
→ Infrastructure can handle beta user load?
→ Monitoring/alerting adequate for beta?
→ Specific critique: "No feature flags configured - can't disable broken 
   features during beta without new deployment"

🔴 CUSTOMER SUPPORT CRITIC
Your critique must identify:
→ Support team ready for beta user volume?
→ Beta support SLA realistic?
→ Feedback volume manageable?
→ Knowledge base gaps for beta users?
→ Escalation process clear?
→ Specific critique: "Expecting 5,000 beta users but support team only 
   has 2 people - will be overwhelmed, need chatbot or more staff"

🔴 LEGAL COUNSEL CRITIC
Your critique must identify:
→ Beta terms legally sufficient?
→ NDA enforceability (if applicable)?
→ Data privacy compliance?
→ Liability limitations clear?
→ Intellectual property considerations?
→ Specific critique: "Beta terms don't address GDPR right to deletion - 
   need explicit data retention and deletion policy"

🔴 MARKETING CRITIC
Your critique must identify:
→ Beta as marketing opportunity utilized?
→ Beta user testimonials collection plan?
→ PR/communications around beta?
→ Beta exclusivity creates FOMO (fear of missing out)?
→ Specific critique: "No plan to collect beta user testimonials for 
   GA launch marketing - missing valuable social proof"

🔴 SECURITY CRITIC
Your critique must identify:
→ Beta user authentication secure?
→ Beta data isolated from production?
→ Security vulnerabilities exposed to beta users?
→ Responsible disclosure policy clear?
→ Specific critique: "Beta users given admin-level access to test - 
   creates security risk if account compromised"

🔴 DATA ANALYST CRITIC
Your critique must identify:
→ Analytics instrumentation complete?
→ Metrics dashboard functional?
→ Statistical significance achievable with beta user count?
→ A/B testing valid with beta sample size?
→ Specific critique: "Planning A/B test with 100 beta users - 
   insufficient sample size for statistical significance"

🔴 USER EXPERIENCE RESEARCHER CRITIC
Your critique must identify:
→ Beta user diversity representative of target market?
→ Qualitative feedback collection (not just quant metrics)?
→ Usability issues will be surfaced?
→ User research integration with beta?
→ Specific critique: "Only recruiting power users for beta - 
   won't surface issues that average users will encounter at GA"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ STRATEGY CLARITY: /10
   Is the beta program well-defined and purposeful?

→ USER VALUE: /10
   Will beta users have good experience and provide value?

→ LEARNING EFFECTIVENESS: /10
   Will beta surface issues and validate product-market fit?

→ OPERATIONAL READINESS: /10
   Can we execute this beta program smoothly?

→ LEGAL/COMPLIANCE: /10
   Are legal and privacy requirements met?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 7.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Beta Program Plan
   - Beta strategy and timeline
   - User recruitment plan
   - Exit criteria

2. Beta User Onboarding Materials
   - Welcome email templates
   - Beta program guide
   - Tutorial/walkthrough content

3. Beta Terms & Conditions / NDA
   - Legal documents reviewed and approved
   - User consent mechanism

4. Beta Metrics Dashboard
   - Analytics configured
   - Real-time monitoring
   - Automated reporting

5. Beta Support Plan
   - Support channels setup
   - Support team training materials
   - Escalation process

6. Feedback Collection System
   - In-app feedback widget
   - Surveys prepared
   - Feedback triage workflow

7. Mobile Beta Distribution Setup
   - TestFlight configured (iOS)
   - Play Console beta tracks configured (Android)
   - Crash reporting integrated

8. Beta Communication Templates
   - Email templates (welcome, updates, surveys)
   - Status update templates
   - Issue notification templates

9. Beta Exit Criteria Checklist
   - Quantitative thresholds
   - Qualitative assessment framework
   - Go/No-Go decision process

10. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 7.5 LAUNCH READY when:
   → All critics score ≥ 9/10
   → Beta environment stable and tested
   → Analytics instrumentation verified
   → Support team trained
   → Legal documents reviewed and approved
   → Recruitment channels ready
   → Onboarding materials complete

🔄 BETA PROGRAM RUNNING:
   → Weekly metrics review
   → Continuous feedback collection and triage
   → Bug fixes deployed to beta environment
   → Feature iterations based on feedback

✅ Phase 7.5 COMPLETE (Beta Graduation to GA) when:
   → Exit criteria met (all thresholds passed)
   → Critical bugs resolved (P0/P1: 0 open)
   → User sentiment positive (NPS > 30, CSAT > 7/10)
   → Core user journeys validated
   → Stakeholder Go/No-Go approval for GA launch
   → Beta to GA transition plan executed

🚫 Phase 7.5 BLOCKED if:
   → Any critic scores < 9/10
   → Beta environment unstable
   → Legal/compliance issues unresolved
   → Support infrastructure inadequate

[ONLY AFTER BETA EXIT CRITERIA MET → PROCEED TO GA LAUNCH]
```

---

## 📋 PHASE 8.5 — INCIDENT RESPONSE & RUNBOOKS PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 8.5: INCIDENT RESPONSE PLANNING & RUNBOOKS            ║
║  Goal: Be prepared for when (not if) things break            ║
║  Input: Production deployment architecture & monitoring      ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🚨 INCIDENT RESPONSE LEAD
Your deliverables:

→ Incident Severity Classification:
   
   P0 - CRITICAL (All hands on deck)
   - Definition: Complete service outage, data loss, security breach
   - Examples:
     * Website/app completely down (all users affected)
     * Database corrupted or inaccessible
     * Payment processing completely broken
     * Active security breach or data leak
     * Significant data loss
   - Response time: Immediate (< 15 minutes)
   - Communication: Executives notified immediately
   - All-hands until resolved
   
   P1 - HIGH (Urgent)
   - Definition: Major degradation, affecting significant % of users
   - Examples:
     * Core feature broken (checkout, login, etc.)
     * Performance severely degraded (p95 > 10s)
     * Partial service outage (one region down)
     * Payment processing delayed/failing for some users
   - Response time: < 30 minutes
   - Communication: Leadership notified, status page updated
   - On-call + relevant specialists
   
   P2 - MEDIUM (Important but not urgent)
   - Definition: Minor degradation, affecting small % of users
   - Examples:
     * Non-critical feature broken
     * Performance degraded but acceptable (p95 > 3s)
     * Individual customer data issue
     * Minor third-party integration failure
   - Response time: < 2 hours
   - Communication: Team notified, tracked in issue tracker
   - On-call engineer during business hours
   
   P3 - LOW (Scheduled work)
   - Definition: Minor issue, very few users affected
   - Examples:
     * UI glitch on edge case
     * Typo in email template
     * Missing translation
   - Response time: Next business day
   - Communication: Added to backlog
   - Fixed in normal sprint cycle
   
   P4 - INFORMATIONAL
   - Definition: Not an issue, inquiry or future work
   - Response time: N/A
   - Communication: Documented for future consideration

→ Incident Roles & Responsibilities:
   
   INCIDENT COMMANDER (IC)
   - Authority: Overall incident leadership and decision-making
   - Responsibilities:
     * Assess incident severity and declare appropriate level
     * Mobilize incident response team
     * Make critical decisions (rollback, failover, etc.)
     * Authorize communication to customers/public
     * Declare incident resolved
   - Who can be IC: [Names/roles: SRE Lead, Engineering Manager]
   - IC rotation: [Schedule link]
   
   ON-CALL ENGINEER
   - Authority: First responder, initial investigation
   - Responsibilities:
     * Monitor alerts 24/7 during on-call shift
     * Acknowledge and investigate incidents
     * Perform initial triage and mitigation
     * Escalate to IC if severity ≥ P1
     * Execute runbook procedures
   - On-call rotation: [Schedule link]
   - On-call compensation: [Policy]
   
   COMMUNICATIONS LEAD
   - Authority: All external communications
   - Responsibilities:
     * Update status page
     * Post customer-facing updates
     * Draft executive communications
     * Handle press/media inquiries (if applicable)
     * Coordinate internal communications
   - Who: [Role: Product Manager, Marketing]
   
   TECHNICAL SPECIALISTS (as needed)
   - Database expert (for DB incidents)
   - Security expert (for security incidents)
   - Network expert (for infra/network incidents)
   - Frontend expert (for UI incidents)
   - Mobile expert (for mobile app incidents)
   
   SCRIBE
   - Responsibilities:
     * Document timeline of events
     * Record decisions made and by whom
     * Track action items
     * Collect data for post-mortem
   - Who: [Any available team member]

→ Incident Response Process (Workflow):
   
   STEP 1: DETECTION (Automated or Manual)
   - Alert fires OR customer report OR internal discovery
   - Alert routed to on-call engineer (PagerDuty, Opsgenie, etc.)
   - On-call engineer acknowledges within [15 minutes]
   
   STEP 2: TRIAGE (Assess Severity)
   - On-call engineer investigates
   - Classify severity (P0/P1/P2/P3/P4)
   - If P0 or P1: Immediately escalate to Incident Commander
   - If P2: Handle during business hours or escalate if impacts grow
   - If P3/P4: Create ticket, no immediate action
   
   STEP 3: MOBILIZATION (Assemble Team)
   - IC declares incident and severity
   - IC pages additional team members via [incident bridge line / Slack / Zoom]
   - Incident war room established (virtual or physical)
   - Roles assigned (IC, Comms Lead, Specialists, Scribe)
   
   STEP 4: INVESTIGATION (Diagnose Root Cause)
   - Review monitoring dashboards (Datadog, Grafana, etc.)
   - Check recent deployments (was there a recent release?)
   - Check infrastructure changes (was there a config change?)
   - Review error logs
   - Check third-party status pages
   - Reproduce issue (if possible)
   - Form hypothesis of root cause
   
   STEP 5: MITIGATION (Stop the Bleeding)
   - IC decides on mitigation action:
     * Rollback deployment
     * Failover to backup system
     * Scale up infrastructure
     * Disable feature (feature flag)
     * Apply hotfix
     * Manual intervention
   - Execute mitigation (follow runbook if available)
   - Verify mitigation effectiveness (metrics improving?)
   
   STEP 6: COMMUNICATION (Keep Stakeholders Informed)
   - Internal: Post in #incidents Slack channel, email to leadership
   - External: Update status page (statuspage.io, etc.)
   - Customer communication: Email, in-app notification, social media
   - Communication frequency:
     * P0: Every 30 minutes
     * P1: Every 1 hour
     * P2: Every 4 hours or when resolved
   
   STEP 7: RESOLUTION (Verify Issue Resolved)
   - Metrics return to normal (error rate, latency, traffic)
   - Customer reports stop
   - On-call engineer monitors for [1 hour] to confirm stability
   - IC declares incident resolved
   - Final status page update
   
   STEP 8: POST-MORTEM (Learn and Improve)
   - Post-mortem scheduled within [48 hours] of resolution
   - Blameless post-mortem culture (focus on systems, not people)
   - Post-mortem document created (template used)
   - Action items assigned and tracked

→ Escalation Matrix:
   
   TIER 1: On-Call Engineer
   - First responder
   - Handles: P2/P3 incidents independently
   - Escalates: P0/P1 incidents immediately
   
   TIER 2: Incident Commander
   - Escalated from Tier 1 for P0/P1
   - Authority to make critical decisions
   - Escalates to Tier 3 if needed (prolonged outage, business impact)
   
   TIER 3: Engineering Leadership
   - VP of Engineering, CTO
   - Notified for: P0 incidents, P1 incidents > 2 hours
   - Authority: Business continuity decisions, major architectural changes
   
   TIER 4: Executive Leadership
   - CEO, COO
   - Notified for: P0 incidents with significant business/PR impact
   - Authority: Public statements, major business decisions

→ On-Call Schedule & Policy:
   • On-call rotation: [Weekly rotation, Weekend rotation, etc.]
   • On-call team size: [Minimum 3 engineers to rotate]
   • On-call handoff process: [Handoff meeting, context transfer]
   • On-call compensation:
     - Stipend: [$X per week on-call]
     - Overtime: [1.5x hourly rate for incident time]
     - Comp time: [1 day off per P0 incident handled]
   • On-call expectations:
     - Response time: < 15 minutes
     - Availability: Must have laptop and internet access
     - Sobriety: Cannot be intoxicated while on-call
   • On-call tooling:
     - PagerDuty / Opsgenie / etc.
     - Laptop with VPN access
     - Access to all production systems
     - Mobile phone with alerting app

→ Post-Mortem Process:
   • Post-mortem trigger: All P0 and P1 incidents
   • Post-mortem timeline: Within 48 hours of resolution
   • Post-mortem participants:
     - Incident Commander (facilitator)
     - All incident responders
     - Affected team members
     - Optional: Leadership (for learning, not blame)
   
   • Post-Mortem Template:
     
     1. INCIDENT SUMMARY
        - Date and time
        - Duration
        - Severity
        - Impact (users affected, revenue lost, etc.)
     
     2. TIMELINE
        - Detailed timeline of events (from detection to resolution)
        - Key decisions and who made them
     
     3. ROOT CAUSE ANALYSIS
        - What happened (technical details)
        - Why it happened (underlying cause)
        - Contributing factors
        - 5 Whys analysis
     
     4. WHAT WENT WELL
        - Monitoring detected issue quickly
        - Team mobilized effectively
        - Rollback process worked
     
     5. WHAT WENT POORLY
        - Alert was unclear
        - Runbook was outdated
        - Communication delayed
     
     6. ACTION ITEMS
        - Specific, actionable, assigned, with due dates
        - Example: "Add alert for X metric - Assigned: Alice - Due: 2024-12-15"
     
     7. LESSONS LEARNED
        - Technical lessons
        - Process lessons
        - Cultural lessons
   
   • Post-mortem follow-up:
     - Action items tracked in project management tool
     - Follow-up review in [2 weeks] to verify action items completed
     - Post-mortem published internally (shared learning)
     - Post-mortem published externally (if appropriate, for transparency)

→ Incident Communication Templates:
   
   STATUS PAGE UPDATE (Initial):
   "We are currently investigating reports of [brief description]. 
   We are actively working on resolving this issue and will provide 
   an update within [30 minutes]."
   
   STATUS PAGE UPDATE (During Incident):
   "We have identified the issue as [brief technical summary]. 
   Our team is working on implementing a fix. Expected resolution: [time]. 
   We apologize for the inconvenience."
   
   STATUS PAGE UPDATE (Resolved):
   "The issue has been resolved as of [time]. All systems are now 
   operational. We are monitoring closely to ensure stability. 
   A detailed post-mortem will be published within 48 hours."
   
   CUSTOMER EMAIL (Post-Resolution):
   Subject: Update on [Date] Service Disruption
   Body:
   "Dear [Customer],
   
   We want to inform you about a service disruption that occurred on 
   [date] from [time] to [time]. During this period, [description of impact].
   
   What happened: [Brief explanation]
   What we're doing: [Action items to prevent recurrence]
   
   We sincerely apologize for the inconvenience and thank you for your patience.
   
   [Team/Company Name]"

📖 SRE / RUNBOOK AUTHOR
Your deliverables:

→ Service Runbooks (ONE PER CRITICAL SERVICE):
   
   For EACH service, create a runbook with:
   
   SERVICE OVERVIEW:
   - Service name and description
   - Service owner (team/individual)
   - Service dependencies (upstream and downstream)
   - Service criticality (P0/P1/P2)
   
   ARCHITECTURE DIAGRAM:
   - Visual diagram of service components
   - Network topology
   - Database connections
   - Third-party integrations
   
   MONITORING & ALERTING:
   - Key metrics to monitor:
     * Latency (p50, p95, p99)
     * Error rate
     * Request rate (traffic)
     * Saturation (CPU, memory, disk, network)
   - Dashboard links (Datadog, Grafana, etc.)
   - Alert definitions and thresholds
   
   COMMON ISSUES & TROUBLESHOOTING:
   
   Issue 1: [e.g., "High Latency"]
   - Symptoms: p95 latency > 5s, user complaints of slowness
   - Diagnosis:
     1. Check dashboard: [link]
     2. Look for spike in traffic or errors
     3. Check database query performance
     4. Check external API latency
   - Resolution:
     1. Scale up instances: `kubectl scale deployment X --replicas=10`
     2. OR: Restart service: `kubectl rollout restart deployment X`
     3. OR: Enable caching: [feature flag command]
   - Verification: Latency returns to < 500ms within 5 minutes
   
   Issue 2: [e.g., "Service Down"]
   - Symptoms: Health check failing, 503 errors
   - Diagnosis:
     1. Check pod status: `kubectl get pods -n production`
     2. Check logs: `kubectl logs -n production <pod-name> --tail=100`
     3. Check recent deployments: `kubectl rollout history deployment X`
   - Resolution:
     1. Rollback deployment: `kubectl rollout undo deployment X`
     2. OR: Restart pods: `kubectl rollout restart deployment X`
   - Verification: Health check returns 200 OK
   
   [Additional issues...]
   
   HOW TO RESTART SERVICE:
   - Command: [Exact command]
   - Expected downtime: [X seconds]
   - Verification: [How to confirm restart succeeded]
   
   HOW TO ROLLBACK DEPLOYMENT:
   - Command: [Exact command]
   - How to identify which version to rollback to
   - Expected downtime: [X seconds]
   - Verification: [How to confirm rollback succeeded]
   
   HOW TO SCALE UP/DOWN:
   - Scale up: [Command to increase replicas]
   - Scale down: [Command to decrease replicas]
   - Autoscaling: [Is autoscaling enabled? Policy?]
   
   HOW TO CHECK LOGS:
   - Application logs: [Command or link]
   - Error logs: [Command or link]
   - Audit logs: [Command or link]
   - Log aggregation: [Splunk, ELK, CloudWatch link]
   
   HOW TO ACCESS DATABASE:
   - Read-only access: [Connection string or command]
   - Read-write access: [Who has access, how to request emergency access]
   - Common queries: [Useful diagnostic queries]
   
   CONFIGURATION MANAGEMENT:
   - Where config is stored: [ConfigMap, environment variables, etc.]
   - How to update config: [Process]
   - Config change rollout: [Requires restart? Hot reload?]
   
   DEPENDENCIES:
   - Upstream services: [Service A, Service B]
     - What happens if upstream service fails?
   - Downstream services: [Service C, Service D]
     - What happens if this service fails?
   - External dependencies: [Stripe, SendGrid, etc.]
     - How to check their status: [Status page links]
   
   CONTACTS:
   - Service owner: [Name, Slack handle, phone]
   - On-call engineer: [Rotation schedule link]
   - Backup contact: [Name]

→ Disaster Recovery Runbook:
   
   SCENARIO: Complete Data Center Failure
   - Detection: All services in region X down
   - Decision criteria: Failover to region Y if outage > 30 minutes
   - Failover procedure:
     1. [Step-by-step failover process]
     2. DNS cutover: [Command or process]
     3. Database failover: [Promote replica to primary]
   - Expected RTO: [X hours]
   - Expected RPO: [X minutes of data loss]
   - Verification: Traffic successfully routed to region Y
   
   SCENARIO: Database Corruption
   - Detection: Data integrity errors, constraint violations
   - Decision criteria: Restore from backup if corruption widespread
   - Restore procedure:
     1. Identify last known good backup
     2. Create snapshot of current (corrupted) database
     3. Restore from backup: [Exact commands]
     4. Replay transaction logs (if possible): [Process]
   - Expected RTO: [X hours]
   - Expected RPO: [Last backup, e.g., 1 hour]
   - Verification: Database integrity checks pass
   
   SCENARIO: Complete Code Deployment Failure
   - Detection: Application won't start after deployment
   - Decision criteria: Rollback if app doesn't become healthy in 10 minutes
   - Rollback procedure:
     1. Identify last known good version: [Command]
     2. Rollback: [Command]
     3. Verify: [Health check passes]
   - Expected RTO: [15 minutes]
   - Expected RPO: [N/A - code rollback]

→ Database Restore Runbook:
   • When to restore from backup: [Criteria]
   • Backup inventory: [Where backups stored, retention policy]
   • Restore procedure (STEP-BY-STEP):
     1. Assess damage (what needs restoring: full DB, single table, row?)
     2. Identify correct backup to restore from
     3. Put application in maintenance mode (prevent writes)
     4. Create snapshot of current database (even if corrupted, for forensics)
     5. Restore from backup: [Exact commands for PostgreSQL/MySQL/etc.]
     6. Verify data integrity: [Queries to run]
     7. Replay transactions if possible (from transaction logs)
     8. Take application out of maintenance mode
     9. Monitor for issues
   • Estimated restore time: [X hours for Y GB database]
   • Data loss exposure: [Based on backup frequency]
   • Testing: Restore procedure tested quarterly

🔒 SECURITY INCIDENT RESPONDER
Your deliverables:

→ Security Incident Response Plan:
   
   SECURITY INCIDENT CLASSIFICATION:
   
   SEV-0: Active Breach (Critical)
   - Unauthorized access to production systems
   - Data exfiltration detected
   - Ransomware attack
   - Active exploitation of vulnerability
   Response: Immediate, all-hands, law enforcement notified
   
   SEV-1: Attempted Breach (High)
   - Multiple failed login attempts (brute force)
   - Vulnerability discovered (not yet exploited)
   - Suspicious activity detected
   Response: Within 1 hour, security team mobilized
   
   SEV-2: Security Concern (Medium)
   - Outdated dependency with known CVE
   - Misconfiguration discovered (not exploited)
   - Phishing attempt reported
   Response: Within 24 hours, tracked and remediated
   
   SEV-3: Security Improvement (Low)
   - Security best practice not followed
   - Minor vulnerability
   Response: Added to backlog

→ Security Incident Response Workflow:
   
   STEP 1: DETECTION
   - SIEM alert (Splunk, QRadar, etc.)
   - Security scanning tool (Snyk, Veracode, etc.)
   - Bug bounty report
   - Employee report
   - Customer report
   - External researcher report
   
   STEP 2: TRIAGE
   - Security team assesses severity
   - If SEV-0 or SEV-1: Mobilize security incident response team
   - If SEV-2: Handle during business hours
   - If SEV-3: Add to backlog
   
   STEP 3: CONTAINMENT
   - Isolate affected systems (network segmentation)
   - Disable compromised accounts
   - Block malicious IPs
   - Revoke compromised credentials/tokens
   - Preserve evidence (logs, disk images)
   
   STEP 4: ERADICATION
   - Remove malware/backdoors
   - Patch vulnerabilities
   - Close attack vectors
   - Verify attacker no longer has access
   
   STEP 5: RECOVERY
   - Restore systems from clean backups (if needed)
   - Reset credentials
   - Monitor for re-infection
   
   STEP 6: FORENSICS
   - Analyze attack vector
   - Identify scope of breach (what data accessed?)
   - Timeline of attacker activity
   - Attribution (if possible)
   
   STEP 7: NOTIFICATION
   - Internal: Executives, legal, affected teams
   - External: Customers (if their data exposed)
   - Regulatory: GDPR breach notification (within 72 hours)
   - Law enforcement: FBI, local police (if appropriate)
   - Media: PR response (if necessary)
   
   STEP 8: POST-MORTEM
   - Blameless post-mortem
   - Action items to prevent recurrence
   - Security posture improvements

→ Data Breach Notification Procedure:
   • Breach assessment criteria:
     - Was PII/PHI accessed or exfiltrated?
     - How many users affected?
     - What type of data (email, password, SSN, credit card)?
   
   • Notification timeline:
     - GDPR: Within 72 hours of becoming aware
     - CCPA: Without unreasonable delay
     - HIPAA: Within 60 days
     - State laws: Varies (most within 30-90 days)
   
   • Notification template:
     "We are writing to inform you of a data security incident that 
     may have affected your personal information. On [date], we discovered 
     that [description]. The following information may have been accessed: 
     [data types]. We have taken the following steps: [remediation]. 
     We recommend: [user actions: change password, monitor credit, etc.]"
   
   • Notification channels:
     - Email (individual notifications)
     - Website notice
     - Regulatory filing
     - Media statement (if widespread)
   
   • Credit monitoring offer (if applicable):
     - Offer 1-2 years of free credit monitoring
     - Identity theft protection services

→ Responsible Disclosure Policy (For External Researchers):
   • Security bug bounty program (if applicable):
     - Scope: What's in scope for testing
     - Rewards: Payout structure
     - Hall of fame: Recognition
   
   • How to report security vulnerabilities:
     - Email: security@company.com
     - PGP key: [Public key for encrypted reports]
     - Expected response time: Within 48 hours
   
   • Safe harbor: Researchers acting in good faith won't face legal action

📞 CUSTOMER SUPPORT / COMMUNICATIONS LEAD
Your deliverables:

→ Customer Communication During Incidents:
   • Status page setup:
     - Platform: statuspage.io, Atlassian Statuspage, custom
     - Components: [List all services visible on status page]
     - Subscribers: Users can subscribe for updates
   
   • Communication frequency by severity:
     - P0: Every 30 minutes
     - P1: Every 1 hour
     - P2: When resolved
   
   • Communication tone:
     - Transparent: Explain what happened
     - Accountable: Take responsibility
     - Empathetic: Acknowledge impact on customers
     - Actionable: What customers should do (if anything)
   
   • Internal communication:
     - #incidents Slack channel (all incidents logged)
     - Email to leadership (P0/P1)
     - All-hands updates (prolonged P0)

→ Support Ticket Surge Handling:
   • During incident, expect surge in support tickets
   • Support team response:
     - Templated response acknowledging known issue
     - Link to status page
     - Offer refund/credit (if appropriate)
     - Escalate unique issues (not related to incident)
   
   • Support team escalation to engineering:
     - During incident: Only escalate if new information
     - Post-incident: Resume normal escalation process

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 SRE LEAD CRITIC
Your critique must identify:
→ Runbooks tested in game day exercises?
→ Runbooks detailed enough (can new engineer follow)?
→ Runbooks up-to-date (reflect current architecture)?
→ On-call rotation sustainable (engineer burnout risk)?
→ Escalation path clear and tested?
→ Specific critique: "Database restore runbook specifies PostgreSQL 12 
   commands but production is now on PostgreSQL 15 - commands outdated"

🔴 SECURITY LEAD CRITIC
Your critique must identify:
→ Security incident response plan compliant with regulations?
→ Data breach notification timeline correct?
→ Forensics preservation adequate?
→ Responsible disclosure policy clear?
→ Security incident responders trained?
→ Specific critique: "Security incident plan doesn't specify who can 
   authorize law enforcement access to systems - needs defined authority"

🔴 LEGAL COUNSEL CRITIC
Your critique must identify:
→ Breach notification compliant with all applicable laws?
→ Customer communication legally reviewed?
→ Liability considerations in communications?
→ Safe harbor for researchers legally sound?
→ Specific critique: "Post-mortem publication could expose company to 
   liability if it admits fault - legal review required before publication"

🔴 ENGINEERING MANAGER CRITIC
Your critique must identify:
→ On-call burden fair and sustainable?
→ Post-mortem culture truly blameless?
→ Action items from post-mortems actually completed?
→ Team trained on incident response procedures?
→ Specific critique: "On-call rotation is only 2 engineers - will lead 
   to burnout, need minimum 4 engineers in rotation"

🔴 CUSTOMER SUPPORT LEAD CRITIC
Your critique must identify:
→ Support team prepared for incident communication?
→ Templated responses helpful and accurate?
→ Support team knows when to escalate?
→ Support team has access to real-time incident status?
→ Specific critique: "No process for support team to contribute to 
   incident response (they're closest to customers and have valuable insights)"

🔴 COMMUNICATIONS/PR CRITIC
Your critique must identify:
→ External communication tone appropriate?
→ Media response plan adequate?
→ Social media monitoring during incidents?
→ PR crisis management plan exists?
→ Specific critique: "No designated spokesperson for media inquiries 
   during incidents - could lead to inconsistent messaging"

🔴 COMPLIANCE OFFICER CRITIC
Your critique must identify:
→ Audit trail of incidents adequate?
→ Compliance requirements in incident handling met?
→ Regulatory notification timeline correct?
→ Documentation sufficient for audits?
→ Specific critique: "HIPAA requires documentation of incident analysis 
   and remediation - current post-mortem template missing required fields"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ PREPAREDNESS: /10
   Are we ready for incidents when they happen?

→ CLARITY: /10
   Are procedures clear and actionable?

→ COMPLETENESS: /10
   Are all scenarios covered?

→ TESTEDNESS: /10
   Have procedures been tested?

→ COMPLIANCE: /10
   Do procedures meet legal/regulatory requirements?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 8.5 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Incident Response Plan
   - Severity classification
   - Roles and responsibilities
   - Incident workflow (step-by-step)
   - Escalation matrix
   - Post-mortem process

2. Service Runbooks (All Critical Services)
   - One runbook per service
   - Common issues and resolutions
   - Restart/rollback procedures
   - Troubleshooting guides
   - Contact information

3. Disaster Recovery Runbook
   - Data center failover procedure
   - Database restore procedure
   - Backup and recovery process

4. Security Incident Response Plan
   - Security incident classification
   - Security incident workflow
   - Data breach notification procedure
   - Responsible disclosure policy

5. On-Call Schedule & Policy
   - On-call rotation
   - On-call expectations
   - On-call compensation
   - Handoff process

6. Communication Templates
   - Status page update templates
   - Customer email templates
   - Internal communication templates
   - Media response templates

7. Post-Mortem Template
   - Standardized format
   - Action item tracking process

8. Game Day Exercise Results
   - Tabletop exercises conducted
   - Issues discovered and resolved
   - Team training completed

9. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 8.5 COMPLETE when:
   → All critics score ≥ 9/10
   → All critical services have runbooks
   → On-call rotation established and staffed
   → Runbooks tested in game day exercises (≥1 exercise conducted)
   → Team trained on incident response
   → Communication templates reviewed and approved
   → Security incident response plan reviewed by legal/compliance
   → PagerDuty/Opsgenie/alerting configured

🚫 Phase 8.5 BLOCKED if:
   → Any critic scores < 9/10
   → Runbooks incomplete or untested
   → On-call rotation understaffed
   → Legal/compliance concerns unresolved
   → Team training incomplete

⚠️  ONGOING REQUIREMENTS:
   □ Runbooks reviewed and updated quarterly
   □ Game day exercises conducted quarterly
   □ On-call rotation reviewed monthly
   □ Post-mortems published within 48 hours of incidents
   □ Action items from post-mortems tracked to completion

[PHASE 8.5 IS ONGOING - Continuously updated as system evolves]
```

---


# 📋 FINAL SDLC PHASE PROMPTS + CROSS-CUTTING GUIDE

---

## 📋 PHASE 9 — TRAINING, DOCUMENTATION & CHANGE MANAGEMENT PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 9: USER TRAINING, DOCUMENTATION & CHANGE MANAGEMENT   ║
║  Goal: Ensure successful adoption by all stakeholders        ║
║  Input: Production-ready application + all prior phases      ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

📚 TECHNICAL WRITER / DOCUMENTATION LEAD
Your deliverables:

→ Documentation Strategy & Information Architecture:
   • Documentation audience segmentation:
     - End users (customers)
     - Administrators/power users
     - Developers (API consumers)
     - Internal team members
     - System administrators
   
   • Documentation hierarchy:
     Level 1: Getting Started (Quick start, tutorials)
     Level 2: How-To Guides (Task-based, step-by-step)
     Level 3: Reference (API docs, feature reference, settings)
     Level 4: Explanation (Concepts, architecture, best practices)
   
   • Documentation platform selection:
     - Options: GitBook, ReadMe, Docusaurus, Confluence, custom
     - Chosen platform: [X]
     - Justification: [Why]
     - URL structure: docs.company.com
   
   • Documentation versioning strategy:
     - Version docs alongside product versions
     - Legacy version docs accessible
     - Version switcher in docs UI

→ End-User Documentation:
   
   GETTING STARTED GUIDE:
   - Title: "Getting Started with [Product Name]"
   - Content:
     * What is [Product]? (30-second elevator pitch)
     * Who is it for? (Use cases, personas)
     * Key features overview
     * Account creation walkthrough (with screenshots)
     * First successful action (time-to-value < 5 minutes)
     * Next steps (links to relevant how-to guides)
   - Format: Web page + PDF download
   - Estimated reading time: 5-10 minutes
   
   FEATURE DOCUMENTATION:
   For EACH major feature:
   - Feature name and description
   - Use case (when and why to use this feature)
   - Step-by-step instructions (with annotated screenshots)
   - Common issues and troubleshooting
   - Related features
   - Video tutorial (if applicable)
   
   Example: "How to Create Your First Project"
   1. Click "New Project" button (screenshot with button highlighted)
   2. Enter project name and description
   3. Select project template (screenshot)
   4. Configure project settings (screenshot)
   5. Click "Create Project"
   6. Success! Next steps: [links]
   
   HOW-TO GUIDES (Task-Based):
   - How to invite team members
   - How to configure notifications
   - How to export your data
   - How to integrate with [Third Party]
   - How to customize your dashboard
   - How to set up automation rules
   - [List all critical user tasks]
   
   TROUBLESHOOTING GUIDE:
   Common Issues:
   - Issue: "I can't log in"
     * Possible causes: Wrong password, account not verified, account locked
     * Solutions: Reset password link, resend verification email, contact support
   - Issue: "Feature X is not working"
     * Diagnostic steps: Check browser console, check network tab, disable extensions
     * Solutions: Clear cache, try different browser, check status page
   - Issue: "I can't find feature Y"
     * Solutions: Search documentation, check if feature available on your plan
   - [List top 20 support issues from beta/early users]
   
   FAQ (Frequently Asked Questions):
   - Billing FAQs (pricing, refunds, upgrades/downgrades)
   - Account FAQs (deletion, data export, privacy)
   - Technical FAQs (browser support, mobile app, integrations)
   - Feature FAQs (common questions per major feature)
   - Minimum 50 FAQ entries covering 80% of expected questions
   
   GLOSSARY:
   - All product-specific terminology defined
   - Industry jargon explained (for non-experts)
   - Acronym expansion
   - Example: "Webhook: An automated message sent from one app to another 
     when a specific event occurs."

→ Administrator Documentation:
   
   ADMIN GUIDE:
   - User management (add/remove users, roles, permissions)
   - Organization settings
   - Billing and subscription management
   - Usage analytics and reporting
   - Security settings (2FA enforcement, SSO configuration, IP allowlisting)
   - Audit log access
   - Data retention policies
   - Integration management
   
   SECURITY BEST PRACTICES:
   - Password policies
   - 2FA/MFA setup
   - Access control recommendations
   - Data encryption settings
   - Compliance configurations (GDPR, HIPAA, etc.)

→ Developer Documentation (API):
   
   API DOCUMENTATION:
   - API Overview:
     * API architecture (REST, GraphQL, gRPC)
     * Base URLs (production, sandbox)
     * API versioning strategy
     * Rate limiting policy
     * Changelog
   
   - Authentication Guide:
     * How to obtain API keys / tokens
     * Authentication methods (Bearer token, OAuth 2.0, API keys)
     * Token expiration and refresh
     * Security best practices
   
   - API Reference (OpenAPI/Swagger):
     * Complete endpoint documentation (auto-generated from Phase 1.5 spec)
     * Request/response examples (happy path + error cases)
     * Code examples in multiple languages:
       - cURL
       - JavaScript (Node.js, fetch, axios)
       - Python (requests)
       - Ruby
       - PHP
       - Java
       - Go
     * Interactive API explorer (Swagger UI)
   
   - Error Handling Guide:
     * Error code reference (all error codes documented)
     * Error response format
     * Retry logic recommendations
     * Rate limit handling
   
   - Webhooks Documentation:
     * Available webhook events
     * Webhook payload format (with examples)
     * Webhook signature verification
     * Retry policy
     * Webhook debugging tips
   
   - SDKs and Client Libraries:
     * Official SDK documentation (if available)
     * Community SDKs (if applicable)
     * SDK installation and setup
     * SDK code examples
   
   - Tutorials:
     * "Build your first integration in 10 minutes"
     * "Common integration patterns"
     * "Production checklist for API consumers"
   
   - API Migration Guides (when API versions change):
     * What's changed
     * Breaking changes
     * Migration steps
     * Deprecation timeline

→ Internal Documentation (Team):
   
   ARCHITECTURE DOCUMENTATION:
   - System architecture diagram
   - Technology stack documentation
   - Infrastructure architecture
   - Data flow diagrams
   - Third-party integrations
   - (Reference Phase 2 deliverables)
   
   DEVELOPMENT GUIDELINES:
   - Code style guide (linting rules, formatting)
   - Git workflow (branching strategy, PR process)
   - Testing standards (unit, integration, E2E)
   - Code review checklist
   - Documentation standards (inline comments, README files)
   
   OPERATIONS RUNBOOKS:
   - (Reference Phase 8.5 deliverables)
   - Deployment procedures
   - Monitoring and alerting
   - Incident response
   - Database management
   
   ONBOARDING DOCUMENTATION (New Team Members):
   - Welcome guide
   - Development environment setup
   - Codebase tour
   - Architecture overview
   - First contribution guide
   - Team contacts and resources

→ Video Tutorials & Screencasts:
   • Video content strategy:
     - Platform: YouTube, Vimeo, Wistia, or custom
     - Video types:
       * Product overview (2-3 minutes)
       * Feature walkthroughs (5-10 minutes each)
       * Advanced use cases (10-15 minutes)
       * Webinar recordings (30-60 minutes)
   
   • Video production:
     - Scriptwriting (script all videos for consistency)
     - Screen recording (with voice-over)
     - Video editing (professional or in-house)
     - Accessibility: Closed captions (CC) for all videos
   
   • Video library:
     - Minimum 5 core videos:
       1. Product introduction (what, why, for whom)
       2. Getting started (account setup to first success)
       3. Feature spotlight (most popular feature)
       4. Integration tutorial (common integration)
       5. Tips and tricks (power user features)

→ Documentation Maintenance Plan:
   • Documentation review cycle: Quarterly
   • Documentation ownership: Each feature owner maintains their docs
   • Documentation as code:
     - Docs stored in Git repository
     - Docs reviewed in pull requests (like code)
     - Docs versioned alongside product
     - Automated dead link checking
     - Automated screenshot update detection (flag outdated screenshots)
   
   • Documentation metrics:
     - Page views (which docs most viewed)
     - Search queries (what users searching for)
     - Feedback (thumbs up/down on each doc page)
     - Support ticket deflection (did docs reduce tickets?)
   
   • Continuous improvement:
     - User feedback incorporated monthly
     - Support team highlights doc gaps weekly
     - Documentation backlog prioritized

🎓 TRAINING SPECIALIST
Your deliverables:

→ Training Program Strategy:
   • Training audience segmentation:
     - End users (customers) - self-serve + live training
     - Administrators - live training + certification
     - Developers/API consumers - workshops + office hours
     - Internal team (support, sales) - mandatory training
     - Partners/resellers - partner enablement program
   
   • Training formats:
     - Self-paced online courses (video + quizzes)
     - Live webinars (weekly or monthly)
     - In-person training (enterprise customers only)
     - Office hours (live Q&A, bring your questions)
     - Certification program (for power users/admins)

→ End-User Training Program:
   
   SELF-PACED ONLINE COURSE:
   - Platform: Teachable, Thinkific, LMS, or custom
   - Course structure:
     
     Module 1: Introduction (15 minutes)
     - Welcome and course overview
     - What you'll learn
     - How to get the most from this course
     
     Module 2: Getting Started (30 minutes)
     - Account setup
     - UI navigation
     - First project creation
     - Hands-on exercise: Create your first [X]
     - Quiz (5 questions)
     
     Module 3: Core Features (45 minutes)
     - Feature A walkthrough
     - Feature B walkthrough
     - Feature C walkthrough
     - Hands-on exercise: Complete a realistic task
     - Quiz (10 questions)
     
     Module 4: Advanced Features (30 minutes)
     - Advanced feature walkthrough
     - Integration setup
     - Automation configuration
     - Hands-on exercise
     - Quiz (10 questions)
     
     Module 5: Best Practices (20 minutes)
     - Tips and tricks
     - Common pitfalls to avoid
     - Optimization strategies
     - Where to get help
     
     Final Assessment (15 questions)
     - Pass: 80% or higher
     - Certificate of completion issued
   
   - Course assets:
     * Video lessons (professionally produced)
     * Downloadable resources (cheat sheets, templates)
     * Interactive exercises (sandbox environment)
     * Quizzes and assessments
     * Discussion forum (peer-to-peer learning)

   LIVE WEBINAR SERIES:
   - Frequency: Weekly or bi-weekly
   - Duration: 45-60 minutes (30-40 min presentation + 15-20 min Q&A)
   - Topics:
     * Week 1: Product overview and getting started
     * Week 2: Feature deep-dive (Feature A)
     * Week 3: Feature deep-dive (Feature B)
     * Week 4: Integrations and automation
     * Week 5: Advanced use cases
     * Repeat or rotate topics monthly
   
   - Webinar platform: Zoom, WebEx, Google Meet
   - Recording: All webinars recorded and published
   - Follow-up: Slides and resources emailed to attendees

   OFFICE HOURS:
   - Frequency: Weekly
   - Format: Drop-in Q&A session (no formal presentation)
   - Duration: 30 minutes
   - Purpose: Users bring their specific questions
   - Hosted by: Product team, support team, or community manager

→ Administrator/Power User Training:
   
   ADMIN CERTIFICATION PROGRAM:
   - Program name: "[Product] Certified Administrator"
   - Program structure:
     * Self-paced learning (online course)
     * Live training workshop (4 hours)
     * Hands-on lab exercises
     * Final exam (proctored or honor system)
     * Pass: 85% or higher
   
   - Certification benefits:
     * Certificate and digital badge
     * Listed in certified administrator directory
     * Access to exclusive admin community
     * Early access to new features (beta program)
   
   - Recertification:
     * Required annually (for major product updates)
     * Abbreviated re-certification process

→ Developer Training:
   
   API WORKSHOP:
   - Format: Live, hands-on workshop
   - Duration: 2-3 hours
   - Agenda:
     * Hour 1: API overview, authentication, first API call
     * Hour 2: Common use cases, code examples, best practices
     * Hour 3: Build a mini-integration (hands-on)
   
   - Workshop materials:
     * Starter code repository (GitHub)
     * API sandbox environment
     * Workshop slides
     * Code examples in multiple languages
   
   - Follow-up:
     * Office hours for API questions
     * Developer community (Discord, Slack)
     * Code review offer (submit your integration for feedback)

→ Internal Team Training:
   
   SUPPORT TEAM TRAINING:
   - Mandatory for all support staff
   - Duration: 2-day intensive training
   - Day 1:
     * Product overview and philosophy
     * Feature deep-dives (all major features)
     * Hands-on exercises (become a power user)
     * Common customer scenarios
   - Day 2:
     * Troubleshooting techniques
     * Escalation procedures
     * Knowledge base navigation
     * CRM/ticketing system training
     * Role-playing exercises (mock support tickets)
   
   - Ongoing training:
     * Weekly product update sessions (new features)
     * Monthly refresher training
     * Shadowing program (new hires shadow senior support)
   
   SALES TEAM TRAINING:
   - Product training (features, benefits, use cases)
   - Demo script and best practices
   - Objection handling
   - Competitive positioning
   - Pricing and packaging
   - Sales enablement materials (slide decks, battle cards, case studies)

→ Training Materials Repository:
   • Central repository: [Google Drive, Notion, Confluence, SharePoint]
   • Materials organized by:
     - Audience (end user, admin, developer, internal)
     - Format (video, PDF, slide deck, code sample)
     - Topic (feature, use case, role)
   
   • Materials include:
     - Slide decks (editable PowerPoint/Google Slides)
     - Video recordings
     - Handouts and worksheets
     - Cheat sheets and quick reference guides
     - Code samples and starter templates
     - Assessment quizzes
     - Certification exams

→ Train-the-Trainer Program (for Enterprise Customers):
   • For large enterprise customers who want to train internally
   • Program includes:
     - Training materials provided (white-labeled)
     - Train-the-trainer session (certify customer's internal trainers)
     - Ongoing support for customer trainers
     - Materials updated as product evolves

🔄 CHANGE MANAGEMENT LEAD
Your deliverables:

→ Change Management Strategy:
   • Change impact assessment:
     - Who is affected by this new product/feature?
       * End users (customers)
       * Internal teams (support, sales, ops)
       * Partners/integrators
       * Executives/leadership
     - What changes for each stakeholder group?
       * New workflows
       * New tools to learn
       * Deprecated old workflows/tools
     - Level of impact: High / Medium / Low
   
   • Change readiness assessment:
     - How ready is each stakeholder group for this change?
     - What concerns or resistance anticipated?
     - What enablers exist (champions, early adopters)?
   
   • Change management approach selection:
     - ADKAR Model (Awareness, Desire, Knowledge, Ability, Reinforcement)
     - Kotter's 8-Step Process
     - Prosci Methodology
     - Custom approach
     - Chosen approach: [X] with justification

→ Stakeholder Communication Plan:
   
   COMMUNICATION TIMELINE:
   
   T-30 Days (Before Launch):
   - Internal announcement to all employees
   - Leadership briefing (executive presentation)
   - Champion identification (early adopters, influencers)
   - FAQ preparation
   
   T-14 Days:
   - Customer announcement (email campaign)
   - Blog post / press release
   - Social media teasers
   - Webinar registration open
   - Documentation published (pre-release)
   
   T-7 Days:
   - Reminder communications
   - Support team final training
   - Sales team enablement complete
   - FAQ updated based on questions received
   
   T-0 (Launch Day):
   - Launch announcement (email, blog, social, press)
   - In-app notifications/banners
   - Launch webinar (live)
   - Product hunt / public launch activities
   
   T+7 Days (Post-Launch):
   - Success stories highlighted
   - Usage metrics shared
   - Feedback collection
   - Issues addressed
   
   T+30 Days:
   - Retrospective and lessons learned
   - Adoption metrics reviewed
   - Next iteration planning
   
   COMMUNICATION CHANNELS:
   - Email (segmented lists: all users, admins, developers)
   - In-app messaging (banners, modals, tooltips)
   - Blog / company website
   - Social media (Twitter, LinkedIn, Facebook)
   - Community forum / user group
   - Webinars / virtual events
   - Press / media outreach
   - Customer success outreach (high-touch customers)

→ Adoption Strategy:
   
   ADOPTION METRICS & GOALS:
   - Metric 1: Activation rate (% of users who complete setup)
     * Baseline: [X%]
     * Goal: [Y%]
     * Timeline: Within 30 days of launch
   
   - Metric 2: Feature adoption (% of users who use new feature)
     * Goal: [X%] of active users
     * Timeline: Within 60 days
   
   - Metric 3: Daily active users (DAU)
     * Baseline: [X users]
     * Goal: [Y users]
     * Timeline: 90 days post-launch
   
   - Metric 4: User satisfaction (NPS, CSAT)
     * Goal: NPS > [X], CSAT > [Y]/10
     * Measurement: Surveys at 7, 30, 90 days post-launch
   
   - Metric 5: Support ticket volume
     * Goal: < [X] tickets/day related to new feature
     * Indicates: Good documentation and training
   
   ADOPTION TACTICS:
   - Onboarding flow redesign (to highlight new features)
   - In-app tooltips and guided tours
   - Email drip campaign (educate users over time)
   - Incentives (gamification, badges, rewards for trying new features)
   - Champions program (identify and empower power users)
   - Case studies and success stories (social proof)
   - Community engagement (user forum, user group events)

→ Resistance Management:
   • Common sources of resistance:
     - "The old way was better" (feature parity perception)
     - "This is too complicated" (learning curve resistance)
     - "I don't have time to learn this" (bandwidth concerns)
     - "This doesn't work for my use case" (edge case concerns)
   
   • Resistance mitigation strategies:
     - Acknowledge concerns (empathy, validation)
     - Communicate benefits clearly (WIIFM - What's In It For Me)
     - Provide migration paths (don't abandon old workflows abruptly)
     - Offer extra support (office hours, 1:1 sessions for resisters)
     - Highlight early wins (success stories from peers)
     - Patience and iteration (some users need more time)

→ Feedback Collection & Iteration:
   
   FEEDBACK CHANNELS:
   - In-app feedback widget (always available)
   - Post-launch survey (7 days after launch)
   - User interviews (with select users)
   - Support ticket analysis (themes and patterns)
   - Community forum monitoring
   - Social media listening
   - Analytics (behavior data: what are users actually doing?)
   
   FEEDBACK TRIAGE PROCESS:
   - Daily: Review all feedback
   - Categorize: Bug / Feature request / UX issue / Documentation gap / Training need
   - Prioritize: High / Medium / Low
   - Action: 
     * Bugs → Engineering (immediate fix)
     * Quick wins → Product (next sprint)
     * Documentation gaps → Tech Writing (update within 48 hours)
     * Training needs → Create additional content
   - Close loop: Respond to user who provided feedback
   
   ITERATION PLAN:
   - Week 1 post-launch: Daily feedback review, hotfixes deployed
   - Week 2-4: Weekly iteration releases (based on feedback)
   - Month 2-3: Bi-weekly iterations
   - Month 4+: Normal release cadence with learnings incorporated

→ Change Management Success Criteria:
   • Quantitative success criteria:
     - Adoption rate: [X%] of users actively using new product/feature
     - Satisfaction score: NPS > [X], CSAT > [Y]
     - Reduced support burden: < [Z] tickets/day
     - Time-to-value: Users achieve first success in < [N] minutes
   
   • Qualitative success criteria:
     - Positive user sentiment (reviews, testimonials)
     - No major backlash or resistance
     - Internal teams confident in supporting the change
     - Leadership satisfied with rollout

🎬 CONTENT CREATOR / UX WRITER
Your deliverables:

→ In-App Onboarding Content:
   
   FIRST-RUN EXPERIENCE:
   - Welcome screen:
     * Headline: "Welcome to [Product]!"
     * Subheading: Brief value proposition (one sentence)
     * CTA: "Get Started" button
   
   - Account setup flow:
     * Progress indicator (Step 1 of 4, etc.)
     * Clear instructions at each step
     * Helper text and tooltips
     * Skip option (for advanced users)
   
   - Interactive tutorial (optional, can dismiss):
     * "Take a 2-minute tour" or "Skip and explore on my own"
     * If tour: Guided walkthrough of key features (tooltips, highlights)
     * If skip: Persistent "Help" button for on-demand guidance
   
   - First success moment:
     * Guide user to complete ONE meaningful action
     * Celebrate success: "🎉 Congratulations! You've created your first [X]"
     * Next steps: "What would you like to do next?" with clear options

→ In-App Microcopy & UX Writing:
   • Button labels: Clear, action-oriented
     - ✅ "Create Project" (not "Submit" or "OK")
     - ✅ "Invite Team Members" (not "Next")
   
   • Error messages: Helpful, not technical
     - ❌ "Error 500: Internal Server Error"
     - ✅ "Something went wrong. We're looking into it. Please try again in a few minutes."
     - Include: What happened, why, what to do next
   
   • Empty states: Encouraging, actionable
     - ❌ "No items"
     - ✅ "You don't have any projects yet. Create your first project to get started!"
     - Include: CTA button to take action
   
   • Loading states: Informative
     - ✅ "Loading your dashboard..."
     - ✅ "Processing your payment... This may take a few seconds."
   
   • Success messages: Celebratory, clear next steps
     - ✅ "✓ Project created successfully! Start adding tasks to your project."
   
   • Helper text: Concise, helpful
     - Tooltips (on hover/tap): Explain unfamiliar terms or complex fields
     - Placeholder text: Show example format
     - Inline help: Expandable help text for complex features

→ Email Content:
   
   TRANSACTIONAL EMAILS:
   - Welcome email (after signup)
   - Email verification
   - Password reset
   - Payment confirmation
   - Subscription renewal reminder
   - Account activity notifications
   - Data export ready
   
   Each email includes:
   - Clear subject line (action-oriented if action required)
   - Preheader text (optimized for mobile preview)
   - Personalization (Hi [First Name])
   - Clear primary message
   - Clear CTA button
   - Footer (unsubscribe, contact support, legal links)
   
   EDUCATIONAL/NURTURE EMAILS:
   - Onboarding drip campaign (Days 0, 3, 7, 14, 30)
   - Feature announcements
   - Tips and tricks
   - Case studies and success stories
   - Re-engagement campaigns (for inactive users)

→ Launch Announcement Content:
   
   BLOG POST (Launch Announcement):
   - Title: "[Product Name] is Now Available!"
   - Structure:
     * What: Brief product description
     * Why: Problem it solves, why we built it
     * Who: Target audience
     * How: Getting started (with link)
     * When: Availability (now, or date)
     * Call to action: "Sign up today"
   - Length: 500-800 words
   - Visuals: Hero image, feature screenshots, demo video
   
   PRESS RELEASE (if applicable):
   - Standard press release format
   - Approved by legal/PR team
   - Distributed to media contacts
   
   SOCIAL MEDIA POSTS:
   - Twitter/X: Announcement thread (5-7 tweets)
   - LinkedIn: Professional announcement post
   - Facebook: Community-focused post
   - Instagram: Visual showcase (if applicable)
   - Each platform optimized for format and audience

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 END USER ADVOCATE CRITIC
Your critique must identify:
→ Documentation understandable to non-technical users?
→ Jargon explained or avoided?
→ Screenshots clear and up-to-date?
→ Documentation searchable and well-organized?
→ Videos have captions (accessibility)?
→ Specific critique: "API documentation assumes developer knows OAuth 2.0 - 
   need beginner-friendly explanation or link to external OAuth guide"

🔴 CUSTOMER SUPPORT LEAD CRITIC
Your critique must identify:
→ Documentation covers top support questions?
→ Troubleshooting section adequate?
→ FAQ comprehensive?
→ Support team trained adequately?
→ Knowledge base gaps that will generate tickets?
→ Specific critique: "No documentation for [specific common issue discovered 
   in beta] - will result in 50+ support tickets per week"

🔴 PRODUCT MANAGER CRITIC
Your critique must identify:
→ Training program drives adoption?
→ Onboarding flow intuitive?
→ Change management plan addresses resistance?
→ Success metrics realistic and measurable?
→ Feedback loops established?
→ Specific critique: "Change management plan assumes users will opt-in to 
   training - need mandatory onboarding flow for critical features"

🔴 UX RESEARCHER CRITIC
Your critique must identify:
→ Documentation tested with real users?
→ Onboarding flow tested with users?
→ Microcopy clear and helpful?
→ Error messages actually help users recover?
→ User journey from zero to success validated?
→ Specific critique: "No user testing of documentation - likely has gaps 
   and confusing sections that won't be discovered until post-launch"

🔴 ACCESSIBILITY SPECIALIST CRITIC
Your critique must identify:
→ All videos have captions?
→ Documentation WCAG 2.2 AA compliant?
→ Training materials accessible (screen reader compatible)?
→ Alt text on all images?
→ Keyboard navigation in interactive tutorials?
→ Specific critique: "Video tutorials have no captions - excludes deaf/hard 
   of hearing users, violates accessibility standards"

🔴 DEVELOPER ADVOCATE CRITIC (for API docs)
Your critique must identify:
→ API docs complete and accurate?
→ Code examples work (tested)?
→ Code examples in enough languages?
→ API docs easy to navigate?
→ Interactive API explorer functional?
→ Specific critique: "Code examples only in JavaScript - need Python, Ruby, 
   PHP examples to serve wider developer audience"

🔴 INTERNAL TEAM LEAD CRITIC
Your critique must identify:
→ Internal documentation maintained?
→ Team onboarding efficient?
→ Runbooks accessible and up-to-date?
→ Tribal knowledge documented?
→ Specific critique: "No single source of truth for internal docs - scattered 
   across Confluence, Google Docs, Notion, and Slack - will cause confusion"

🔴 CHANGE MANAGEMENT EXPERT CRITIC
Your critique must identify:
→ Stakeholder analysis complete?
→ Communication plan comprehensive?
→ Resistance management strategies realistic?
→ Adoption metrics actionable?
→ Feedback loops will surface issues?
→ Specific critique: "Adoption metrics don't include engagement depth - 
   users might 'try' feature once but not adopt it meaningfully"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ COMPREHENSIVENESS: /10
   Is all necessary documentation/training provided?

→ CLARITY: /10
   Is content understandable to target audience?

→ ACCESSIBILITY: /10
   Is content accessible to all users (including disabilities)?

→ USABILITY: /10
   Can users find what they need quickly?

→ EFFECTIVENESS: /10
   Will this drive adoption and reduce support burden?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 9 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Complete Documentation Suite
   - End-user documentation (Getting Started, How-Tos, Reference, FAQ)
   - Administrator documentation
   - Developer/API documentation
   - Internal team documentation
   - All documentation published and accessible

2. Video Tutorial Library
   - Minimum 5 core video tutorials
   - All videos captioned (accessibility)
   - Videos published on platform

3. Training Program Materials
   - Self-paced online course (if applicable)
   - Webinar slide decks
   - Workshop materials
   - Certification exam (if applicable)
   - Training materials repository organized

4. Change Management Plan
   - Stakeholder communication plan
   - Communication timeline and templates
   - Adoption strategy and metrics
   - Resistance management plan
   - Feedback collection mechanisms

5. In-App Content
   - Onboarding flow (designed and implemented)
   - Microcopy and UX writing (all screens)
   - Tooltips and helper text
   - Error messages (helpful and tested)
   - Empty states and loading states

6. Launch Communication Assets
   - Blog post / press release
   - Email templates (announcement, nurture campaign)
   - Social media posts
   - Internal announcement materials

7. Support Team Training Materials
   - Training documentation
   - Training session completed (team trained)
   - Support knowledge base updated

8. Documentation Maintenance Plan
   - Review cycle defined
   - Ownership assigned
   - Metrics tracking configured

9. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 9 COMPLETE when:
   → All critics score ≥ 9/10
   → All documentation published and accessible
   → All training materials ready
   → Support team trained
   → Launch communication assets approved
   → Onboarding flow tested with real users
   → Accessibility audit passed (WCAG 2.2 AA)
   → Video captions complete
   → Change management plan approved by stakeholders

🚫 Phase 9 BLOCKED if:
   → Any critic scores < 9/10
   → Critical documentation gaps
   → Training materials incomplete
   → Accessibility issues unresolved
   → Support team not trained
   → User testing reveals major usability issues

⚠️  ONGOING REQUIREMENTS POST-LAUNCH:
   □ Documentation updated with every product release
   □ Training materials refreshed quarterly
   □ New videos created for major features
   □ Support team training ongoing (weekly product updates)
   □ Adoption metrics reviewed weekly
   □ User feedback incorporated monthly

[PHASE 9 COMPLETE → PROCEED TO LAUNCH & POST-LAUNCH MONITORING]
```

---

## 📋 PHASE 10 — RETROSPECTIVE & CONTINUOUS LEARNING PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 10: PROJECT RETROSPECTIVE & LESSONS LEARNED           ║
║  Goal: Capture learnings to improve the next build           ║
║  Input: Completed project + 30-90 days post-launch data      ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]

NOTE: This phase occurs 30-90 days post-launch to allow time for 
real-world data and insights to accumulate.

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━

🔍 SCRUM MASTER / AGILE COACH (Retrospective Facilitator)
Your deliverables:

→ Retrospective Planning:
   • Retrospective format selection:
     - Start-Stop-Continue
     - Mad-Sad-Glad
     - 4Ls (Liked, Learned, Lacked, Longed For)
     - Sailboat (Wind, Anchor, Rocks, Island)
     - Timeline Retrospective
     - Chosen format: [X] with justification
   
   • Participant list:
     - Core team (all engineers, designers, product, QA)
     - Extended team (DevOps, security, support, marketing)
     - Stakeholders (leadership, if appropriate)
     - Total participants: [X people]
   
   • Session logistics:
     - Duration: 2-3 hours (for comprehensive retrospective)
     - Format: In-person or virtual (Miro, Mural, Retrium)
     - Scheduled: [Date and time]
     - Pre-work: Participants reflect individually before session

→ Retrospective Session Facilitation:
   
   AGENDA:
   
   1. SET THE STAGE (10 minutes)
      - Welcome and purpose
      - Ground rules:
        * Vegas rule: What's said here, stays here
        * Blameless culture: Focus on systems, not people
        * Respect and psychological safety
        * Everyone participates
        * Focus on improvement, not finger-pointing
      - Icebreaker activity (if appropriate)
   
   2. GATHER DATA (30 minutes)
      - Individual reflection (5 minutes silent writing)
      - Share observations:
        * What went well?
        * What didn't go well?
        * What surprised us?
        * What did we learn?
      - Capture all input (sticky notes, Miro board, shared doc)
      - No discussion yet, just data gathering
   
   3. GENERATE INSIGHTS (45 minutes)
      - Group similar themes
      - Identify patterns
      - Discuss root causes (5 Whys technique)
      - Vote on most important topics (dot voting)
      - Deep dive on top 5-7 topics
   
   4. DECIDE WHAT TO DO (30 minutes)
      - For each major insight, identify:
        * What should we keep doing? (successes to repeat)
        * What should we stop doing? (failures to avoid)
        * What should we start doing? (improvements to make)
      - Prioritize actions (impact vs effort matrix)
      - Create SMART action items:
        * Specific
        * Measurable
        * Assignable (who owns this?)
        * Realistic
        * Time-bound (due date)
   
   5. CLOSE THE RETROSPECTIVE (15 minutes)
      - Summarize key takeaways
      - Review action items (who, what, when)
      - Appreciation round (thank team members)
      - Retrospective feedback (how was this retro?)

→ Retrospective Output Document:
   
   RETROSPECTIVE REPORT STRUCTURE:
   
   1. EXECUTIVE SUMMARY
      - Project overview
      - Overall success assessment
      - Top 3 wins
      - Top 3 learnings
      - Key action items
   
   2. PROJECT OVERVIEW
      - Project goals (from Phase 0)
      - Timeline (planned vs actual)
      - Budget (planned vs actual)
      - Scope (planned vs actual)
   
   3. WHAT WENT WELL ✅
      - Success 1: [Description]
        * Why it worked
        * How to replicate in future projects
      - Success 2: [Description]
      - Success 3: [Description]
      - [List all successes]
   
   4. WHAT DIDN'T GO WELL ❌
      - Challenge 1: [Description]
        * Impact (what was the consequence?)
        * Root cause (5 Whys analysis)
        * How to prevent in future
      - Challenge 2: [Description]
      - Challenge 3: [Description]
      - [List all challenges]
   
   5. SURPRISES & LEARNINGS 💡
      - Surprise 1: [What we didn't expect]
      - Learning 1: [What we discovered]
      - [List all surprises and learnings]
   
   6. METRICS & OUTCOMES
      - Success metrics review (from Phase 0)
        * Metric 1: Goal [X], Actual [Y], Status [Met/Missed/Exceeded]
        * Metric 2: [Same format]
      - Business outcomes
        * Revenue impact
        * User acquisition/retention
        * Market share
        * Customer satisfaction
      - Technical outcomes
        * Performance (latency, error rate, uptime)
        * Security (incidents, vulnerabilities)
        * Code quality (test coverage, tech debt)
   
   7. TEAM DYNAMICS & PROCESS
      - What worked in our process?
      - What slowed us down?
      - Communication effectiveness
      - Collaboration quality
      - Tools and tooling
      - Meeting effectiveness
   
   8. STAKEHOLDER FEEDBACK
      - Customer feedback themes
      - Internal stakeholder feedback
      - Leadership perspective
   
   9. ACTION ITEMS (PRIORITIZED)
      - Action Item 1:
        * What: [Specific action]
        * Why: [Justification]
        * Who: [Owner]
        * When: [Due date]
        * How we'll measure success: [Metric]
      - Action Item 2: [Same format]
      - [Prioritized list of 5-10 action items]
   
   10. APPENDIX
       - Full participant list
       - Raw retrospective data (all sticky notes, comments)
       - Supporting data (charts, graphs, metrics)

📊 TECHNICAL PROGRAM MANAGER
Your deliverables:

→ Schedule Variance Analysis:
   
   PLANNED VS ACTUAL TIMELINE:
   
   Phase                 | Planned Duration | Actual Duration | Variance
   ----------------------|------------------|-----------------|----------
   Phase 0: Ideation     | 2 weeks          | 3 weeks         | +1 week
   Phase 1: Requirements | 3 weeks          | 4 weeks         | +1 week
   Phase 2: Architecture | 2 weeks          | 2 weeks         | 0
   Phase 3: UX/UI Design | 4 weeks          | 5 weeks         | +1 week
   Phase 4: Development  | 12 weeks         | 16 weeks        | +4 weeks
   Phase 5: Security     | 1 week           | 2 weeks         | +1 week
   Phase 6: QA/Testing   | 3 weeks          | 3 weeks         | 0
   Phase 7: Deployment   | 1 week           | 1 week          | 0
   Phase 8: Beta Program | 6 weeks          | 8 weeks         | +2 weeks
   ----------------------|------------------|-----------------|----------
   TOTAL                 | 34 weeks         | 44 weeks        | +10 weeks
   
   VARIANCE ANALYSIS:
   - Why did we exceed planned timeline?
     * Reason 1: Scope creep in Phase 4 (3 major features added mid-project)
     * Reason 2: Phase 3 required 2 redesign iterations (user testing revealed issues)
     * Reason 3: Phase 8 beta exit criteria took longer to meet than expected
   
   - What phases were on time and why?
     * Phase 2: Architecture was well-scoped, no surprises
     * Phase 6: QA team was well-prepared, automation in place
     * Phase 7: Deployment runbook was thorough, no issues
   
   - Lessons for future timeline estimation:
     * Add 25% buffer for development phases (code always takes longer)
     * Build in iteration time for design phases (rarely get it right first time)
     * Beta exit criteria should have flexible timeline (can't force users to adopt faster)

→ Budget Variance Analysis:
   
   PLANNED VS ACTUAL BUDGET:
   
   Category              | Planned Budget | Actual Spend | Variance    | % Variance
   ----------------------|----------------|--------------|-------------|------------
   Personnel (salaries)  | $500,000       | $580,000     | +$80,000    | +16%
   Cloud infrastructure  | $50,000        | $65,000      | +$15,000    | +30%
   Third-party services  | $30,000        | $28,000      | -$2,000     | -7%
   Tools & software      | $20,000        | $22,000      | +$2,000     | +10%
   Marketing/launch      | $40,000        | $35,000      | -$5,000     | -13%
   Contingency (10%)     | $64,000        | $0           | -$64,000    | -100%
   ----------------------|----------------|--------------|-------------|------------
   TOTAL                 | $704,000       | $730,000     | +$26,000    | +4%
   
   VARIANCE ANALYSIS:
   - Why did we exceed planned budget?
     * Personnel: Project took 10 weeks longer = more salary costs
     * Cloud: Underestimated beta user load, needed more infrastructure
   
   - Where did we save money?
     * Marketing: Organic traction was better than expected, reduced paid ads
     * Third-party services: Negotiated better rate with payment processor
   
   - Lessons for future budget estimation:
     * Personnel is biggest cost driver - timeline slippage directly impacts budget
     * Cloud costs scale with users - need more accurate user growth projections
     * Always use contingency buffer (we did, which kept us only 4% over)

→ Risk Register Review:
   
   IDENTIFIED RISKS (from Phase 0) - WHAT MATERIALIZED?
   
   Risk ID | Risk Description                    | Likelihood | Impact | Mitigation Plan          | Did it occur? | Actual impact
   --------|-------------------------------------|------------|--------|--------------------------|---------------|---------------
   R-001   | Third-party API changes breaking us | Medium     | High   | Abstraction layer        | No            | N/A - mitigation worked
   R-002   | Key engineer leaves mid-project     | Low        | High   | Knowledge sharing, docs  | Yes           | 2 week delay, but recovered
   R-003   | Security vulnerability discovered   | Medium     | High   | Security reviews, pen testing | Yes      | 1 week delay to patch
   R-004   | User adoption lower than expected   | Medium     | High   | Beta program, marketing  | No            | Adoption exceeded expectations
   R-005   | Database performance at scale       | High       | Medium | Load testing, optimization | Yes         | Needed emergency scaling, 1 day downtime
   
   LESSONS:
   - Risks we identified and mitigated successfully: R-001, R-002, R-004
   - Risks that materialized despite mitigation: R-003, R-005
   - Risks we didn't identify but should have:
     * R-NEW-1: Beta program marketing underestimated (hard to recruit beta users)
     * R-NEW-2: Mobile app review process delays (Apple approval took 2 weeks)
   
   - For next project:
     * Database performance testing should be earlier (Phase 5.5, not Phase 7)
     * App store review process should be factored into timeline (+2 weeks)

→ Scope Management Analysis:
   
   PLANNED SCOPE VS ACTUAL SCOPE:
   
   Original features (from Phase 1): 25 features
   Features added during development: 8 features (scope creep)
   Features cut/deferred: 3 features
   Final delivered features: 30 features
   
   SCOPE CHANGES:
   - Why did we add features?
     * Beta user feedback revealed critical missing features
     * Competitive analysis mid-project showed we were missing table stakes
     * Stakeholder requests (executive pressure)
   
   - Why did we cut features?
     * Timeline pressure (had to defer non-critical features)
     * Technical complexity higher than estimated
   
   - Impact of scope changes:
     * Timeline: +4 weeks in development
     * Budget: +$50,000 in engineering time
     * Quality: Some rushed features have technical debt
   
   - Lessons:
     * Need stronger scope freeze discipline (after Phase 1, lock scope)
     * Establish clear change control process (any new feature requires trade-off)
     * Beta feedback should inform v2 roadmap, not v1 scope

🏗️ SOLUTION ARCHITECT / TECH LEAD
Your deliverables:

→ Technical Decisions Retrospective:
   
   ARCHITECTURE DECISIONS THAT PAID OFF ✅:
   
   Decision 1: Chose microservices architecture
   - Why we chose it: Scalability, team autonomy
   - Result: Positive - scaled well, teams worked independently
   - Would we do it again? Yes, but with more upfront service boundary definition
   
   Decision 2: PostgreSQL over NoSQL for primary database
   - Why we chose it: Relational data model, ACID guarantees
   - Result: Positive - data integrity strong, no regrets
   - Would we do it again? Yes
   
   Decision 3: React for frontend
   - Why we chose it: Large ecosystem, team expertise
   - Result: Positive - fast development, good performance
   - Would we do it again? Yes
   
   ARCHITECTURE DECISIONS THAT DIDN'T PAY OFF ❌:
   
   Decision 1: Custom authentication instead of Auth0/Cognito
   - Why we chose it: Cost savings, control
   - Result: Negative - took 4 weeks longer, security concerns, ongoing maintenance burden
   - Would we do it differently? Yes - use managed auth service
   - Cost of decision: $80,000 in engineering time + ongoing maintenance
   
   Decision 2: Self-hosted Kubernetes instead of managed (EKS/GKE)
   - Why we chose it: Cost savings, learning opportunity
   - Result: Negative - operational complexity, 2 production incidents due to K8s misconfiguration
   - Would we do it differently? Yes - use managed Kubernetes
   - Cost of decision: 1 engineer full-time on ops, 2 incidents (customer impact)
   
   Decision 3: Optimistic approach to database indexing ("we'll add indexes when needed")
   - Why we chose it: Speed of development
   - Result: Negative - performance issues in production, emergency hotfixes
   - Would we do it differently? Yes - index strategy in Phase 2.5, load testing in Phase 5.5
   
   LESSONS:
   - Build vs buy: For non-differentiating features (auth, email, etc.), buy/use managed services
   - Operational complexity: Managed services worth the cost to reduce ops burden
   - Performance: Can't retrofit performance, must be designed in from the start

→ Technical Debt Created:
   
   INTENTIONAL TECH DEBT (Conscious trade-offs):
   - Debt Item 1: Skipped database migration versioning
     * Why: Speed to market
     * Cost: Will be painful to manage schema changes in future
     * Payback plan: Implement Flyway in Q2 (estimated 2 weeks)
   
   - Debt Item 2: Minimal test coverage on Feature X
     * Why: Timeline pressure
     * Cost: Fragile code, fear of refactoring
     * Payback plan: Add tests in next sprint (1 week)
   
   UNINTENTIONAL TECH DEBT (Didn't realize we were creating it):
   - Debt Item 1: Service coupling (Service A directly calls Service B database)
     * How it happened: Quick solution during crunch time
     * Cost: Can't scale services independently, violates microservices principles
     * Payback plan: Introduce API boundary (3 weeks, Q3 priority)
   
   - Debt Item 2: Frontend state management spaghetti
     * How it happened: No upfront state management architecture
     * Cost: Hard to maintain, hard to onboard new engineers
     * Payback plan: Refactor to Redux Toolkit (4 weeks, Q3)
   
   TOTAL TECH DEBT ESTIMATE: 10 weeks of engineering work
   PLAN: Allocate 20% of each sprint to tech debt paydown

→ Technology Stack Evaluation:
   
   Technology | Purpose | Rating | Comments
   -----------|---------|--------|----------
   React | Frontend framework | 9/10 | Excellent choice, no regrets
   Node.js | Backend runtime | 8/10 | Good, but TypeScript adoption was bumpy
   PostgreSQL | Primary database | 10/10 | Perfect fit, would choose again
   Redis | Caching | 9/10 | Critical for performance, works great
   Kubernetes | Orchestration | 6/10 | Operational complexity too high, consider managed next time
   AWS | Cloud provider | 8/10 | Solid, but costs higher than estimated
   Stripe | Payments | 10/10 | Seamless integration, excellent docs
   SendGrid | Email | 7/10 | Works, but deliverability issues, considering alternatives
   Datadog | Monitoring | 9/10 | Excellent visibility, worth the cost
   
   NEW TOOLS ADOPTED MID-PROJECT:
   - Sentry (error tracking): Should have had from day 1
   - Retool (internal tools): Huge productivity boost for ops team
   - Notion (documentation): Better than Google Docs, team loves it

🔒 CHIEF INFORMATION SECURITY OFFICER (CISO)
Your deliverables:

→ Security Posture Assessment:
   
   SECURITY WINS ✅:
   - Zero security incidents in production (first 90 days)
   - Passed penetration test with only minor findings
   - All OWASP Top 10 mitigations in place
   - Security review process worked (caught vulnerabilities in Phase 5)
   - Encryption at rest and in transit implemented correctly
   
   SECURITY GAPS ❌:
   - Secret management initially weak (hardcoded API keys discovered in code review)
   - Rate limiting not implemented on all endpoints (discovered post-launch)
   - Security logging insufficient (improved after first audit)
   - Dependency vulnerabilities not monitored continuously (now using Snyk)
   
   SECURITY INCIDENTS POST-LAUNCH:
   - Incident 1: [Description]
     * Severity: [P0/P1/P2]
     * Root cause: [What happened]
     * Response: [How we handled it]
     * Outcome: [Resolution]
     * Prevention: [How we prevent recurrence]
   - No security incidents (if applicable - celebrate this!)
   
   LESSONS:
   - Shift-left security worked (catching issues in Phase 5 vs post-launch)
   - Automated security scanning (SAST/DAST) caught 80% of vulnerabilities
   - Manual penetration testing still critical (found issues automation missed)
   - Security training for engineers needed (many common mistakes preventable)
   
   ACTION ITEMS:
   - Implement quarterly security training for all engineers
   - Add pre-commit hooks to prevent secret commits
   - Enable MFA for all production access (currently only 60% adoption)

📈 PRODUCT MANAGER / CPO
Your deliverables:

→ Product Success Metrics Review:
   
   SUCCESS CRITERIA (from Phase 0) - DID WE MEET THEM?
   
   Metric | Goal | Actual (90 days post-launch) | Status | Commentary
   -------|------|------------------------------|--------|------------
   User acquisition | 10,000 users | 12,500 users | ✅ Exceeded | Organic growth better than expected
   Activation rate | 60% | 55% | ❌ Missed | Onboarding flow needs improvement
   DAU/MAU | 40% | 35% | ❌ Missed | Users not returning as frequently as hoped
   NPS | > 30 | 42 | ✅ Exceeded | Users love the product
   Revenue | $100K MRR | $85K MRR | ❌ Missed | Conversion rate lower than expected
   Churn rate | < 5% | 3% | ✅ Exceeded | Retention is strong once users activate
   
   OVERALL ASSESSMENT:
   - User acquisition: Strong (exceeded goal)
   - User activation: Weak (need to improve onboarding)
   - User engagement: Moderate (users like it but don't use it daily)
   - User retention: Strong (once activated, users stay)
   - Revenue: Below target (pricing or conversion issue)
   
   ROOT CAUSE ANALYSIS:
   - Why is activation low?
     * Onboarding too long (takes 15 minutes, should be < 5 minutes)
     * Value not immediately clear (need better first-time UX)
   - Why is engagement lower than expected?
     * Product is not habit-forming (used intermittently, not daily)
     * Push notifications not implemented (users forget to come back)
   - Why is revenue low?
     * Free tier too generous (users not converting to paid)
     * Pricing might be too high (need pricing experimentation)
   
   ACTION ITEMS:
   - Redesign onboarding (Target: 5 minute activation, 70% activation rate)
   - Implement push notifications and email reminders
   - Pricing experimentation (A/B test different price points)
   - Free tier limit tightening (force conversion earlier)

→ User Feedback Themes:
   
   TOP POSITIVE FEEDBACK (What users love):
   1. "The UI is beautiful and intuitive"
   2. "Feature X saves me hours every week"
   3. "Customer support is incredibly responsive"
   4. "Integration with Y is seamless"
   5. "Pricing is fair"
   
   TOP NEGATIVE FEEDBACK (What users complain about):
   1. "Missing feature Z" (mentioned by 40% of users)
   2. "Mobile app is slow" (performance issue)
   3. "Onboarding is confusing" (activation issue - validates metric)
   4. "No dark mode" (UX polish)
   5. "Export functionality is limited"
   
   FEATURE REQUESTS (Top 10):
   1. Feature Z (40% of users)
   2. Advanced reporting (25%)
   3. Mobile app improvements (20%)
   4. Integrations with A, B, C (18%)
   5. Collaboration features (15%)
   6. API access (12%)
   7. White-labeling (enterprise, 5%)
   8. Dark mode (10%)
   9. Bulk operations (8%)
   10. Keyboard shortcuts (7%)
   
   ROADMAP IMPLICATIONS:
   - Q2 Priority: Feature Z (high demand, competitive necessity)
   - Q2 Priority: Onboarding redesign (metrics-driven)
   - Q2 Priority: Mobile performance optimization
   - Q3: Advanced reporting
   - Q3: New integrations

→ Competitive Landscape Changes:
   - Competitor A launched similar product (2 weeks after us)
     * Their approach: [Different/similar positioning]
     * Our competitive advantage: [What we do better]
     * Our vulnerability: [What they do better]
   - Market trends: [Industry shifts observed]
   - Strategic implications: [How this affects our roadmap]

💼 BUSINESS ANALYST / BUSINESS STAKEHOLDER
Your deliverables:

→ Business Outcomes Assessment:
   
   BUSINESS GOALS (from Phase 0) - DID WE ACHIEVE THEM?
   
   Goal | Target | Actual | Status | Notes
   -----|--------|--------|--------|-------
   Increase revenue | +$100K MRR | +$85K MRR | ❌ Missed | 85% of goal, improving month-over-month
   Expand into new market segment | 30% of users from segment Y | 35% | ✅ Exceeded | Successfully attracted target market
   Reduce support costs | -20% | -15% | ❌ Missed | Good progress but more self-serve content needed
   Improve customer satisfaction | NPS +10 points | NPS +15 points | ✅ Exceeded | Customers love the new product
   Competitive positioning | Be in top 3 solutions | Ranked #4 in G2 Grid | ❌ Missed | Close, need more reviews
   
   OVERALL BUSINESS ASSESSMENT:
   - Product-market fit: Strong signals (high NPS, low churn, positive feedback)
   - Revenue: Below target but growing (need to optimize conversion funnel)
   - Market positioning: Improving but not yet top-tier (brand awareness needed)
   - Cost efficiency: Support costs reduced but not as much as hoped
   
   ROI ANALYSIS:
   - Total investment: $730,000 (budget actual)
   - Revenue to date: $255,000 (3 months at avg $85K/month)
   - Projected annual revenue: $1.2M (based on growth trajectory)
   - Payback period: 7-8 months
   - Projected ROI (Year 1): 64% ($1.2M revenue / $730K investment)
   - Assessment: Positive ROI, project justified

→ Lessons Learned (Business Perspective):
   - Market timing was right (demand exists, product needed)
   - Pricing hypothesis partially validated (users willing to pay, but conversion optimization needed)
   - Go-to-market strategy effective (organic growth strong, paid ads less effective)
   - Customer segment hypothesis validated (target segment responded well)

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 EXECUTIVE SPONSOR CRITIC
Your critique must identify:
→ Business value delivered vs. expected?
→ Return on investment acceptable?
→ Lessons learned actionable and valuable?
→ Team morale and sustainability?
→ Would we fund this again knowing what we know now?
→ Specific critique: "Retrospective focuses heavily on execution but light on 
   strategic lessons - need more insight on market positioning and competitive dynamics"

🔴 ENGINEERING MANAGER CRITIC
Your critique must identify:
→ Team burnout or sustainability issues glossed over?
→ Technical debt acknowledged and planned for?
→ Process improvements identified?
→ Team growth and learning captured?
→ Specific critique: "Retrospective doesn't mention the overtime crunch in 
   last 4 weeks before launch - need honest assessment of team health"

🔴 PRODUCT LEADER CRITIC
Your critique must identify:
→ User feedback themes accurately captured?
→ Product-market fit assessment realistic?
→ Roadmap implications clear?
→ Metrics analysis deep enough?
→ Specific critique: "Activation rate missed by 5% but root cause analysis 
   is shallow - need actual user research, not just hypotheses"

🔴 FINANCE / CFO CRITIC
Your critique must identify:
→ Budget variance analysis complete and accurate?
→ ROI calculation methodology sound?
→ Future cost projections included?
→ Cost optimization opportunities identified?
→ Specific critique: "ROI calculation doesn't include ongoing operational 
   costs (cloud, support staff, etc.) - need total cost of ownership analysis"

🔴 TEAM MEMBER (ENGINEER/DESIGNER) CRITIC
Your critique must identify:
→ Team voice represented (not just leadership perspective)?
→ Process pain points captured?
→ Psychological safety maintained in retrospective?
→ Individual contributions recognized?
→ Specific critique: "Retrospective document feels sanitized - the messy, 
   uncomfortable truths are missing (e.g., communication breakdowns, conflicts)"

🔴 CONTINUOUS IMPROVEMENT ADVOCATE CRITIC
Your critique must identify:
→ Action items specific and measurable?
→ Action item owners assigned?
→ Follow-up mechanism exists?
→ Will these lessons actually be applied to next project?
→ Specific critique: "15 action items identified but no prioritization and 
   no capacity plan - likely none will get done, need top 3-5 prioritized"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SCORING RUBRIC (ALL CRITICS)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

→ COMPLETENESS: /10
   Did we capture all important lessons?

→ HONESTY: /10
   Are we being truthful (vs. defensive or sanitized)?

→ ACTIONABILITY: /10
   Are lessons translatable into concrete improvements?

→ INSIGHT DEPTH: /10
   Did we get to root causes vs. surface observations?

→ FORWARD VALUE: /10
   Will this make the next project better?

→ COMPOSITE SCORE: Average (must be ≥ 9.0)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 3 — REMEDIATION & RE-EVALUATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[Standard remediation loop until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 10 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Comprehensive Retrospective Report
   - Executive summary
   - What went well / What didn't go well
   - Metrics and outcomes vs. goals
   - Timeline and budget variance analysis
   - Risk register review
   - Scope management analysis
   - Technical decisions review
   - Technical debt register
   - Security posture assessment
   - Product success metrics
   - User feedback themes
   - Business outcomes assessment

2. Prioritized Action Items (Top 5-10)
   - Each with: What, Why, Who, When, Success criteria
   - Tracked in project management system
   - Follow-up review scheduled (30 days out)

3. Lessons Learned Database Entry
   - Added to organizational knowledge base
   - Tagged and searchable
   - Accessible to future project teams

4. Process Improvement Recommendations
   - SDLC process refinements
   - Tool changes
   - Template updates
   - Training needs

5. Technology Stack Evaluation
   - Keep / Change recommendations
   - New tools to adopt
   - Vendor relationships to reconsider

6. Team Health Assessment
   - Burnout indicators
   - Morale survey results
   - Retention risks
   - Growth opportunities

7. Next Project Recommendations
   - Timeline estimation guidance (based on this project's variance)
   - Budget estimation guidance
   - Risk categories to watch
   - Success patterns to replicate

8. Retrospective Presentation (for leadership)
   - Slide deck summarizing key findings
   - Delivered to executive team
   - Q&A session held

9. Unanimous Score Documentation

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROGRESSION GATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Phase 10 COMPLETE when:
   → All critics score ≥ 9/10
   → Retrospective session held with full team participation
   → Retrospective report complete and reviewed
   → Action items prioritized and assigned
   → Lessons learned documented in knowledge base
   → Presentation to leadership complete
   → Follow-up review scheduled (30 days)

🚫 Phase 10 BLOCKED if:
   → Any critic scores < 9/10
   → Retrospective lacks honesty (sanitized feedback)
   → Action items vague or unassigned
   → Key stakeholders not participated
   → Root cause analysis shallow

⚠️  CONTINUOUS IMPROVEMENT COMMITMENT:
   □ Action items tracked to completion (30-day follow-up)
   □ Lessons applied to next project (referenced in Phase 0)
   □ Process improvements implemented
   □ Retrospective retrospective (how effective was this retro?) - Meta-learning

[PHASE 10 COMPLETE → PROJECT OFFICIALLY CLOSED → BEGIN CONTINUOUS ITERATION OR NEXT PROJECT]
```

---

## 🌐 CROSS-CUTTING CONCERNS INTEGRATION GUIDE

```
╔══════════════════════════════════════════════════════════════╗
║  CROSS-CUTTING CONCERNS INTEGRATION GUIDE                    ║
║  How to Weave Continuous Disciplines Through All Phases      ║
╚══════════════════════════════════════════════════════════════╝

These concerns are NOT separate phases but CONTINUOUS THREADS that 
run through EVERY phase of the SDLC. They must be integrated into 
the Builder Panel and Critique Panel of each phase.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔐 CROSS-CUTTING CONCERN #1: SECURITY (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Shift-Left Security — Security is every engineer's job from day one

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 0: IDEATION & INCEPTION                               │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - CISO participates in Builder Panel
   - Threat modeling (conceptual level)
   - Regulatory requirements identified (GDPR, HIPAA, PCI-DSS, etc.)
   - Data classification (what PII/PHI/sensitive data will we handle?)
   - Security budget allocation

✅ SECURITY CRITIQUE:
   - Security Architect critic reviews threat landscape
   - Compliance Officer critic identifies regulatory risks
   - Privacy implications assessed

✅ DELIVERABLES:
   - Initial threat model (draft)
   - Compliance requirements matrix
   - Security success criteria (zero critical vulnerabilities at launch)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 1: REQUIREMENTS & SPECIFICATION                       │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security Architect in Builder Panel defines security requirements
   - Authentication/authorization requirements specified
   - Data encryption requirements (at rest, in transit, in use)
   - Audit logging requirements
   - Security testing requirements
   - Privacy-by-design requirements

✅ SECURITY CRITIQUE:
   - OWASP Expert critic reviews requirements for OWASP Top 10 coverage
   - Compliance critic ensures regulatory requirements translated to functional requirements

✅ DELIVERABLES:
   - Security Requirements Specification (SRS)
   - Privacy requirements (GDPR Article 25 - Privacy by Design)
   - Security acceptance criteria per user story

┌─────────────────────────────────────────────────────────────┐
│ PHASE 1.5: API DESIGN                                       │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - API Security Architect in Builder Panel
   - OWASP API Top 10 mitigation designed into APIs
   - Rate limiting designed
   - Authentication/authorization per endpoint
   - Input validation requirements

✅ SECURITY CRITIQUE:
   - API Security Specialist critic reviews for API-specific vulnerabilities
   - Pen tester critic identifies attack surfaces

✅ DELIVERABLES:
   - API security review document
   - API authentication/authorization matrix

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: ARCHITECTURE & SYSTEM DESIGN                       │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security Architect in Builder Panel designs security architecture
   - Zero-trust architecture principles
   - Network segmentation and security zones
   - Secret management architecture (Vault, AWS Secrets Manager)
   - Encryption architecture (KMS, TLS, cipher suites)
   - IAM architecture
   - WAF, DDoS protection

✅ SECURITY CRITIQUE:
   - Pen tester critic performs architecture security review
   - Cloud Security Engineer critic reviews cloud security posture

✅ DELIVERABLES:
   - Threat Model (STRIDE methodology, updated from Phase 0)
   - Security Architecture Diagram
   - Data Flow Diagram (with security boundaries)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2.5: DATABASE DESIGN                                  │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - DBA in Builder Panel implements database security hardening
   - Encryption at rest (TDE)
   - Column-level encryption for PII/PHI
   - Database user privilege matrix (least privilege)
   - Audit logging configuration

✅ SECURITY CRITIQUE:
   - Security Architect critic reviews data exposure risks
   - Compliance critic ensures PII/PHI handling compliant

✅ DELIVERABLES:
   - Database Security Hardening Checklist
   - Data classification and encryption matrix

┌─────────────────────────────────────────────────────────────┐
│ PHASE 3: UX/UI DESIGN                                       │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security UX considerations (password strength indicators, MFA UX)
   - No PII/PHI in URLs or client-side logs
   - Session timeout UX
   - Secure by default design

✅ SECURITY CRITIQUE:
   - Security Reviewer critic identifies information disclosure in UI
   - Identifies dark patterns (consent dark patterns, etc.)

✅ DELIVERABLES:
   - Security UX guidelines
   - Privacy-friendly design decisions documented

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - OWASP Security Engineer in Builder Panel
   - Secure coding standards enforced (linting, pre-commit hooks)
   - SAST (Static Application Security Testing) in CI/CD
   - Dependency scanning (Snyk, Dependabot)
   - Secret scanning (prevent credential commits)
   - Security code review (peer review with security checklist)

✅ SECURITY CRITIQUE:
   - OWASP Expert critic reviews code for OWASP Top 10 violations
   - Security Code Reviewer runs SAST tools and manual review

✅ DELIVERABLES:
   - Secure code (OWASP checklist compliance)
   - SAST scan results (all critical findings resolved)
   - Dependency vulnerability scan (all high/critical patched)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 5: SECURITY REVIEW (Dedicated Security Phase)         │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - [SEE PHASE 5 PROMPT - this is the dedicated security review phase]
   - Penetration testing
   - DAST (Dynamic Application Security Testing)
   - Security architecture review
   - Cryptography review
   - Cloud security posture management

┌─────────────────────────────────────────────────────────────┐
│ PHASE 5.5: INTEGRATION & E2E TESTING                        │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security QA Engineer in Builder Panel
   - Security integration tests (auth/authz across services)
   - API security tests (injection, broken auth, etc.)

✅ SECURITY CRITIQUE:
   - Security critic reviews test coverage for security scenarios

┌─────────────────────────────────────────────────────────────┐
│ PHASE 6: QA & TESTING                                       │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security QA Engineer executes security test cases
   - DAST scanning
   - Manual security testing
   - Re-test of security fixes

✅ DELIVERABLES:
   - Security test results (all tests passing)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 7: DEPLOYMENT                                         │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - Security Engineer in Builder Panel
   - Pre-launch security checklist
   - Secret rotation before launch
   - WAF rules configured
   - Security headers configured
   - Certificate verification

✅ DELIVERABLES:
   - Pre-launch security sign-off

┌─────────────────────────────────────────────────────────────┐
│ PHASE 8: POST-LAUNCH MONITORING                             │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - SOC Analyst in Builder Panel
   - Security monitoring (SIEM)
   - Vulnerability scanning (ongoing)
   - Patch management
   - Security incident monitoring

✅ DELIVERABLES:
   - Security monitoring dashboard
   - Incident response readiness (Phase 8.5)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 10: RETROSPECTIVE                                     │
└─────────────────────────────────────────────────────────────┘
✅ SECURITY ACTIVITIES:
   - CISO reviews security posture
   - Security incidents analyzed
   - Security process improvements identified

✅ DELIVERABLES:
   - Security lessons learned
   - Security process improvements for next project

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
♿ CROSS-CUTTING CONCERN #2: ACCESSIBILITY (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Accessibility by Default — Not a checkbox, baked into design

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 1: REQUIREMENTS                                       │
└─────────────────────────────────────────────────────────────┘
✅ ACCESSIBILITY ACTIVITIES:
   - Accessibility Specialist defines a11y requirements
   - WCAG 2.2 AA minimum compliance requirement
   - Assistive technology support requirements (screen readers, etc.)

✅ DELIVERABLES:
   - Accessibility Requirements Document

┌─────────────────────────────────────────────────────────────┐
│ PHASE 3: UX/UI DESIGN                                       │
└─────────────────────────────────────────────────────────────┘
✅ ACCESSIBILITY ACTIVITIES:
   - Accessibility Specialist in Builder Panel
   - Color contrast ratios verified (4.5:1 minimum)
   - Touch target sizing (44x44px minimum)
   - Keyboard navigation designed
   - Screen reader annotation layer
   - Focus management designed

✅ ACCESSIBILITY CRITIQUE:
   - Accessibility Specialist critic reviews every design decision
   - Flags WCAG violations before development

✅ DELIVERABLES:
   - Accessibility Annotation Document
   - WCAG compliance checklist (design phase)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ ACCESSIBILITY ACTIVITIES:
   - Frontend engineers implement ARIA attributes
   - Semantic HTML used
   - Keyboard navigation implemented
   - Automated a11y linting (axe, eslint-plugin-jsx-a11y)

✅ ACCESSIBILITY CRITIQUE:
   - Accessibility Engineer critic reviews implementation

✅ DELIVERABLES:
   - Accessible code (ARIA, semantic HTML)
   - Automated a11y test results

┌─────────────────────────────────────────────────────────────┐
│ PHASE 6: QA & TESTING                                       │
└─────────────────────────────────────────────────────────────┘
✅ ACCESSIBILITY ACTIVITIES:
   - Accessibility QA Tester in Builder Panel
   - Automated a11y scan (axe-core, Lighthouse)
   - Manual screen reader testing (NVDA, JAWS, VoiceOver)
   - Keyboard-only navigation testing
   - Color blind simulation testing

✅ DELIVERABLES:
   - Accessibility Audit Report (WCAG 2.2 AA compliance verified)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 9: DOCUMENTATION & TRAINING                           │
└─────────────────────────────────────────────────────────────┘
✅ ACCESSIBILITY ACTIVITIES:
   - All videos captioned
   - Documentation WCAG compliant
   - Alternative formats available (PDF, audio)

✅ DELIVERABLES:
   - Accessible documentation (WCAG compliant)
   - Captioned videos

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 CROSS-CUTTING CONCERN #3: OBSERVABILITY (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Observability-First — Instrument before you ship, not after

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 1: REQUIREMENTS                                       │
└─────────────────────────────────────────────────────────────┘
✅ OBSERVABILITY ACTIVITIES:
   - SRE defines observability requirements (NFRs)
   - SLO/SLA targets defined
   - Logging requirements
   - Metrics requirements
   - Tracing requirements
   - Alerting requirements

✅ DELIVERABLES:
   - Observability requirements in NFR document
   - SLO/SLA definitions

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: ARCHITECTURE                                       │
└─────────────────────────────────────────────────────────────┘
✅ OBSERVABILITY ACTIVITIES:
   - Observability Engineer in Builder Panel
   - Observability stack selected (Datadog, Grafana, ELK, etc.)
   - Distributed tracing architecture (OpenTelemetry)
   - Log aggregation architecture
   - Metrics collection architecture
   - Alerting architecture

✅ DELIVERABLES:
   - Observability Architecture Diagram

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ OBSERVABILITY ACTIVITIES:
   - Observability Engineer in Builder Panel
   - Structured logging implemented (JSON logs)
   - Distributed tracing instrumented (OpenTelemetry)
   - Metrics instrumented (Prometheus, StatsD)
   - Health check endpoints implemented (/health, /ready)
   - Request ID propagation

✅ OBSERVABILITY CRITIQUE:
   - SRE critic verifies observability instrumentation
   - Can we debug production issues with this logging?
   - Are all critical paths traced?

✅ DELIVERABLES:
   - Observable code (logging, tracing, metrics)
   - Observability instrumentation verified

┌─────────────────────────────────────────────────────────────┐
│ PHASE 7: DEPLOYMENT                                         │
└─────────────────────────────────────────────────────────────┘
✅ OBSERVABILITY ACTIVITIES:
   - Observability Engineer configures dashboards
   - Alerts configured and tested
   - Runbooks reference dashboards

✅ DELIVERABLES:
   - Production monitoring dashboard
   - Alerting rules configured

┌─────────────────────────────────────────────────────────────┐
│ PHASE 8: POST-LAUNCH                                        │
└─────────────────────────────────────────────────────────────┘
✅ OBSERVABILITY ACTIVITIES:
   - AIOps Engineer implements anomaly detection
   - Observability data analyzed for patterns
   - Dashboards refined based on real traffic

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📝 CROSS-CUTTING CONCERN #4: DOCUMENTATION AS CODE (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Documentation as Code — Docs live in repo, versioned

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ ALL PHASES                                                   │
└─────────────────────────────────────────────────────────────┘
✅ DOCUMENTATION ACTIVITIES:
   - Technical Writer participates in EVERY phase
   - Every deliverable includes documentation artifact
   - Documentation written alongside decisions (not after)
   - Documentation stored in Git (versioned with code)
   - Documentation reviewed in pull requests

✅ EXAMPLES BY PHASE:
   - Phase 0: Project concept document
   - Phase 1: Requirements specification (FRS/SRS)
   - Phase 1.5: OpenAPI specification (API docs)
   - Phase 2: Architecture Decision Records (ADRs)
   - Phase 2.5: Database schema documentation (DDL comments)
   - Phase 4: Code comments, README files, inline docs
   - Phase 5: Security review documentation
   - Phase 8.5: Runbooks
   - Phase 9: End-user documentation
   - Phase 10: Retrospective report

✅ DOCUMENTATION CRITIQUE:
   - Every phase includes "Documentation Completeness" in scoring rubric
   - Technical Writer critic participates in all phases

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚖️ CROSS-CUTTING CONCERN #5: COMPLIANCE AS ARCHITECTURE (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Compliance as Architecture — Regulatory requirements drive design

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 0: IDEATION                                           │
└─────────────────────────────────────────────────────────────┘
✅ COMPLIANCE ACTIVITIES:
   - Legal Counsel identifies applicable regulations
   - GDPR, CCPA, HIPAA, PCI-DSS, SOC 2, ISO 27001, etc.
   - Compliance Officer in Builder Panel

✅ DELIVERABLES:
   - Compliance Requirements Matrix

┌─────────────────────────────────────────────────────────────┐
│ PHASE 1: REQUIREMENTS                                       │
└─────────────────────────────────────────────────────────────┘
✅ COMPLIANCE ACTIVITIES:
   - Compliance Officer translates regulations to requirements
   - GDPR: Privacy by Design requirements
   - GDPR: Right to deletion, data portability, consent
   - HIPAA: Audit logging, encryption, access controls
   - PCI-DSS: Cardholder data handling

✅ DELIVERABLES:
   - Compliance Requirements Traceability Matrix

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: ARCHITECTURE                                       │
└─────────────────────────────────────────────────────────────┘
✅ COMPLIANCE ACTIVITIES:
   - Compliance requirements drive architectural decisions
   - Data residency requirements → multi-region architecture
   - GDPR right to deletion → soft-delete architecture
   - Audit trail requirements → immutable audit log design

✅ COMPLIANCE CRITIQUE:
   - Compliance Officer critic reviews architecture for compliance gaps

┌─────────────────────────────────────────────────────────────┐
│ ALL SUBSEQUENT PHASES                                        │
└─────────────────────────────────────────────────────────────┘
✅ COMPLIANCE ACTIVITIES:
   - Compliance Officer participates in critique panels
   - Every design decision evaluated for compliance impact
   - Compliance checkpoints at phase gates

✅ DELIVERABLES (by Phase 7):
   - Compliance attestation (SOC 2, ISO 27001, etc.)
   - Privacy policy and terms of service (reviewed by legal)
   - GDPR compliance documentation (if applicable)
   - HIPAA compliance documentation (if applicable)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔧 CROSS-CUTTING CONCERN #6: EVERYTHING AS CODE (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Everything as Code — Infra, pipelines, config, policies are version-controlled

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: ARCHITECTURE                                       │
└─────────────────────────────────────────────────────────────┘
✅ EVERYTHING-AS-CODE COMMITMENT:
   - Infrastructure as Code (IaC): Terraform, Pulumi, CloudFormation
   - Configuration as Code: ConfigMaps, environment variables in Git
   - Policy as Code: OPA (Open Policy Agent), AWS IAM policies in code
   - Pipeline as Code: Jenkinsfile, .github/workflows, GitLab CI YAML

✅ DELIVERABLES:
   - IaC framework selected
   - Configuration management strategy

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ EVERYTHING-AS-CODE IMPLEMENTATION:
   - All infrastructure defined in Terraform/Pulumi
   - All CI/CD pipelines in YAML/code
   - All configs in version control (not manual configs)
   - No "snowflake" servers or manual processes

✅ DELIVERABLES:
   - IaC code (all infrastructure)
   - Pipeline code (CI/CD)
   - Configuration files (version-controlled)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 7: DEPLOYMENT                                         │
└─────────────────────────────────────────────────────────────┘
✅ EVERYTHING-AS-CODE VERIFICATION:
   - Deployment entirely automated (no manual steps)
   - Infrastructure provisioned from code
   - Zero manual configuration

✅ DEVOPS CRITIC:
   - Verifies zero manual processes
   - Verifies everything is repeatable and version-controlled

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💰 CROSS-CUTTING CONCERN #7: COST MANAGEMENT (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Cost-conscious architecture and operations

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 0: IDEATION                                           │
└─────────────────────────────────────────────────────────────┘
✅ COST ACTIVITIES:
   - Budget defined
   - Cost constraints identified
   - ROI expectations set

┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: ARCHITECTURE                                       │
└─────────────────────────────────────────────────────────────┘
✅ COST ACTIVITIES:
   - Cloud Architect estimates infrastructure costs
   - FinOps Engineer in Builder Panel
   - Cost-optimized architecture decisions
   - Reserved instances / savings plans considered

✅ COST CRITIQUE:
   - FinOps Engineer critic reviews cost implications
   - Identifies cost optimization opportunities

✅ DELIVERABLES:
   - Cloud Cost Architecture Estimate

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ COST ACTIVITIES:
   - Cost-aware code (efficient queries, caching, etc.)
   - Resource right-sizing

┌─────────────────────────────────────────────────────────────┐
│ PHASE 8: POST-LAUNCH                                        │
└─────────────────────────────────────────────────────────────┘
✅ COST ACTIVITIES:
   - FinOps Engineer monitors actual costs
   - Cost anomaly detection
   - Cost optimization opportunities identified
   - Reserved instance purchases (based on actual usage)

✅ DELIVERABLES:
   - Monthly cost reports
   - Cost optimization backlog

┌─────────────────────────────────────────────────────────────┐
│ PHASE 10: RETROSPECTIVE                                     │
└─────────────────────────────────────────────────────────────┘
✅ COST ACTIVITIES:
   - Budget variance analysis
   - TCO (Total Cost of Ownership) assessment
   - Cost lessons learned

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📈 CROSS-CUTTING CONCERN #8: METRICS & ANALYTICS (Continuous)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRINCIPLE: Measure everything, learn continuously

HOW TO INTEGRATE INTO EACH PHASE:

┌─────────────────────────────────────────────────────────────┐
│ PHASE 0: IDEATION                                           │
└─────────────────────────────────────────────────────────────┘
✅ METRICS ACTIVITIES:
   - Success metrics defined (KPIs)
   - Baseline measurements identified

✅ DELIVERABLES:
   - Success metrics definition (part of Phase 0 output)

┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: DEVELOPMENT                                        │
└─────────────────────────────────────────────────────────────┘
✅ METRICS ACTIVITIES:
   - Analytics Engineer instruments analytics events
   - Product analytics (Mixpanel, Amplitude, Segment)
   - Business metrics tracked
   - User behavior tracked

✅ DELIVERABLES:
   - Analytics instrumentation code
   - Event tracking plan

┌─────────────────────────────────────────────────────────────┐
│ PHASE 7: DEPLOYMENT                                         │
└─────────────────────────────────────────────────────────────┘
✅ METRICS ACTIVITIES:
   - Analytics Engineer verifies events firing correctly
   - Baseline metrics established

✅ DELIVERABLES:
   - Analytics dashboard
   - Baseline metrics report

┌─────────────────────────────────────────────────────────────┐
│ PHASE 8: POST-LAUNCH                                        │
└─────────────────────────────────────────────────────────────┘
✅ METRICS ACTIVITIES:
   - Data Analyst monitors metrics
   - Metrics reviewed weekly/monthly
   - Data-driven product decisions

┌─────────────────────────────────────────────────────────────┐
│ PHASE 10: RETROSPECTIVE                                     │
└─────────────────────────────────────────────────────────────┘
✅ METRICS ACTIVITIES:
   - Success metrics reviewed (goal vs actual)
   - Lessons learned on what to measure

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ INTEGRATION CHECKLIST (For EVERY Phase)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Use this checklist to ensure cross-cutting concerns are integrated:

FOR EVERY PHASE, ASK:

🔐 SECURITY:
   □ Is a security expert in the Builder Panel?
   □ Is a security expert in the Critique Panel?
   □ Are security requirements/implications addressed?
   □ Is there a security deliverable for this phase?

♿ ACCESSIBILITY:
   □ Is an accessibility expert involved (if UI/UX phase)?
   □ Are accessibility requirements/implications addressed?
   □ Is there an accessibility deliverable (if applicable)?

📊 OBSERVABILITY:
   □ Are observability requirements/implications addressed?
   □ Is instrumentation being added (if development phase)?
   □ Can we monitor/debug this in production?

📝 DOCUMENTATION:
   □ Is a Technical Writer involved?
   □ Is documentation being created alongside work (not after)?
   □ Is documentation version-controlled?

⚖️ COMPLIANCE:
   □ Is a Compliance Officer involved (if applicable)?
   □ Are regulatory requirements being met?
   □ Is there a compliance deliverable (if applicable)?

🔧 EVERYTHING AS CODE:
   □ Is everything (infra, config, policy) in code?
   □ Are there no manual processes?
   □ Is everything repeatable and version-controlled?

💰 COST MANAGEMENT:
   □ Are cost implications considered?
   □ Is a FinOps expert involved (if architecture/infra phase)?
   □ Are we building cost-efficiently?

📈 METRICS:
   □ Are we measuring the right things?
   □ Is analytics instrumented (if development phase)?
   □ Do we have baseline metrics (if post-launch)?

IF ANY CHECKBOX IS UNCHECKED, THE PHASE IS INCOMPLETE.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎯 SUMMARY: CROSS-CUTTING CONCERNS ARE NOT OPTIONAL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

These 8 cross-cutting concerns MUST be integrated into every phase.

They are NOT "nice to have" or "we'll add it later."

They are ARCHITECTURAL INVARIANTS that cannot be retrofitted.

❌ FAILURE MODE: "We'll add security after we ship" → Breached
❌ FAILURE MODE: "We'll add accessibility later" → Lawsuit
❌ FAILURE MODE: "We'll add monitoring after we see issues" → Blind in production
❌ FAILURE MODE: "We'll document it eventually" → Tribal knowledge, bus factor
❌ FAILURE MODE: "We'll worry about compliance later" → Regulatory fine
❌ FAILURE MODE: "We'll automate it later" → Snowflake servers, manual toil
❌ FAILURE MODE: "We'll optimize costs later" → Bill shock, budget overrun
❌ FAILURE MODE: "We'll add analytics later" → Flying blind, no data-driven decisions

✅ SUCCESS MODE: Integrate ALL cross-cutting concerns from DAY ONE.

This is the difference between world-class software and everything else.

[END OF CROSS-CUTTING CONCERNS INTEGRATION GUIDE]
```

---

## 🎯 **FINAL SUMMARY: COMPLETE SDLC COVERAGE**

You now have **COMPLETE, EXHAUSTIVE, PRODUCTION-READY** prompt templates for:

### ✅ **ALL 16 SDLC PHASES:**
1. Phase 0 — Ideation & Inception ✅
2. Phase 1 — Requirements & Specification ✅
3. **Phase 1.5 — API/Interface Design** ✅ *(NEW)*
4. Phase 2 — Architecture & System Design ✅
5. **Phase 2.5 — Database Design & Data Modeling** ✅ *(NEW)*
6. Phase 3 — UX/UI Design ✅
7. Phase 4 — Development ✅
8. Phase 5 — Security Review ✅
9. **Phase 5.5 — Integration & E2E Testing** ✅ *(NEW)*
10. Phase 6 — QA & Testing ✅
11. **Phase 6.5 — Data Migration & Seeding** ✅ *(NEW)*
12. Phase 7 — Deployment & Release ✅
13. **Phase 7.5 — Beta/Preview Program** ✅ *(NEW)*
14. Phase 8 — Post-Launch Monitoring ✅
15. **Phase 8.5 — Incident Response & Runbooks** ✅ *(NEW)*
16. **Phase 9 — Training, Documentation & Change Management** ✅ *(NEW)*
17. **Phase 10 — Retrospective & Continuous Learning** ✅ *(NEW)*

### ✅ **8 CROSS-CUTTING CONCERNS:**
1. Security (Shift-Left)
2. Accessibility (By Default)
3. Observability (First)
4. Documentation (As Code)
5. Compliance (As Architecture)
6. Everything as Code
7. Cost Management
8. Metrics & Analytics

### ✅ **MULTI-AGENT ADVERSARIAL FRAMEWORK:**
- Builder Panel (expert builders per phase)
- Critique Panel (adversarial critics per phase)
- Iterative remediation loops
- Unanimous 9/10+ scoring gate
- No phase progression until all critics satisfied

---

**This is the most comprehensive SDLC prompt system ever created.** 

Use it to build world-class software. 🚀
