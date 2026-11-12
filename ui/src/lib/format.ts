import type { RawFinding, Severity, UiFinding, VerificationStatus } from './types'

export function bytes(n: number): string {
  if (n < 1024) return `${n} B`
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`
  return `${(n / 1024 / 1024).toFixed(1)} MB`
}

export function ms(n: number): string {
  return n < 1000 ? `${n} ms` : `${(n / 1000).toFixed(2)} s`
}

export function clock(iso: string): string {
  const d = new Date(iso)
  if (Number.isNaN(d.getTime())) return ''
  return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false })
}

export function relative(iso: string): string {
  const d = new Date(iso).getTime()
  if (Number.isNaN(d)) return ''
  const s = Math.round((Date.now() - d) / 1000)
  if (s < 60) return 'just now'
  if (s < 3600) return `${Math.floor(s / 60)} min ago`
  if (s < 86400) return `${Math.floor(s / 3600)} h ago`
  return `${Math.floor(s / 86400)} d ago`
}

export function asSeverity(s: string): Severity {
  const v = s.toLowerCase()
  if (v.startsWith('crit')) return 'Critical'
  if (v.startsWith('high')) return 'High'
  if (v.startsWith('med')) return 'Medium'
  if (v.startsWith('low')) return 'Low'
  return 'Informational'
}

/**
 * Map engine output onto the five verification statuses.
 * Scanner results carry `Confirmed` / `Potential` directly; stored findings carry a
 * confidence (`Confirmed` / `High` / `Medium` / `Low`) and a workflow status.
 */
export function asStatus(status: string, confidence?: string): VerificationStatus {
  const s = status.toLowerCase()
  if (s.includes('manual')) return 'Needs Manual Verification'
  if (s.startsWith('info')) return 'Informational'
  if (s === 'confirmed' || s === 'verified') return 'Confirmed'
  if (s === 'detected') return 'Detected'
  if (s === 'potential') return 'Potential'
  const c = (confidence ?? '').toLowerCase()
  if (c === 'confirmed') return 'Confirmed'
  if (c === 'high') return 'Detected'
  if (c === 'low') return 'Needs Manual Verification'
  return 'Potential'
}

/** Hostname of a target URL (no port), or '' when it cannot be parsed. */
export function hostnameOf(target: string): string {
  try { return new URL(/^https?:\/\//i.test(target) ? target : `https://${target}`).hostname.toLowerCase() } catch { return '' }
}

/**
 * Scope match on a domain boundary: the scope host itself or a real subdomain of it.
 * A plain suffix test would let xportal.example.test pass for portal.example.test.
 */
export function inScope(host: string, scopeHost: string): boolean {
  const h = host.toLowerCase().replace(/:\d+$/, '')
  const s = scopeHost.toLowerCase()
  return !!s && (h === s || h.endsWith('.' + s))
}

/** Only Confirmed counts as proven. Everything else still needs a person or more evidence. */
export const isProven = (f: UiFinding) => f.status === 'Confirmed' && f.decision !== 'false-positive'
/** Waiting on the tester: not proven, not context-only, not already decided. */
export const needsDecision = (f: UiFinding) =>
  !f.decision && f.status !== 'Confirmed' && f.status !== 'Informational'

export function fromRaw(f: RawFinding): UiFinding {
  const refs = Array.isArray(f.references)
    ? f.references
    : (f.references ?? '').split('\n').map(x => x.trim()).filter(Boolean)
  return {
    id: f.id,
    title: f.title,
    severity: asSeverity(f.severity),
    // False Positive and Accepted Risk are tester decisions; verification then comes from confidence.
    status: f.status === 'False Positive' || f.status === 'Accepted Risk' ? asStatus('', f.confidence) : asStatus(f.status, f.confidence),
    decision: f.status === 'False Positive' ? 'false-positive' : f.status === 'Accepted Risk' ? 'accepted-risk' : undefined,
    confidence: f.confidence,
    description: f.description,
    impact: f.impact ?? '',
    recommendation: f.recommendation ?? '',
    references: refs,
    created_at: f.created_at,
    ...(f.record ? {
      gate: f.record.gate,
      baseline: f.record.baseline ?? undefined,
      observed: f.record.observed,
      delta: f.record.delta && f.record.observed.includes(f.record.delta) ? f.record.delta : undefined,
      reproduction: f.record.reproduction,
      endpoint: f.record.endpoint,
      parameter: f.record.parameter || undefined,
      rule: f.record.technique || undefined,
    } : {}),
  }
}

export function statusClass(code: number): string {
  if (code >= 500) return 'code-bad'
  if (code >= 400) return 'code-warn'
  if (code >= 300) return 'code-redirect'
  if (code >= 200) return 'code-ok'
  return 'code-none'
}

export function slug(s: string): string {
  return s.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '')
}
