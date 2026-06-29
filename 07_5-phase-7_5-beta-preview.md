# MAARS Phase 7.5

Source: maarspromptpatch.md

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

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 8 — Post-Launch Monitoring** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
