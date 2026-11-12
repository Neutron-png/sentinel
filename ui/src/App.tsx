import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import {
  ArrowsLeftRight, Binary, CheckCircle, Crosshair, FileText, Gear, House, MagnifyingGlass, Moon,
  PaperPlaneRight, ShieldCheck, Sun, Target, XCircle,
} from '@phosphor-icons/react'
import type { Icon } from '@phosphor-icons/react'
import * as api from './lib/bridge'
import { hostnameOf as hostnameOfTarget, inScope, needsDecision } from './lib/format'
import type { Assessment, HistoryEntry, ProxyStatus, UiFinding, View } from './lib/types'
import { AppContext, type AppState } from './state'
import { CommandPalette, type Command } from './components/CommandPalette'
import { Logo, Kbd } from './components/Marks'
import { Modal } from './components/Modal'
import { useToast } from './components/Toast'
import { ErrorState } from './components/States'
import { Overview } from './views/Overview'
import { Proxy } from './views/Proxy'
import { Repeater } from './views/Repeater'
import { Scanner } from './views/Scanner'
import { Decoder } from './views/Decoder'
import { Findings } from './views/Findings'
import { Reports } from './views/Reports'
import { Settings } from './views/Settings'

/* Primary nav follows the journey: scope, capture, test, decide, report. */
const NAV: { id: View; label: string; icon: Icon }[] = [
  { id: 'overview', label: 'Project', icon: House },
  { id: 'proxy', label: 'Proxy', icon: ArrowsLeftRight },
  { id: 'repeater', label: 'Repeater', icon: PaperPlaneRight },
  { id: 'scanner', label: 'Scanner', icon: Crosshair },
  { id: 'findings', label: 'Findings', icon: ShieldCheck },
  { id: 'reports', label: 'Reports', icon: FileText },
]
const UTILITY: { id: View; label: string; icon: Icon }[] = [
  { id: 'decoder', label: 'Decoder', icon: Binary },
  { id: 'settings', label: 'Settings', icon: Gear },
]

const VIEWS: View[] = ['overview', 'proxy', 'repeater', 'scanner', 'decoder', 'findings', 'reports', 'settings']
const viewFromHash = (): View => {
  const v = location.hash.replace(/^#\/?/, '').split('?')[0] as View
  return VIEWS.includes(v) ? v : 'overview'
}

function readTheme(): 'dark' | 'light' {
  const q = new URLSearchParams(location.search).get('theme')
  if (q === 'light' || q === 'dark') return q
  try {
    const s = localStorage.getItem('sentinel.theme')
    if (s === 'light' || s === 'dark') return s
  } catch { /* storage unavailable */ }
  return 'dark'
}

const clean = (e: unknown) => String(e).replace(/^Error:\s*/, '')
const hostOf = (target: string) => {
  try { return new URL(/^https?:\/\//i.test(target) ? target : `https://${target}`).host } catch { return target }
}

const DEFAULT_REQ = 'GET / HTTP/1.1\r\nHost: portal.halden-freight.test\r\nAccept: text/html\r\nConnection: close\r\n\r\n'

export default function App() {
  const toast = useToast()
  const [view, setView] = useState<View>(viewFromHash)
  const [theme, setTheme] = useState(readTheme)
  const [palette, setPalette] = useState(false)

  const [assessments, setAssessments] = useState<Assessment[]>([])
  const [selectedId, setSelectedId] = useState<string | null>(null)
  const [findings, setFindings] = useState<UiFinding[]>([])
  const [findingsLoading, setFindingsLoading] = useState(true)
  const [focusFinding, setFocusFinding] = useState<string | null>(null)
  const [proxy, setProxy] = useState<ProxyStatus | null>(null)
  const [traffic, setTraffic] = useState<HistoryEntry[]>([])
  const [freshIds, setFreshIds] = useState<Set<string>>(new Set())
  const [repeaterReq, setRepeaterReq] = useState(DEFAULT_REQ)
  const [newOpen, setNewOpen] = useState(false)
  const [projectsError, setProjectsError] = useState<string | null>(null)
  const [findingsError, setFindingsError] = useState<string | null>(null)
  const [trafficError, setTrafficError] = useState<string | null>(null)
  const findingsReq = useRef(0)

  const selected = assessments.find(a => a.id === selectedId) ?? null

  const fail = useCallback((e: unknown) => toast(String(e).replace(/^Error:\s*/, ''), 'bad'), [toast])

  const go = useCallback((v: View) => {
    if (location.hash !== `#/${v}`) history.pushState(null, '', `#/${v}`)
    setView(v)
  }, [])

  useEffect(() => {
    const on = () => setView(viewFromHash())
    window.addEventListener('popstate', on)
    window.addEventListener('hashchange', on)
    return () => { window.removeEventListener('popstate', on); window.removeEventListener('hashchange', on) }
  }, [])

  useEffect(() => {
    document.documentElement.dataset.theme = theme
    try { localStorage.setItem('sentinel.theme', theme) } catch { /* ignore */ }
  }, [theme])

  const [projectsLoaded, setProjectsLoaded] = useState(false)

  const refreshAssessments = useCallback(async () => {
    try {
      const list = await api.listAssessments()
      setAssessments(list)
      setProjectsError(null)
      setSelectedId(id => (id && list.some(a => a.id === id) ? id : list[0]?.id ?? null))
    } catch (e) { setProjectsError(clean(e)) } finally { setProjectsLoaded(true) }
  }, [])

  // Until projects are known, findings count as loading so no false "No findings" flashes.
  // A request token drops late answers from a project the user already switched away from.
  const refreshFindings = useCallback(async () => {
    const req = ++findingsReq.current
    if (!selectedId) { setFindings([]); if (projectsLoaded) setFindingsLoading(false); return }
    setFindingsLoading(true)
    try {
      const list = await api.listFindings(selectedId)
      if (req !== findingsReq.current) return
      setFindings(list)
      setFindingsError(null)
    } catch (e) {
      if (req === findingsReq.current) setFindingsError(clean(e))
    } finally {
      if (req === findingsReq.current) setFindingsLoading(false)
    }
  }, [selectedId, projectsLoaded])

  const refreshTraffic = useCallback(async () => {
    try {
      setTraffic(await api.listTraffic(200))
      setTrafficError(null)
    } catch (e) { setTrafficError(clean(e)) }
  }, [])

  // Switching project clears the previous project's findings at once, never shows them under the new name.
  const selectProject = useCallback((id: string | null) => {
    setSelectedId(cur => {
      if (cur !== id) { setFindings([]); setFindingsLoading(true); setFocusFinding(null) }
      return id
    })
  }, [])

  useEffect(() => { refreshAssessments() }, [refreshAssessments])
  useEffect(() => { refreshFindings() }, [refreshFindings])

  // Proxy status and the live traffic tail run for the whole session, not per view.
  useEffect(() => {
    api.proxyStatus().then(setProxy).catch(() => setProxy({ listening: false, port: 8080, tor_enabled: false, tor_addr: '' }))
    refreshTraffic()
    return api.onTraffic(entry => {
      setTraffic(t => [entry, ...t].slice(0, 500))
      setFreshIds(s => new Set(s).add(entry.id))
      setTimeout(() => setFreshIds(s => { const n = new Set(s); n.delete(entry.id); return n }), 700)
    })
  }, [refreshTraffic])

  const toRepeater = useCallback((raw: string) => {
    setRepeaterReq(raw.includes('\n\n') || raw.includes('\r\n\r\n') ? raw : raw.replace(/\s*$/, '') + '\r\n\r\n')
    go('repeater')
    toast('Request sent to Repeater')
  }, [go, toast])

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') { e.preventDefault(); setPalette(p => !p) }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [])

  const commands = useMemo<Command[]>(() => [
    ...[...NAV, ...UTILITY]
      .map(n => ({ id: `go-${n.id}`, group: 'Go to', label: n.label, run: () => go(n.id) })),
    { id: 'new', group: 'Project', label: 'New project', run: () => setNewOpen(true) },
    { id: 'scan', group: 'Action', label: 'Start a scan', run: () => go('scanner') },
    { id: 'decide', group: 'Action', label: 'Review findings that need a decision', run: () => go('findings') },
    { id: 'export', group: 'Action', label: 'Export a report', run: () => go('reports') },
    { id: 'proxy-addr', group: 'Action', label: 'Copy proxy address', run: () => {
      const addr = `127.0.0.1:${proxy?.port ?? 8080}`
      if (!navigator.clipboard) { toast('Clipboard is not available here', 'bad'); return }
      navigator.clipboard.writeText(addr).then(() => toast('Proxy address copied'), () => toast('Copy failed', 'bad'))
    } },
    ...assessments.map(a => ({ id: `p-${a.id}`, group: 'Switch project', label: a.name, run: () => selectProject(a.id) })),
    { id: 'theme', group: 'Appearance', label: theme === 'dark' ? 'Use light theme' : 'Use dark theme', run: () => setTheme(t => (t === 'dark' ? 'light' : 'dark')) },
  ], [assessments, go, theme, selectProject, proxy, toast])

  // Badges show what needs attention in this project, not global totals.
  const scopeHost = selected ? hostnameOfTarget(selected.target) : ''
  const scopedTraffic = scopeHost ? traffic.filter(t => inScope(t.host, scopeHost)).length : traffic.length

  const state: AppState = {
    assessments, selected, select: selectProject, refreshAssessments, openNewProject: () => setNewOpen(true),
    projectsLoaded, projectsError,
    findings, findingsLoading, findingsError, refreshFindings, focusFinding, setFocusFinding,
    proxy, setProxy, traffic, freshIds, trafficError, refreshTraffic,
    repeaterReq, setRepeaterReq, toRepeater,
    view, go, fail,
  }

  return (
    <AppContext.Provider value={state}>
      <a className="skip" href="#main">Skip to content</a>
      <div className="shell">
        <aside className="sidebar" aria-label="Primary">
          <div className="brand">
            <Logo />
            <span>Sentinel</span>
          </div>
          <nav className="nav">
            {NAV.map(n => <NavItem key={n.id} n={n} view={view} go={go} count={n.id === 'findings' ? findings.filter(needsDecision).length : n.id === 'proxy' ? scopedTraffic : 0} />)}
          </nav>
          <div className="sidebar-foot">
            {UTILITY.map(n => <NavItem key={n.id} n={n} view={view} go={go} count={0} />)}
            <button type="button" className="proxy-block" onClick={() => go('settings')} aria-label="Proxy status, open settings">
              <span className={'proxy-line' + (proxy && !proxy.listening ? ' is-down' : '')}>
                {proxy?.listening ? <CheckCircle size={14} weight="fill" className="live-icon is-live" aria-hidden="true" /> : <XCircle size={14} weight="fill" className="live-icon" aria-hidden="true" />}
                {proxy?.listening ? 'Proxy listening' : 'Proxy stopped'}
              </span>
              <span className="mono proxy-addr">127.0.0.1:{proxy?.port ?? 8080}</span>
              <span className="mono proxy-up">{proxy?.tor_enabled ? `via socks5h ${proxy.tor_addr}` : 'upstream direct'}</span>
            </button>
          </div>
        </aside>

        <div className="main">
          <header className="topbar">
            {assessments.length > 0 ? (
              <label className="project-switch">
                <span className="sr-only">Project</span>
                <select value={selectedId ?? ''} onChange={e => selectProject(e.target.value || null)}>
                  {assessments.map(a => <option key={a.id} value={a.id}>{a.name}</option>)}
                </select>
              </label>
            ) : projectsLoaded && !projectsError && (
              <button type="button" className="btn btn-ghost btn-sm" onClick={() => setNewOpen(true)}>New project</button>
            )}
            {selected && (
              <span className="scope-chip mono" title="In scope. Every engine checks this before sending.">
                <Target size={14} aria-hidden="true" />
                <span className="truncate">{hostOf(selected.target)}</span>
              </span>
            )}
            <button type="button" className="cmd-trigger" onClick={() => setPalette(true)}>
              <MagnifyingGlass size={14} aria-hidden="true" />
              <span>Go to or run</span>
              <Kbd>Ctrl K</Kbd>
            </button>
            <div className="topbar-end">
              {!api.isTauri && <span className="demo-note" title="Running outside the desktop shell. Data is a fixed sample.">Demo data</span>}
              <button
                type="button"
                className="btn-icon"
                aria-label={theme === 'dark' ? 'Use light theme' : 'Use dark theme'}
                onClick={() => setTheme(t => (t === 'dark' ? 'light' : 'dark'))}
              >
                {theme === 'dark' ? <Sun size={16} /> : <Moon size={16} />}
              </button>
            </div>
          </header>

          <main id="main" className={`content content-${view}`} tabIndex={-1}>
            <div className="view" key={view}>
            {projectsError && assessments.length === 0 && view !== 'decoder' && view !== 'settings' ? (
              <ErrorState title="Projects could not be loaded" detail={projectsError} onRetry={refreshAssessments} />
            ) : <>
            {view === 'overview' && <Overview />}
            {view === 'proxy' && <Proxy />}
            {view === 'repeater' && <Repeater />}
            {view === 'scanner' && <Scanner />}
            {view === 'decoder' && <Decoder />}
            {view === 'findings' && <Findings />}
            {view === 'reports' && <Reports />}
            {view === 'settings' && <Settings theme={theme} setTheme={setTheme} />}
            </>}
            </div>
          </main>
        </div>
      </div>

      <CommandPalette open={palette} onClose={() => setPalette(false)} commands={commands} />
      <NewProject open={newOpen} onClose={() => setNewOpen(false)} onCreated={async a => {
        await refreshAssessments()
        selectProject(a.id)
        go('overview')
        toast(`Project "${a.name}" created`)
      }} />
    </AppContext.Provider>
  )
}

function NewProject({ open, onClose, onCreated }: { open: boolean; onClose: () => void; onCreated: (a: Assessment) => void }) {
  const [form, setForm] = useState({ name: '', target: '', environment: 'Staging', scope: 'Web Application', methodology: 'OWASP' })
  const [busy, setBusy] = useState(false)
  const [touched, setTouched] = useState(false)
  const [submitError, setSubmitError] = useState('')
  // Accepts hosts, IPs and localhost with ports, as long as it is a full http(s) URL.
  const targetOk = (() => {
    try { const u = new URL(form.target.trim()); return /^https?:$/.test(u.protocol) && u.hostname !== '' } catch { return false }
  })()
  const nameOk = form.name.trim() !== '' && form.name.trim().length <= 120
  const ok = nameOk && targetOk

  const submit = async () => {
    setTouched(true)
    setSubmitError('')
    if (!ok) return
    setBusy(true)
    try {
      const a = await api.createAssessment({ ...form, name: form.name.trim(), target: form.target.trim() })
      onClose()
      setForm(f => ({ ...f, name: '', target: '' }))
      setTouched(false)
      onCreated(a)
    } catch (e) { setSubmitError(clean(e)) } finally { setBusy(false) }
  }

  return (
    <Modal
      open={open}
      onClose={onClose}
      title="New project"
      footer={<>
        <button type="button" className="btn btn-ghost" onClick={onClose}>Cancel</button>
        <button type="button" className="btn btn-primary" onClick={submit} disabled={busy}>{busy ? 'Creating' : 'Create project'}</button>
      </>}
    >
      <form onSubmit={e => { e.preventDefault(); submit() }} className="form">
        <div className="field">
          <label htmlFor="np-name">Name</label>
          <input id="np-name" value={form.name} onChange={e => setForm({ ...form, name: e.target.value })} autoComplete="off"
            maxLength={120} aria-invalid={touched && !nameOk} aria-describedby={touched && !nameOk ? 'np-name-err' : undefined} />
          {touched && !nameOk && <p id="np-name-err" className="help help-bad">Give the project a name.</p>}
        </div>
        <div className="field">
          <label htmlFor="np-target">Target</label>
          <input
            id="np-target"
            className="mono"
            value={form.target}
            onChange={e => setForm({ ...form, target: e.target.value })}
            aria-invalid={touched && !targetOk}
            aria-describedby="np-target-help"
            autoComplete="off"
            spellCheck={false}
          />
          <p id="np-target-help" className={'help' + (touched && !targetOk ? ' help-bad' : '')}>
            {touched && !targetOk
              ? 'Enter a full URL with http:// or https://, for example https://portal.example.test or http://localhost:3000'
              : 'This host becomes the first include rule. Nothing outside scope is ever sent.'}
          </p>
        </div>
        <div className="field-row">
          <div className="field">
            <label htmlFor="np-env">Environment</label>
            <select id="np-env" value={form.environment} onChange={e => setForm({ ...form, environment: e.target.value })}>
              {['Production', 'Staging', 'Development'].map(x => <option key={x}>{x}</option>)}
            </select>
          </div>
          <div className="field">
            <label htmlFor="np-scope">Type</label>
            <select id="np-scope" value={form.scope} onChange={e => setForm({ ...form, scope: e.target.value })}>
              {['Web Application', 'API', 'Mobile Backend', 'Network'].map(x => <option key={x}>{x}</option>)}
            </select>
          </div>
          <div className="field">
            <label htmlFor="np-method">Methodology</label>
            <select id="np-method" value={form.methodology} onChange={e => setForm({ ...form, methodology: e.target.value })}>
              {['OWASP', 'PTES', 'OSSTMM', 'Custom'].map(x => <option key={x}>{x}</option>)}
            </select>
          </div>
        </div>
        {submitError && <p className="inline-error inline-error-flush" role="alert"><strong>Project not created.</strong> {submitError}</p>}
        <button type="submit" hidden />
      </form>
    </Modal>
  )
}

function NavItem({ n, view, go, count }: { n: { id: View; label: string; icon: Icon }; view: View; go: (v: View) => void; count: number }) {
  return (
    <a
      href={`#/${n.id}`}
      className="nav-item"
      aria-current={view === n.id ? 'page' : undefined}
      onClick={e => { e.preventDefault(); go(n.id) }}
    >
      <n.icon size={16} aria-hidden="true" />
      <span>{n.label}</span>
      {count > 0 && <span className="nav-count">{count}</span>}
    </a>
  )
}
