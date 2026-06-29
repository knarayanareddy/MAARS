# MAARS Phase 8.5

Source: maarspromptpatch.md

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

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 9 — Training, Documentation & Change Management** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
