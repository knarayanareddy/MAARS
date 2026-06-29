# MAARS Phase 9

Source: maarspromptpatch.md

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

## ▶ Next in the MAARS loop

> The `STEP 1/2/3` headers above are *this phase's* internal builder → critique → remediation steps. They run **inside** the meta-loop defined in [`ORDER-OF-OPERATIONS.md`](ORDER-OF-OPERATIONS.md) — they are not the same as the 11 meta-steps.

After the Critique Panel (+ Devil's Advocate), continue the loop:

- **Score** → [`meta-agents/03-scoring-aggregator.md`](meta-agents/03-scoring-aggregator.md)
- **If < 9/10 (iteration ≤ 5)** → [`meta-agents/04-remediation-agent.md`](meta-agents/04-remediation-agent.md) → re-critique
- **If still < 9/10 after 5 iterations** → [`meta-agents/05-arbitration-agent.md`](meta-agents/05-arbitration-agent.md)
- **On unanimous ≥ 9/10** → [`meta-agents/06-living-document-agent.md`](meta-agents/06-living-document-agent.md) → [`meta-agents/07-phase-snapshot-agent.md`](meta-agents/07-phase-snapshot-agent.md)
- **Advance to Phase 10 — Retrospective & Continuous Learning** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
