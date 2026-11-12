import { useState } from 'react'
import { PaperPlaneRight } from '@phosphor-icons/react'
import * as api from '../lib/bridge'
import { bytes, ms } from '../lib/format'
import type { RepeaterResponse } from '../lib/types'
import { useApp } from '../state'
import { Code, Kbd } from '../components/Marks'
import { HttpView } from '../components/HttpView'
import { Tabs } from '../components/Tabs'

interface Sent { at: number; req: string; res: RepeaterResponse }

export function Repeater() {
  const app = useApp()
  const [sending, setSending] = useState(false)
  const [error, setError] = useState('')
  const [history, setHistory] = useState<Sent[]>([])
  const [idx, setIdx] = useState(0)
  const [part, setPart] = useState<'body' | 'headers'>('body')

  const current = history[idx]
  // Catch the two mistakes that make every send fail, before sending.
  const firstLine = app.repeaterReq.split(/\r?\n/)[0]?.trim() ?? ''
  const lineOk = /^[A-Z]+ \S+ HTTP\/\d(\.\d)?$/.test(firstLine)
  const hostOk = /^host:\s*\S+/im.test(app.repeaterReq)
  const problem = !app.repeaterReq.trim() ? 'Write a request to send.'
    : !lineOk ? 'The first line should look like GET /path HTTP/1.1'
    : !hostOk ? 'Add a Host header so Sentinel knows where to send it.'
    : ''

  const send = async () => {
    if (sending || problem) return
    setSending(true)
    setError('')
    try {
      const res = await api.repeaterSend(app.repeaterReq.replace(/\r?\n/g, '\r\n'))
      setHistory(h => [{ at: Date.now(), req: app.repeaterReq, res }, ...h].slice(0, 50))
      setIdx(0)
    } catch (e) {
      setError(String(e).replace(/^Error:\s*/, ''))
    } finally {
      setSending(false)
    }
  }

  return (
    <div className="split split-even">
      <section className="pane" aria-label="Request">
        <div className="pane-bar">
          <h1 className="pane-title">Request</h1>
          <button type="button" className="btn btn-primary btn-sm push-end" onClick={send} disabled={sending || !!problem} aria-keyshortcuts="Control+Enter" aria-describedby={problem ? 'rep-problem' : undefined}>
            <PaperPlaneRight size={14} aria-hidden="true" />
            {sending ? 'Sending' : 'Send'}
            <Kbd>Ctrl Enter</Kbd>
          </button>
        </div>
        <textarea
          className="editor mono"
          value={app.repeaterReq}
          onChange={e => app.setRepeaterReq(e.target.value)}
          onKeyDown={e => { if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') { e.preventDefault(); send() } }}
          spellCheck={false}
          aria-label="Raw HTTP request"
          aria-invalid={!!problem}
        />
        {problem && <p id="rep-problem" className="pane-hint help-bad" role="status">{problem}</p>}
        {history.length > 0 && (
          <div className="pane-foot mono">
            <label className="history-pick">
              <span>History</span>
              <select value={idx} onChange={e => { const i = Number(e.target.value); setIdx(i); app.setRepeaterReq(history[i].req) }}>
                {history.map((h, i) => (
                  <option key={h.at} value={i}>
                    #{history.length - i}  {h.res.status_code}  {h.req.split('\n')[0].slice(0, 60)}
                  </option>
                ))}
              </select>
            </label>
          </div>
        )}
      </section>

      <section className="pane" aria-label="Response" aria-busy={sending}>
        <div className="pane-bar">
          <h2 className="pane-title">Response</h2>
          {current && (
            <span className="mono resp-strip">
              <Code code={current.res.status_code} />
              <span className="dim">{current.res.status_text}</span>
              <span>{current.res.protocol}</span>
              <span>{ms(current.res.duration_ms)}</span>
              <span>{bytes(current.res.body.length)}</span>
            </span>
          )}
          {current && (
            <span className="push-end">
              <Tabs label="Response part" value={part} onChange={setPart} options={[
                { id: 'body', label: 'Body' },
                { id: 'headers', label: 'Headers' },
              ]} />
            </span>
          )}
        </div>
        {error ? (
          <div className="inline-error" role="alert">
            <strong>Request not sent.</strong> {error}
            <p className="note">Check that the first line is METHOD PATH HTTP/1.1 and that a Host header is present.</p>
          </div>
        ) : current ? (
          <div className="scroll">
            {part === 'body'
              ? <pre className="http">{prettyBody(current.res.body) || '(empty body)'}</pre>
              : <HttpView text={`${current.res.protocol} ${current.res.status_code} ${current.res.status_text}\n${current.res.headers_text}`} />}
            {current.res.body.length >= 256 * 1024 && <p className="note pad">Body truncated at 256 KB.</p>}
          </div>
        ) : (
          <div className="empty empty-quiet"><p>No response yet. <Kbd>Ctrl Enter</Kbd> sends the request.</p></div>
        )}
      </section>
    </div>
  )
}

function prettyBody(b: string): string {
  const t = b.trim()
  if (!(t.startsWith('{') || t.startsWith('['))) return b
  try { return JSON.stringify(JSON.parse(t), null, 2) } catch { return b }
}
