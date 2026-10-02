import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

interface Assessment {
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

interface Finding {
  id: string
  assessment_id: string
  title: string
  description: string
  severity: string
  confidence: string
  status: string
}

interface HistoryEntry {
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

interface RepeaterResponse {
  status_code: number
  status_text: string
  protocol: string
  url: string
  headers_text: string
  body: string
  duration_ms: number
}

interface ProxyStatus {
  listening: boolean
  port: number
  tor_enabled: boolean
  tor_addr: string
}

type View = 'home' | 'traffic' | 'findings' | 'repeater' | 'scanner' | 'reports' | 'settings'

const NAV: { id: View; label: string; icon: string }[] = [
  { id: 'home', label: 'Home', icon: '⌂' },
  { id: 'traffic', label: 'Proxy / Traffic', icon: '⇄' },
  { id: 'findings', label: 'Findings', icon: '▲' },
  { id: 'repeater', label: 'Repeater', icon: '↻' },
  { id: 'scanner', label: 'Scanner', icon: '◎' },
  { id: 'reports', label: 'Reports', icon: '▤' },
  { id: 'settings', label: 'Settings', icon: '⚙' },
]

const SEV_CLASS: Record<string, string> = {
  Critical: 'sev-crit',
  High: 'sev-high',
  Medium: 'sev-med',
  Low: 'sev-low',
  Informational: 'sev-info',
}

function App() {
  const [view, setView] = useState<View>('home')
  const [assessments, setAssessments] = useState<Assessment[]>([])
  const [findings, setFindings] = useState<Finding[]>([])
  const [selected, setSelected] = useState<Assessment | null>(null)
  const [showNew, setShowNew] = useState(false)
  const [error, setError] = useState('')
  const [traffic, setTraffic] = useState<HistoryEntry[]>([])
  const [selectedEntry, setSelectedEntry] = useState<HistoryEntry | null>(null)
  const [proxyStatus, setProxyStatus] = useState<ProxyStatus | null>(null)
  const [torForm, setTorForm] = useState({ tor_enabled: false, tor_addr: '127.0.0.1:9050' })
  const [repeaterReq, setRepeaterReq] = useState('GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n')
  const [repeaterResp, setRepeaterResp] = useState<RepeaterResponse | null>(null)
  const [sending, setSending] = useState(false)
  const [form, setForm] = useState({
    name: '',
    target: '',
    environment: 'Production',
    scope: 'Web Application',
    methodology: 'OWASP',
  })

  async function load() {
    try {
      setAssessments(await invoke<Assessment[]>('list_assessments'))
    } catch (e) {
      setError(String(e))
    }
  }

  async function loadFindings(a: Assessment) {
    try {
      setFindings(await invoke<Finding[]>('list_findings', { assessmentId: a.id }))
    } catch (e) {
      setError(String(e))
    }
  }

  useEffect(() => { load() }, [])

  useEffect(() => {
    if (view !== 'traffic' && view !== 'settings') return
    invoke<HistoryEntry[]>('list_traffic', { limit: 200 }).then(setTraffic).catch(e => setError(String(e)))
    invoke<ProxyStatus>('proxy_status').then(s => {
      setProxyStatus(s)
      setTorForm({ tor_enabled: s.tor_enabled, tor_addr: s.tor_addr })
    }).catch(e => setError(String(e)))
    const un = listen<HistoryEntry>('traffic-captured', e => {
      setTraffic(t => [e.payload, ...t].slice(0, 500))
    })
    return () => { un.then(f => f()) }
  }, [view])

  async function createAssessment() {
    try {
      await invoke('create_assessment', {
        name: form.name,
        target: form.target,
        environment: form.environment,
        scope: form.scope,
        methodology: form.methodology,
      })
      setShowNew(false)
      setForm({ ...form, name: '', target: '' })
      await load()
    } catch (e) {
      setError(String(e))
    }
  }

  async function removeAssessment(id: string) {
    try {
      await invoke('delete_assessment', { assessmentId: id })
      if (selected?.id === id) { setSelected(null); setFindings([]) }
      await load()
    } catch (e) {
      setError(String(e))
    }
  }

  async function sendRepeater() {
    setSending(true)
    try {
      setRepeaterResp(await invoke<RepeaterResponse>('repeater_send', { rawRequest: repeaterReq }))
    } catch (e) {
      setError(String(e))
    } finally {
      setSending(false)
    }
  }

  function toRepeater(entry: HistoryEntry) {
    setRepeaterReq(entry.request_headers || `GET ${entry.path || '/'} HTTP/1.1\r\nHost: ${entry.host}\r\nConnection: close\r\n\r\n`)
    setRepeaterResp(null)
    setView('repeater')
  }

  const counts = {
    total: assessments.length,
    active: assessments.filter(a => a.status === 'Active').length,
    draft: assessments.filter(a => a.status === 'Draft').length,
  }

  return (
    <div className="app">
      <aside className="sidebar">
        <div className="logo"><span className="dot" />SENTINEL</div>
        {NAV.map(n => (
          <button
            key={n.id}
            className={'nav-item' + (view === n.id ? ' active' : '')}
            onClick={() => setView(n.id)}
          >
            <span>{n.icon}</span>{n.label}
            {n.id === 'findings' && findings.length > 0 && (
              <span className="count">{findings.length}</span>
            )}
          </button>
        ))}
        <div className="proxy-status">
          <div className="live"><span className="dot2" />Proxy listening</div>
          <div className="addr">127.0.0.1:8080</div>
        </div>
      </aside>

      <div className="main">
        <header className="topbar">
          <select
            value={selected?.id ?? ''}
            onChange={e => {
              const a = assessments.find(x => x.id === e.target.value) ?? null
              setSelected(a)
              if (a) loadFindings(a)
            }}
          >
            <option value="">No assessment selected</option>
            {assessments.map(a => (
              <option key={a.id} value={a.id}>{a.name}</option>
            ))}
          </select>
              <button className="cmd-btn">⌕ Search or run command <kbd>Ctrl K</kbd></button>
              {selected && <span style={{ fontSize: 11, color: 'var(--dim)' }}>◆ {selected.name}</span>}
        </header>

        <main className="content">
          {error && <div className="empty" style={{ borderColor: 'var(--sev-crit)', color: 'var(--sev-crit)' }}>{error}</div>}

          {view === 'home' && (
            <>
              <div className="stats">
                <div className="stat">
                  <div className="label">Assessments</div>
                  <div className="value">{counts.total}</div>
                  <div className="sub">{counts.active} active · {counts.draft} draft</div>
                </div>
                <div className="stat">
                  <div className="label">Proxy</div>
                  <div className="value" style={{ color: 'var(--ok)' }}>Up</div>
                  <div className="sub">127.0.0.1:8080</div>
                </div>
                <div className="stat">
                  <div className="label">Findings</div>
                  <div className="value">{findings.length}</div>
                  <div className="sub">{selected ? selected.name : 'no assessment'}</div>
                </div>
                <div className="stat">
                  <div className="label">Engine</div>
                  <div className="value">Rust</div>
                  <div className="sub">tokio · sqlite</div>
                </div>
              </div>

              <div className="section-title">Assessments</div>
              {assessments.length === 0 ? (
                <div className="empty">
                  <h3>No assessments yet</h3>
                  <p>Create your first assessment to start the workflow.</p>
                  <div style={{ marginTop: 14 }}>
                    <button className="new-assess" onClick={() => setShowNew(true)}>+ New Assessment</button>
                  </div>
                </div>
              ) : (
                <>
                  <div style={{ marginBottom: 10 }}>
                    <button className="new-assess" onClick={() => setShowNew(true)}>+ New Assessment</button>
                  </div>
                  <div className="assessments">
                    {assessments.map(a => (
                      <div key={a.id} className="assess-card" onClick={() => { setSelected(a); loadFindings(a) }}>
                        <div>
                          <div className="nm">{a.name}</div>
                          <div className="tg">{a.target}</div>
                        </div>
                        <div className="meta">
                          <span>{a.environment} · {a.scope}</span>
                          <span className={'badge ' + a.status.toLowerCase()}>{a.status}</span>
                          <button className="btn-ghost" onClick={e => { e.stopPropagation(); removeAssessment(a.id) }}>✕</button>
                        </div>
                      </div>
                    ))}
                  </div>
                </>
              )}
            </>
          )}

          {view === 'findings' && (
            selected ? (
              <div className="assessments">
                {findings.length === 0
                  ? <div className="empty"><h3>No findings</h3><p>Run the scanner to populate findings for this assessment.</p></div>
                  : findings.map(f => (
                    <div key={f.id} className={'assess-card finding ' + f.severity.toLowerCase()}>
                      <div>
                        <div className="nm">
                          <span className={SEV_CLASS[f.severity]}>[{f.severity.toUpperCase()}]</span> {f.title}
                        </div>
                        <div className="tg">{f.description}</div>
                      </div>
                      <div className="meta">
                        <span>{f.confidence} · {f.status}</span>
                      </div>
                    </div>
                  ))}
              </div>
            ) : <div className="empty"><h3>Select an assessment</h3><p>Pick one from the top bar to view its findings.</p></div>
          )}

          {view === 'repeater' && (
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 12, height: '100%' }}>
              <div className="pane" style={{ minHeight: 0 }}>
                <div className="hd">
                  <span className="tab on">Request</span>
                  <button
                    className="new-assess"
                    style={{ marginLeft: 'auto', padding: '4px 12px' }}
                    onClick={sendRepeater}
                    disabled={sending}
                  >
                    {sending ? 'Sending…' : '▶ Send'}
                  </button>
                </div>
                <textarea
                  value={repeaterReq}
                  onChange={e => setRepeaterReq(e.target.value)}
                  spellCheck={false}
                  style={{
                    flex: 1, background: 'var(--bg)', border: 'none', outline: 'none',
                    color: 'var(--dim)', fontFamily: 'var(--mono)', fontSize: 12,
                    padding: 12, resize: 'none', minHeight: 300,
                  }}
                />
              </div>
              <div className="pane" style={{ minHeight: 0 }}>
                <div className="hd">
                  <span className="tab on">Response</span>
                  {repeaterResp && (
                    <span className="ml-auto font-mono" style={{ marginLeft: 'auto', fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--mute)' }}>
                      {repeaterResp.protocol} · {repeaterResp.duration_ms}ms
                    </span>
                  )}
                </div>
                {repeaterResp ? (
                  <>
                    <div style={{ padding: '8px 12px', fontFamily: 'var(--mono)', fontSize: 12 }}>
                      <span className={repeaterResp.status_code >= 400 ? 'bad' : 'ok'}>
                        {repeaterResp.status_code} {repeaterResp.status_text}
                      </span>
                      <span style={{ color: 'var(--mute)' }}> — {repeaterResp.url}</span>
                    </div>
                    <pre className="body" style={{ borderTop: '1px solid var(--line)' }}>{repeaterResp.headers_text || '(no headers)'}</pre>
                    <pre className="body" style={{ borderTop: '1px solid var(--line)' }}>{repeaterResp.body || '(empty body)'}</pre>
                  </>
                ) : (
                  <pre className="body" style={{ color: 'var(--mute)' }}>No response yet. Edit the request and hit Send.</pre>
                )}
              </div>
            </div>
          )}

          {view === 'traffic' && (
            <>
              <div className="stats" style={{ gridTemplateColumns: 'repeat(3,1fr)', marginBottom: 12 }}>
                <div className="stat">
                  <div className="label">Captured</div>
                  <div className="value">{traffic.length}</div>
                  <div className="sub">live from proxy</div>
                </div>
                <div className="stat">
                  <div className="label">Errors</div>
                  <div className="value" style={{ color: 'var(--sev-crit)' }}>
                    {traffic.filter(t => t.status_code >= 500).length}
                  </div>
                  <div className="sub">5xx responses</div>
                </div>
                <div className="stat">
                  <div className="label">TLS</div>
                  <div className="value">{traffic.filter(t => t.tls_enabled).length}</div>
                  <div className="sub">HTTPS transactions</div>
                </div>
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: '3fr 2fr', gap: 12 }}>
                <div className="tbl">
                  <table>
                    <thead>
                      <tr><th>Method</th><th>Host</th><th>Path</th><th>Status</th><th>Size</th><th>Time</th></tr>
                    </thead>
                    <tbody>
                      {traffic.length === 0 ? (
                        <tr><td colSpan={6} style={{ textAlign: 'center', padding: 30, color: 'var(--mute)' }}>
                          No traffic yet — set your browser proxy to 127.0.0.1:{proxyStatus?.port ?? 8080}
                          {proxyStatus?.tor_enabled ? ' (routed via Tor)' : ''} and browse.
                        </td></tr>
                      ) : traffic.map(t => (
                        <tr key={t.id} className={selectedEntry?.id === t.id ? 'sel' : ''} onClick={() => setSelectedEntry(t)}>
                          <td><span className={'m ' + (t.method === 'GET' ? 'm1' : 'm2')}>{t.method}</span></td>
                          <td style={{ maxWidth: 160, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{t.host}</td>
                          <td style={{ maxWidth: 220, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{t.path}{t.query && `?${t.query}`}</td>
                          <td className={t.status_code >= 500 ? 'bad' : t.status_code >= 400 ? 'warn' : 'ok'}>{t.status_code}</td>
                          <td>{t.response_size > 1024 ? `${(t.response_size / 1024).toFixed(1)}k` : t.response_size}</td>
                          <td>{t.duration_ms}ms</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                  <div className="tfoot">
                    <span>● live tail</span><span>{traffic.length} shown</span>
                    {proxyStatus?.tor_enabled && <span style={{ color: 'var(--acc)' }}>via Tor {proxyStatus.tor_addr}</span>}
                  </div>
                </div>

                <div className="pane">
                  <div className="hd">
                    <span className="tab on">Request</span>
                    <span className="tab">Response</span>
                    <button
                      className="btn-ghost"
                      style={{ marginLeft: 'auto', padding: '3px 10px' }}
                      onClick={() => selectedEntry && toRepeater(selectedEntry)}
                    >
                      ↻ Send to Repeater
                    </button>
                  </div>
                  {selectedEntry ? (
                    <pre className="body">{selectedEntry.request_headers || '(empty)'}</pre>
                  ) : (
                    <pre className="body" style={{ color: 'var(--mute)' }}>Select a transaction to inspect it.</pre>
                  )}
                  {selectedEntry && (
                    <pre className="body" style={{ borderTop: '1px solid var(--line)' }}>
                      {selectedEntry.response_headers || '(no response)'}
                    </pre>
                  )}
                  <div className="fbar">
                    <span>{selectedEntry?.protocol ?? ''}</span>
                    <span>{selectedEntry ? `${selectedEntry.request_size}B → ${selectedEntry.response_size}B` : ''}</span>
                    <span style={{ marginLeft: 'auto' }}>{selectedEntry?.tls_enabled ? 'TLS intercepted' : 'plain'}</span>
                  </div>
                </div>
              </div>
            </>
          )}
          {view === 'scanner' && <div className="placeholder">Scanner view — wired to the scan pipeline (next milestone)</div>}
          {view === 'reports' && <div className="placeholder">Reports view — wired to the reporting module (next milestone)</div>}
          {view === 'settings' && (
            <div style={{ maxWidth: 480 }}>
              <div className="section-title" style={{ marginTop: 0 }}>Anonymity</div>
              <div className="stat" style={{ marginBottom: 12 }}>
                <div className="label" style={{ marginBottom: 10 }}>Route all outbound traffic through Tor (SOCKS5)</div>
                <div className="field">
                  <label>
                    <input
                      type="checkbox"
                      checked={torForm.tor_enabled}
                      onChange={e => setTorForm({ ...torForm, tor_enabled: e.target.checked })}
                      style={{ width: 'auto', marginRight: 8 }}
                    />
                    Enabled — requests leave through a SOCKS5 proxy (Tor or compatible)
                  </label>
                </div>
                <div className="field">
                  <label>SOCKS5 address (Tor default: 127.0.0.1:9050 — run the Tor service yourself)</label>
                  <input
                    value={torForm.tor_addr}
                    onChange={e => setTorForm({ ...torForm, tor_addr: e.target.value })}
                    placeholder="127.0.0.1:9050"
                    style={{ fontFamily: 'var(--mono)' }}
                  />
                </div>
                <button
                  className="new-assess"
                  disabled={!torForm.tor_addr}
                  style={{ opacity: torForm.tor_addr ? 1 : 0.5 }}
                  onClick={async () => {
                    try {
                      const s = await invoke<ProxyStatus>('set_proxy_mode', {
                        torEnabled: torForm.tor_enabled,
                        torAddr: torForm.tor_addr,
                      })
                      setProxyStatus(s)
                    } catch (e) { setError(String(e)) }
                  }}
                >
                  Apply & restart proxy
                </button>
                {proxyStatus && (
                  <div style={{ marginTop: 12, fontSize: 11, color: 'var(--dim)', fontFamily: 'var(--mono)' }}>
                    proxy: 127.0.0.1:{proxyStatus.port} · upstream: {proxyStatus.tor_enabled ? `socks5://${proxyStatus.tor_addr}` : 'direct'}
                  </div>
                )}
              </div>
              <div className="empty">
                <p>Tool HTTP clients (repeater, scanner, crawler) honor the same Tor setting via <code>socks5h</code> — DNS resolves through Tor too.</p>
              </div>
            </div>
          )}
        </main>
      </div>

      {showNew && (
        <div className="modal-overlay" onClick={() => setShowNew(false)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h3>New Assessment</h3>
            <div className="field">
              <label>Name</label>
              <input value={form.name} onChange={e => setForm({ ...form, name: e.target.value })} placeholder="Acme Corp — Web Pentest" />
            </div>
            <div className="field">
              <label>Target</label>
              <input value={form.target} onChange={e => setForm({ ...form, target: e.target.value })} placeholder="https://acme.com" />
            </div>
            <div className="field">
              <label>Environment</label>
              <select value={form.environment} onChange={e => setForm({ ...form, environment: e.target.value })}>
                {['Production', 'Staging', 'Development'].map(x => <option key={x}>{x}</option>)}
              </select>
            </div>
            <div className="field">
              <label>Scope</label>
              <select value={form.scope} onChange={e => setForm({ ...form, scope: e.target.value })}>
                {['Web Application', 'API', 'Mobile Backend', 'Network'].map(x => <option key={x}>{x}</option>)}
              </select>
            </div>
            <div className="field">
              <label>Methodology</label>
              <select value={form.methodology} onChange={e => setForm({ ...form, methodology: e.target.value })}>
                {['OWASP', 'PTES', 'OSSTMM', 'Custom'].map(x => <option key={x}>{x}</option>)}
              </select>
            </div>
            <div className="modal-actions">
              <button className="btn-ghost" onClick={() => setShowNew(false)}>Cancel</button>
              <button className="new-assess" onClick={createAssessment} disabled={!form.name || !form.target}
                style={{ opacity: !form.name || !form.target ? 0.5 : 1 }}>Create</button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
