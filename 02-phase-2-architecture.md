# MAARS Phase 2

Source: maarsprompt.md

---

## 📋 PHASE 2 — ARCHITECTURE & SYSTEM DESIGN PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 2: ARCHITECTURE & SYSTEM DESIGN                       ║
║  Goal: Produce the complete, battle-hardened system design   ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]
[INPUT: Phase 0 + Phase 1 Deliverables]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
🏗️ SOLUTION ARCHITECT (Lead)
   → System architecture diagram (described in full detail)
   → Component breakdown and responsibilities
   → Technology stack selection with justification
   → Integration patterns and middleware decisions
   → Architectural patterns chosen (microservices/monolith/
     modular monolith/event-driven/CQRS/Event Sourcing)
   → ADRs (Architecture Decision Records) for every major decision

☁️ CLOUD ARCHITECT
   → Cloud provider selection with justification
   → Multi-region / multi-AZ strategy
   → Cloud-native services selected
   → Cost architecture (estimated)
   → Vendor lock-in mitigation strategy

🔒 SECURITY ARCHITECT
   → Zero-trust architecture design
   → Network segmentation and security zones
   → Threat model (STRIDE methodology applied)
   → Secret management architecture (Vault, AWS Secrets Manager)
   → Identity and access management architecture
   → Encryption architecture (KMS, TLS 1.3, cipher suites)
   → WAF, DDoS protection, API gateway security

📊 DATA ARCHITECT
   → Database selection per service (polyglot persistence)
   → Data flow diagrams (DFDs Level 0, 1, 2)
   → Entity Relationship Diagrams (ERDs)
   → Data partitioning and sharding strategy
   → Caching architecture (Redis, CDN, in-memory)
   → Search architecture (Elasticsearch/Algolia)
   → Data warehouse / analytics architecture

📱 MOBILE ARCHITECT
   → Mobile architecture pattern (MVVM/MVI/Clean Architecture)
   → Offline-first strategy and sync mechanism
   → Native vs. cross-platform decision with justification
   → Push notification architecture
   → Mobile API strategy (BFF — Backend for Frontend pattern)

🔁 DEVOPS / SRE ARCHITECT
   → CI/CD pipeline architecture
   → Environment strategy (dev/staging/prod/DR)
   → Infrastructure as Code framework (Terraform/Pulumi)
   → Container orchestration design (K8s architecture)
   → Observability stack (logging/metrics/tracing/alerting)
   → Disaster recovery architecture (RTO/RPO plans)
   → Blue-green / canary deployment strategy

🤖 AI/ML ARCHITECT (if applicable)
   → ML pipeline architecture
   → Model serving infrastructure
   → Feature store design
   → LLM/RAG architecture (if applicable)
   → Data feedback loops

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 SENIOR PENETRATION TESTER
   → Attack surface analysis of the proposed architecture
   → Where are the weakest entry points?
   → What does lateral movement look like in this architecture?
   → What blast radius does a breach have?

🔴 SRE / CHAOS ENGINEERING EXPERT
   → What are the single points of failure?
   → What happens when [Service X] dies?
   → Are the SLOs achievable with this design?
   → Failure mode analysis (FMEA) on critical paths

🔴 SENIOR BACKEND ENGINEER
   → Scalability bottlenecks?
   → Race conditions and distributed systems pitfalls?
   → Database design anti-patterns?
   → N+1 query risks, connection pool exhaustion risks?

🔴 CLOUD ARCHITECT CRITIC
   → Cloud cost risks (bill shock scenarios)?
   → Vendor lock-in severity assessment?
   → Compliance gaps in the cloud design?

🔴 ENTERPRISE ARCHITECT CRITIC
   → Does this integrate with realistic enterprise environments?
   → Future-proofing assessment?
   → Technology obsolescence risks?

🔴 FINOPS ENGINEER
   → Where will cost overruns occur?
   → What will this realistically cost at 10x/100x scale?
   → What cost optimizations are missing?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 2 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ System Architecture Document (SAD)
→ Architecture Decision Records (ADRs)
→ Threat Model Document (STRIDE)
→ Data Flow Diagrams (L0/L1/L2)
→ Entity Relationship Diagrams (ERDs)
→ Infrastructure Architecture Diagram
→ CI/CD Pipeline Design
→ Observability Architecture Plan
→ Technology Stack Decision Matrix
→ Cloud Cost Architecture Estimate
→ DR/BCP Architecture Plan
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
- **Advance to Phase 2.5 — Database Design & Data Modeling** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
