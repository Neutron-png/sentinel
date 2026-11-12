import { useEffect, useMemo, useRef, useState } from 'react'
import { Copy, Funnel, PaperPlaneRight } from '@phosphor-icons/react'
import { bytes, clock, hostnameOf, inScope, ms } from '../lib/format'
import { useApp } from '../state'
import { Code, Kbd, Method } from '../components/Marks'
import { HttpView } from '../components/HttpView'
import { Tabs } from '../components/Tabs'
import { useToast } from '../components/Toast'
import { Banner } from '../components/States'

export function Proxy() {
  const app = useApp()
  const toast = useToast()
  const [filter, setFilter] = useState('')
  const [selId, setSelId] = useState<string | null>(null)
  const [pane, setPane] = useState<'request' | 'response'>('request')
  const tbody = useRef<HTMLTableSectionElement>(null)

  // History is shared by every project; default to this project's scope so counts match the journey.
  const [scopeOnly, setScopeOnly] = useState<'scope' | 'all'>('scope')
  const scopeHost = app.selected ? hostnameOf(app.selected.target) : ''
  const base = useMemo(
    () => (scopeOnly === 'scope' && scopeHost ? app.traffic.filter(t => inScope(t.host, scopeHost)) : app.traffic),
    [app.traffic, scopeOnly, scopeHost],
  )
  const rows = useMemo(() => {
    const q = filter.trim().toLowerCase()
    if (!q) return base
    return base.filter(t =>
      `${t.method} ${t.host}${t.path}?${t.query} ${t.status_code}`.toLowerCase().includes(q))
  }, [base, filter])

  const sel = rows.find(r => r.id === selId) ?? null
  const port = app.proxy?.port ?? 8080

  // J / K move through rows, Ctrl R sends the selection to Repeater.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const t = e.target as HTMLElement
      if (t.closest('input, textarea, select')) return
      if (e.key === 'j' || e.key === 'k') {
        const i = rows.findIndex(r => r.id === selId)
        const n = rows[Math.max(0, Math.min(rows.length - 1, i + (e.key === 'j' ? 1 : -1)))]
        if (n) {
          setSelId(n.id)
          tbody.current?.querySelector(`[data-id="${n.id}"]`)?.scrollIntoView({ block: 'nearest' })
        }
      }
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'r' && sel) {
        e.preventDefault()
        app.toRepeater(sel.request_headers)
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [rows, selId, sel, app])

  const errors = base.filter(t => t.status_code >= 500).length
  const hosts = new Set(base.map(t => t.host)).size

  return (
    <div className="split split-proxy">
      <section className="pane" aria-label="Captured traffic">
        <div className="pane-bar">
          <h1 className="pane-title">Proxy history</h1>
          {scopeHost && (
            <Tabs label="Traffic shown" value={scopeOnly} onChange={setScopeOnly} options={[
              { id: 'scope', label: 'In scope', count: app.traffic.filter(t => inScope(t.host, scopeHost)).length },
              { id: 'all', label: 'All', count: app.traffic.length },
            ]} />
          )}
          <span className="mono dim pane-sub">
            {hosts} host{hosts === 1 ? '' : 's'}{errors > 0 && <>, <span className="code-bad">{errors} 5xx</span></>}
          </span>
          <label className="filter">
            <Funnel size={14} aria-hidden="true" />
            <span className="sr-only">Filter traffic</span>
            <input value={filter} onChange={e => setFilter(e.target.value)} placeholder="Filter by host, path or status" spellCheck={false} />
          </label>
        </div>

        {app.proxy && !app.proxy.listening && (
          <Banner tone="bad" action={<button type="button" className="btn btn-ghost btn-sm" onClick={() => app.go('settings')}>Open settings</button>}>
            The proxy is not listening on 127.0.0.1:{port}. New traffic will not be captured.
          </Banner>
        )}
        {app.trafficError && (
          <Banner action={<button type="button" className="btn btn-ghost btn-sm" onClick={app.refreshTraffic}>Try again</button>}>
            History could not be loaded: {app.trafficError}
          </Banner>
        )}
        {app.traffic.length === 0 ? (
          <div className="empty">
            <h2 className="h3">No traffic captured yet</h2>
            <p>Point your browser or system proxy at this address, then browse the target.</p>
            <div className="copy-line">
              <code className="mono">127.0.0.1:{port}</code>
              <button type="button" className="btn btn-ghost btn-sm" onClick={() => {
                if (!navigator.clipboard) { toast('Clipboard is not available here', 'bad'); return }
                navigator.clipboard.writeText(`127.0.0.1:${port}`).then(() => toast('Proxy address copied'), () => toast('Copy failed', 'bad'))
              }}>
                <Copy size={14} aria-hidden="true" /> Copy
              </button>
            </div>
            <p className="note">HTTPS needs the Sentinel CA certificate installed in the browser.</p>
          </div>
        ) : (
          <div className="table-wrap">
            <table className="table">
              <thead>
                <tr>
                  <th className="col-time">Time</th>
                  <th className="col-method">Method</th>
                  <th>Host</th>
                  <th>Path</th>
                  <th className="num">Status</th>
                  <th className="num">Size</th>
                  <th className="num">Time</th>
                </tr>
              </thead>
              <tbody ref={tbody}>
                {rows.length === 0 && (
                  <tr><td colSpan={7} className="table-empty">
                    {filter
                      ? <>No request matches "{filter}".</>
                      : <>Nothing captured for {scopeHost} yet. {app.traffic.length} request{app.traffic.length === 1 ? '' : 's'} went to other hosts.{' '}
                        <button type="button" className="link-btn" onClick={() => setScopeOnly('all')}>Show all traffic</button></>}
                  </td></tr>
                )}
                {rows.map(t => (
                  <tr
                    key={t.id}
                    data-id={t.id}
                    className={(t.id === selId ? 'is-selected ' : '') + (app.freshIds.has(t.id) ? 'is-fresh' : '')}
                    onClick={() => setSelId(t.id)}
                    aria-selected={t.id === selId}
                  >
                    <td className="mono dim col-time">{clock(t.timestamp)}</td>
                    <td className="col-method"><Method m={t.method} /></td>
                    <td className="mono truncate col-host">{t.host}</td>
                    <td className="mono truncate col-path">{t.path}{t.query && <span className="dim">?{t.query}</span>}</td>
                    <td className="num"><Code code={t.status_code} /></td>
                    <td className="num mono">{bytes(t.response_size)}</td>
                    <td className={'num mono' + (t.duration_ms > 2000 ? ' code-warn' : '')}>{ms(t.duration_ms)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
        <div className="pane-foot mono">
          <span>{filter ? `${rows.length} of ${base.length} shown` : 'Live, newest first, 500 most recent kept'}</span>
          <span className="foot-keys"><Kbd>J</Kbd><Kbd>K</Kbd> move <Kbd>Ctrl R</Kbd> to Repeater</span>
        </div>
      </section>

      <section className="pane" aria-label="Message inspector">
        <div className="pane-bar">
          <Tabs label="Message part" value={pane} onChange={setPane} options={[
            { id: 'request', label: 'Request' },
            { id: 'response', label: 'Response' },
          ]} />
          <button
            type="button"
            className="btn btn-primary btn-sm push-end"
            disabled={!sel}
            onClick={() => sel && app.toRepeater(sel.request_headers)}
          >
            <PaperPlaneRight size={14} aria-hidden="true" /> Send to Repeater
          </button>
        </div>
        {sel ? (
          <>
            <div className="msg-meta mono">
              <span>{sel.protocol}</span>
              <span>{sel.tls_enabled ? 'TLS intercepted' : 'Plain HTTP'}</span>
              <span>{bytes(sel.request_size)} out, {bytes(sel.response_size)} in</span>
            </div>
            <div className="scroll">
              <HttpView
                text={pane === 'request' ? sel.request_headers : sel.response_headers}
                empty={pane === 'request' ? 'The request was not recorded.' : 'No response. The connection closed or timed out before the server answered.'}
              />
            </div>
          </>
        ) : (
          <div className="empty empty-quiet">
            <p>Select a request to inspect it. The same message can move to Repeater, the Scanner or a finding.</p>
          </div>
        )}
      </section>
    </div>
  )
}
