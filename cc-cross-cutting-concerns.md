# Cross-Cutting Concerns Integration Guide

Source: maarspromptpatch.md

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