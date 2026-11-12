/* One place that talks to the Rust core. Outside Tauri (browser preview) every call
   falls back to demo data so the UI can be reviewed without the desktop shell. */
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { DEMO_ASSESSMENTS, DEMO_FINDINGS, DEMO_TRAFFIC, demoEntry } from './demo'
import { fromRaw } from './format'
import type {
  Assessment,
  CoreStatus,
  HistoryEntry,
  ProxyStatus,
  RawFinding,
  RepeaterResponse,
  ScanFinding,
  ScanProgress,
  UiFinding,
} from './types'

export const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

/* Demo-only switches for reviewing states in a browser: ?demo=empty | error | stopped.
   They never apply inside the desktop shell. */
const demoMode = !isTauri && typeof location !== 'undefined' ? new URLSearchParams(location.search).get('demo') : null
const demoFail = (what: string) => { throw new Error(`database is locked (os error 33) while reading ${what}`) }

let demoAssessments = [...DEMO_ASSESSMENTS]
let demoProxy: ProxyStatus = { listening: true, port: 8080, tor_enabled: false, tor_addr: '127.0.0.1:9050' }
const wait = (t: number) => new Promise(r => setTimeout(r, t))

export async function listAssessments(): Promise<Assessment[]> {
  if (isTauri) return invoke<Assessment[]>('list_assessments')
  if (demoMode === 'error') demoFail('assessments')
  if (demoMode === 'empty') return []
  return demoAssessments
}

export async function createAssessment(a: {
  name: string
  target: string
  environment: string
  scope: string
  methodology: string
}): Promise<Assessment> {
  if (isTauri) return invoke<Assessment>('create_assessment', a)
  const now = new Date().toISOString()
  const created: Assessment = { ...a, id: `demo-${Date.now()}`, status: 'Draft', created_at: now, updated_at: now }
  demoAssessments = [created, ...demoAssessments]
  return created
}

export async function deleteAssessment(id: string): Promise<void> {
  if (isTauri) return invoke('delete_assessment', { assessmentId: id })
  demoAssessments = demoAssessments.filter(a => a.id !== id)
}

export async function listFindings(assessmentId: string): Promise<UiFinding[]> {
  if (isTauri) {
    const raw = await invoke<RawFinding[]>('list_findings', { assessmentId })
    return raw.map(fromRaw)
  }
  // Demo: apply the decisions made in this browser session on top of the fixed sample.
  return (DEMO_FINDINGS[assessmentId] ?? []).map(f => {
    const d = demoDecisions.get(f.id)
    if (!d || d === 'Draft') return f
    if (d === 'Verified') return { ...f, status: 'Confirmed' as const }
    return { ...f, decision: d === 'False Positive' ? 'false-positive' as const : 'accepted-risk' as const }
  })
}

const demoDecisions = new Map<string, CoreStatus>()

/** Record the tester's decision. Verified confirms it; Draft reopens it. */
export async function updateFindingStatus(assessmentId: string, findingId: string, status: CoreStatus): Promise<void> {
  if (isTauri) {
    await invoke('update_finding_status', { assessmentId, findingId, status })
    return
  }
  await wait(250)
  if (demoMode === 'error') demoFail('finding')
  demoDecisions.set(findingId, status)
}

export async function listTraffic(limit = 200): Promise<HistoryEntry[]> {
  if (isTauri) return invoke<HistoryEntry[]>('list_traffic', { limit })
  if (demoMode === 'error') demoFail('history')
  if (demoMode === 'empty') return []
  return DEMO_TRAFFIC.slice(0, limit)
}

export async function proxyStatus(): Promise<ProxyStatus> {
  if (isTauri) return invoke<ProxyStatus>('proxy_status')
  if (demoMode === 'stopped') return { ...demoProxy, listening: false }
  return demoProxy
}

export async function setProxyMode(torEnabled: boolean, torAddr: string): Promise<ProxyStatus> {
  if (isTauri) return invoke<ProxyStatus>('set_proxy_mode', { torEnabled, torAddr })
  await wait(400)
  demoProxy = { ...demoProxy, tor_enabled: torEnabled, tor_addr: torAddr }
  return demoProxy
}

export async function repeaterSend(rawRequest: string): Promise<RepeaterResponse> {
  if (isTauri) return invoke<RepeaterResponse>('repeater_send', { rawRequest })
  await wait(380)
  const first = rawRequest.split('\n')[0] ?? ''
  const host = /^host:\s*(.+)$/im.exec(rawRequest)?.[1]?.trim() ?? 'portal.halden-freight.test'
  const path = first.split(' ')[1] ?? '/'
  const injected = /'|%27/.test(path)
  return {
    status_code: injected ? 500 : 200,
    status_text: injected ? 'Internal Server Error' : 'OK',
    protocol: 'HTTP/1.1',
    url: `https://${host}${path}`,
    headers_text: `Server: nginx/1.24.0\r\nContent-Type: application/json\r\nCache-Control: no-store`,
    body: injected
      ? `{"error":"ERROR: unterminated quoted string at or near \\"'10442''\\""}`
      : `{"order":{"id":"10442","status":"in_transit","eta":"2026-11-14"}}`,
    duration_ms: injected ? 63 : 71,
  }
}

/** Live proxy captures. Returns an unsubscribe function. */
export function onTraffic(cb: (e: HistoryEntry) => void): () => void {
  if (isTauri) {
    const un = listen<HistoryEntry>('traffic-captured', ev => cb(ev.payload))
    return () => { un.then(f => f()) }
  }
  if (demoMode) return () => {}
  let i = 0
  const t = setInterval(() => {
    if (i >= 8) return clearInterval(t)
    cb(demoEntry(i * 3 + 4, 0))
    i++
  }, 5200)
  return () => clearInterval(t)
}

export interface ScanHandlers {
  onProgress: (p: ScanProgress) => void
  onFinding: (f: ScanFinding) => void
}

let demoCancel = false

/** Ask the running scan to stop. Results found so far are kept. */
export async function cancelScan(): Promise<void> {
  if (isTauri) { await invoke('cancel_scan'); return }
  demoCancel = true
}

export async function startScan(assessmentId: string, target: string, h: ScanHandlers): Promise<ScanFinding[]> {
  if (isTauri) {
    const un1 = listen<ScanProgress>('scan-progress', e => h.onProgress(e.payload))
    const un2 = listen<ScanFinding>('scan-finding', e => h.onFinding(e.payload))
    try {
      return await invoke<ScanFinding[]>('start_scan', { assessmentId, target })
    } finally {
      un1.then(f => f())
      un2.then(f => f())
    }
  }
  const rules: [string, string, ScanFinding[]][] = [
    ['sqli', 'SQL injection', [{
      rule_id: 'sqli', title: 'SQL injection in order lookup', severity: 'Critical', confidence: 'Confirmed',
      status: 'Confirmed', description: 'New database error versus baseline, confirmed with a boolean pair.',
      recommendation: 'Use parameterized queries.', evidence: "id=10442' -> 500, syntax error",
    }]],
    ['xss', 'Cross-site scripting', [{
      rule_id: 'xss', title: 'Reflected cross-site scripting in search', severity: 'High', confidence: 'High',
      status: 'Potential', description: 'Verbatim unescaped reflection in an HTML text context.',
      recommendation: 'Encode output.', evidence: 'q reflected at offset 1184',
    }]],
    ['path-traversal', 'Path traversal', []],
  ]
  const out: ScanFinding[] = []
  demoCancel = false
  for (const [id, name, found] of rules) {
    for (const stage of ['baseline', 'mutate', 'observe', 'compare', 'verify']) {
      if (demoCancel) throw new Error('Scan cancelled. Results found before cancelling are kept.')
      h.onProgress({ rule_id: id, rule_name: name, stage, done: false })
      await wait(520)
    }
    for (const f of found) { h.onFinding(f); out.push(f) }
    h.onProgress({ rule_id: id, rule_name: name, stage: 'complete', done: true })
  }
  return out
}
