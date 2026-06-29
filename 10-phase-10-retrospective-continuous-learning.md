# MAARS Phase 10

Source: maarspromptpatch.md

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
