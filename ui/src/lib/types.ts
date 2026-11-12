export interface Assessment {
  id: string
  name: string
  target: string
  environment: string
  scope: string
  methodology: string
  status: string
  created_at: string
  updated_at: string
}

/** Raw finding row as the Rust `Finding` model serializes it. */
export interface RawFinding {
  id: string
  assessment_id: string
  title: string
  description: string
  severity: string
  confidence: string
  status: string
  impact?: string
  recommendation?: string
  references?: string[] | string
  created_at?: string
  /** Verification record captured by the scanner (see src/scanner/sdk/record.rs). */
  record?: RawRecord | null
}

export interface RawRecord {
  technique: string
  parameter: string
  payload: string
  endpoint: string
  baseline: string | null
  observed: string
  delta: string
  reproduction: string
  corroborated_by?: string[]
  gate: { question: string; answer: string; state: 'passed' | 'failed' | 'open' }[]
}

export interface HistoryEntry {
  id: string
  timestamp: string
  method: string
  scheme: string
  host: string
  port: number
  path: string
  query: string
  url: string
  protocol: string
  status_code: number
  request_size: number
  response_size: number
  duration_ms: number
  tls_enabled: boolean
  request_headers: string
  response_headers: string
}

export interface RepeaterResponse {
  status_code: number
  status_text: string
  protocol: string
  url: string
  headers_text: string
  body: string
  duration_ms: number
}

export interface ProxyStatus {
  listening: boolean
  port: number
  tor_enabled: boolean
  tor_addr: string
}

export interface ScanFinding {
  rule_id: string
  title: string
  severity: string
  confidence: string
  status: string
  description: string
  recommendation: string
  evidence: string
}

export interface ScanProgress {
  rule_id: string
  rule_name: string
  stage: string
  done: boolean
}

/** The five verification statuses. Never merged, renamed or abbreviated. */
export type VerificationStatus =
  | 'Confirmed'
  | 'Detected'
  | 'Potential'
  | 'Needs Manual Verification'
  | 'Informational'

export const STATUS_ORDER: VerificationStatus[] = [
  'Confirmed',
  'Detected',
  'Needs Manual Verification',
  'Potential',
  'Informational',
]

export type Severity = 'Critical' | 'High' | 'Medium' | 'Low' | 'Informational'
export const SEVERITY_ORDER: Severity[] = ['Critical', 'High', 'Medium', 'Low', 'Informational']

export type GateState = 'passed' | 'failed' | 'open'

export interface GateStep {
  question: string
  answer: string
  state: GateState
}

/** The tester's decision, stored in the core's finding status field. Separate from verification. */
export type Decision = 'false-positive' | 'accepted-risk'
/** Labels the core's FindingStatus accepts. */
export type CoreStatus = 'Draft' | 'Verified' | 'False Positive' | 'Accepted Risk'

/** Finding as the UI reasons about it. */
export interface UiFinding {
  id: string
  decision?: Decision
  title: string
  severity: Severity
  status: VerificationStatus
  confidence: string
  description: string
  impact: string
  recommendation: string
  references: string[]
  endpoint?: string
  parameter?: string
  rule?: string
  created_at?: string
  gate?: GateStep[]
  baseline?: string
  observed?: string
  /** Substring of `observed` that is the delta to highlight. */
  delta?: string
  reproduction?: string
}

export type View =
  | 'overview'
  | 'proxy'
  | 'repeater'
  | 'scanner'
  | 'decoder'
  | 'findings'
  | 'reports'
  | 'settings'
