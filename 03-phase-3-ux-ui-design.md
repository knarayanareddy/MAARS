# MAARS Phase 3

Source: maarsprompt.md

---

## 📋 PHASE 3 — UX/UI DESIGN PROMPT

```
╔══════════════════════════════════════════════════════════════╗
║  PHASE 3: UX/UI DESIGN                                       ║
║  Goal: Design a world-class, accessible, user-centered UI   ║
╚══════════════════════════════════════════════════════════════╝

[APPLY MASTER SYSTEM PROMPT PRINCIPLES]
[INPUT: All Prior Phase Deliverables]

━━━━━━━━━━━━━━━━━━━━━━
STEP 1 — BUILDER PANEL
━━━━━━━━━━━━━━━━━━━━━━
🔬 UX RESEARCHER
   → User personas (detailed: goals, frustrations, 
     tech proficiency, accessibility needs)
   → User journey maps per persona
   → Jobs-to-be-done (JTBD) framework applied
   → Usability heuristics baseline

🗺️ UX DESIGNER
   → Information architecture (IA) — site map / screen map
   → User flows for every critical path
   → Wireframes (lo-fi) for all screens/views
   → Interaction models and navigation patterns
   → Error states, empty states, loading states designed

🎨 UI DESIGNER
   → Design system / component library specification
   → Typography scale and system
   → Color system (with accessible contrast ratios)
   → Spacing and grid system
   → Iconography system
   → Hi-fi mockups for all screens
   → Dark mode / light mode designs

♿ ACCESSIBILITY SPECIALIST
   → WCAG 2.2 AA audit of every design decision
   → Focus management and keyboard navigation design
   → Screen reader annotation layer
   → Color blindness simulation review
   → Touch target sizing (minimum 44x44px)
   → Cognitive load assessment

📱 MOBILE-SPECIFIC (iOS + Android)
   → iOS Human Interface Guidelines compliance
   → Android Material Design 3 compliance
   → Platform-specific interaction patterns
   → Gesture design and conflicts audit
   → Responsive breakpoint designs

✍️ CONTENT STRATEGIST
   → UX copy for every UI element
   → Error message writing (human, helpful, not cryptic)
   → Microcopy for onboarding, empty states, CTAs
   → Terminology consistency glossary

━━━━━━━━━━━━━━━━━━━━━━━━━━
STEP 2 — CRITIQUE PANEL
━━━━━━━━━━━━━━━━━━━━━━━━━━
🔴 ACCESSIBILITY QA TESTER
   → WCAG failure points (specific guideline violations)
   → Screen reader traversal issues
   → Color contrast failures (specific ratios)
   → Missing ARIA labels, roles, landmarks

🔴 SENIOR IOS DEVELOPER
   → Design elements that are impossible/expensive to build in iOS
   → HIG violations
   → Performance implications of design choices

🔴 SENIOR ANDROID DEVELOPER
   → Material Design violations
   → Android-specific impossibilities/complications
   → Fragmentation risks (screen sizes, OS versions)

🔴 SECURITY REVIEWER (UX Security)
   → Dark patterns? (flagged for ethical and legal risk)
   → Information disclosure in UI (data exposure risks)
   → Session timeout UX? Password/auth UX security?

🔴 PERFORMANCE ENGINEER
   → Asset weight assessment
   → Animation performance implications
   → Image loading strategy gaps

🔴 REAL USER PROXY (UX Researcher in Critic Mode)
   → Cognitive overload points?
   → Where will users get lost?
   → What will first-time users misunderstand?

SCORING → REMEDIATION → RE-SCORE [Until all ≥ 9/10]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ PHASE 3 DELIVERABLES (Only at ≥ 9/10)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
→ User Persona Documents
→ User Journey Maps
→ Information Architecture Map
→ User Flow Diagrams (all critical paths)
→ Lo-fi Wireframes (all screens)
→ Hi-fi Mockups (all screens, all states)
→ Design System Specification
→ Accessibility Annotation Document
→ UX Copy & Microcopy Library
→ Platform-Specific Design Specs (iOS/Android)
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
- **Advance to Phase 4 — Development** → fire [`meta-agents/01-continuity-agent.md`](meta-agents/01-continuity-agent.md) with the snapshot capsule as input
