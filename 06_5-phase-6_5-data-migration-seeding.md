# MAARS Phase 6.5

Source: maarspromptpatch.md

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
