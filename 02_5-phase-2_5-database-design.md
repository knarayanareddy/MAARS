# MAARS Phase 2.5

Source: maarspromptpatch.md

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

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 3 — UX / UI Design** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
