// Single source of truth for cross-boundary contracts (D1 §6).
// Mirrors the Rust types in src-tauri/src/contracts.rs — keep them in sync.

export type StepId = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11;

export type Severity = "CRITICAL" | "MAJOR" | "MINOR" | "ADVISORY";
export type GateDecision = "PASS" | "FAIL" | "ARBITRATION";
export type LoopMode = "full-auto" | "semi-auto" | "supervised";

export interface Finding {
  id: string;
  severity: Severity;
  what: string;
  why: string;
  how_to_fix: string;
  status: "OPEN" | "RESOLVED" | "ACCEPTED_RISK";
}

export interface CriticScore {
  critic_id: string;
  score: number;
  valid: boolean;
  invalidation_reason: string | null;
  key_condition: string;
  findings: Finding[];
}

// The gate's only input (R7).
export interface ScoreReport {
  phase: number;
  iteration: number;
  phase_score: number;
  decision: GateDecision;
  bottleneck_critic: string | null;
  next_action: string;
  critic_scores: CriticScore[];
}

export type StructuralMarker =
  | { kind: "builderPersona"; name: string }
  | { kind: "critiqueSection"; critic: string }
  | { kind: "scoreCardStart" }
  | { kind: "remediationTicket"; id: string }
  | { kind: "phaseExit" };

// The single backend -> frontend seam (ADR-005).
export type AgentEvent =
  | { type: "stepStart"; runId: string; phaseId: string; stepId: StepId; iteration: number }
  | { type: "token"; runId: string; stepId: StepId; chunk: string; seq: number }
  | { type: "marker"; runId: string; stepId: StepId; marker: StructuralMarker }
  | { type: "stepComplete"; runId: string; stepId: StepId; artifactRef: string }
  | { type: "score"; runId: string; phaseId: string; report: ScoreReport }
  | { type: "fileWritten"; runId: string; path: string; phaseId: string }
  | { type: "phasePassed"; runId: string; phaseId: string; snapshotRef: string }
  | { type: "needsHuman"; runId: string; reason: string; payload: unknown }
  | { type: "error"; runId: string; stepId: StepId | null; error: string; recoverable: boolean };

export type Platform = "web" | "mobile" | "api" | "desktop" | "embedded";
export type Scale = "prototype" | "startup" | "enterprise";
export type Compliance = "none" | "GDPR" | "HIPAA" | "SOC2" | "PCI";
export type Preset = "Lite" | "Standard" | "Full";

export interface Phase0Seed {
  idea: string;
  context: {
    platform: Platform[];
    scale: Scale;
    compliance: Compliance[];
    constraints?: string;
  };
  preset: Preset;
  // Phase A express path: a single model for every step. Keys never travel here.
  model: { providerId: string; model: string };
}

export interface ProjectMeta {
  id: string;
  name: string;
  preset: Preset;
  currentPhase: string;
  lastActive: string;
}

export type RouteRole = "continuity" | "research" | "builder" | "critique" | "scoring" | "arbitration" | "snapshot";
export type ContextBand = "Healthy" | "Amber" | "Red" | "Critical";

export interface ModelCapabilities {
  jsonMode: boolean;
  functionCalling: boolean;
  streaming: boolean;
  estContextWindow: number;
}

export interface ProviderRef {
  id: string;
  kind: string;
  label: string;
  baseUrl: string;
  model: string;
  capabilities: ModelCapabilities;
}

export interface ContextBudget {
  usedTokens: number;
  limitTokens: number;
  ratio: number;
  band: ContextBand;
  recommendation: string;
}

export interface RoutePlan {
  role: RouteRole;
  providerId: string;
  providerKind: string;
  model: string;
  reason: string;
  requirements: string[];
  capabilities: ModelCapabilities;
  budget: ContextBudget;
}

export interface RoutingOverview {
  preset: Preset;
  routes: RoutePlan[];
}

export type AuditVerdict = "Pass" | "Warn";

export interface AuditFinding {
  label: string;
  verdict: AuditVerdict;
  detail: string;
}

export interface AuditSection {
  name: string;
  findings: AuditFinding[];
}

export interface DashboardAuditReport {
  overall: AuditVerdict;
  security: AuditSection;
  performance: AuditSection;
  accessibility: AuditSection;
}
