import { useEffect, useMemo, useState } from 'react'
import { ArrowCounterClockwise, CaretDown, Check, Circle, Copy, PaperPlaneRight, SealCheck, ShieldSlash, X } from '@phosphor-icons/react'
import * as api from '../lib/bridge'
import { needsDecision } from '../lib/format'
import { SEVERITY_ORDER, STATUS_ORDER } from '../lib/types'
import type { CoreStatus, GateStep, UiFinding, VerificationStatus } from '../lib/types'
import { useApp } from '../state'
import { SeverityTag, StatusMark } from '../components/Marks'
import { HttpView } from '../components/HttpView'
import { ErrorState } from '../components/States'
import { useToast } from '../components/Toast'

/* One sentence per status: what it means for the tester, right now. */
const VERDICT: Record<VerificationStatus, string> = {
  Confirmed: 'Proven. It passed all nine questions and is ready to report.',
  Detected: 'Observed and reproduced, but not independently verified.',
  'Needs Manual Verification': 'The engine could not decide. You need to check it.',
  Potential: 'Something changed, but a harmless explanation is still possible.',
  Informational: 'Not a vulnerability. Kept as context.',
}

export function Findings() {
  const app = useApp()
  const [selId, setSelId] = useState<string | null>(app.focusFinding)

  useEffect(() => { if (app.focusFinding) { setSelId(app.focusFinding); app.setFocusFinding(null) } }, [app])

  const sorted = useMemo(() => [...app.findings].sort((a, b) =>
    STATUS_ORDER.indexOf(a.status) - STATUS_ORDER.indexOf(b.status) ||
    SEVERITY_ORDER.indexOf(a.severity) - SEVERITY_ORDER.indexOf(b.severity)), [app.findings])
  const open = sorted.filter(f => !f.decision)
  const decided = sorted.filter(f => f.decision)
  const groups = STATUS_ORDER.map(s => [s, open.filter(f => f.status === s)] as const).filter(([, l]) => l.length)
  const waiting = app.findings.filter(needsDecision).length
  const sel = app.findings.find(f => f.id === selId) ?? sorted[0] ?? null

  if (!app.selected && app.findingsLoading) return null
  if (!app.selected) {
    return (
      <div className="empty-page">
        <h1 className="h1">No project yet</h1>
        <p className="lead">Findings belong to a project. Create one to start.</p>
        <button type="button" className="btn btn-primary" onClick={app.openNewProject}>New project</button>
      </div>
    )
  }

  if (app.findingsError && app.findings.length === 0) {
    return <ErrorState title="Findings could not be loaded" detail={app.findingsError} onRetry={app.refreshFindings} />
  }

  if (!app.findingsLoading && app.findings.length === 0) {
    return (
      <div className="empty-page">
        <h1 className="h1">No findings yet</h1>
        <p className="lead">That can be the right answer. Sentinel only records what it can show. Scan the target, or replay a request in Repeater.</p>
        <div className="btn-row">
          <button type="button" className="btn btn-primary" onClick={() => app.go('scanner')}>Start a scan</button>
          <button type="button" className="btn btn-quiet" onClick={() => app.go('repeater')}>Open Repeater</button>
        </div>
      </div>
    )
  }

  return (
    <div className="split split-findings">
      <section className="pane" aria-label="Findings list">
        <div className="pane-bar">
          <h1 className="pane-title">Findings</h1>
          <span className="mono dim pane-sub">{app.findings.length}</span>
          {waiting > 0 && <span className="pane-sub push-end waiting-note">{waiting} waiting on you</span>}
        </div>
        <div className="scroll">
          {app.findingsLoading ? (
            <div className="skeleton-list" aria-label="Loading findings"><span /><span /><span /></div>
          ) : (
            groups.map(([status, list]) => (
              <div key={status} className="f-group">
                <h2 className="f-group-head">
                  <StatusMark status={status} label />
                  <span className="mono dim">{list.length}</span>
                </h2>
                <ul>
                  {list.map(f => (
                    <li key={f.id}>
                      <button
                        type="button"
                        className="f-row"
                        aria-current={sel?.id === f.id ? 'true' : undefined}
                        onClick={() => setSelId(f.id)}
                      >
                        <span className="f-title">{f.title || 'Untitled finding'}</span>
                        <span className="f-meta">
                          <SeverityTag severity={f.severity} compact />
                          {f.endpoint && <span className="mono dim truncate">{f.endpoint}</span>}
                        </span>
                      </button>
                    </li>
                  ))}
                </ul>
              </div>
            ))
          )}
          {!app.findingsLoading && decided.length > 0 && (
            <div className="f-group f-group-decided">
              <h2 className="f-group-head"><span className="status-label dim">Decided</span><span className="mono dim">{decided.length}</span></h2>
              <ul>
                {decided.map(f => (
                  <li key={f.id}>
                    <button type="button" className="f-row" aria-current={sel?.id === f.id ? 'true' : undefined} onClick={() => setSelId(f.id)}>
                      <span className="f-title">{f.title || 'Untitled finding'}</span>
                      <span className="f-meta">
                        <span className="decision-tag">{f.decision === 'false-positive' ? 'False positive' : 'Accepted risk'}</span>
                        <SeverityTag severity={f.severity} compact />
                      </span>
                    </button>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      </section>

      {sel ? <FindingDetail key={sel.id} f={sel} /> : <section className="pane" />}
    </div>
  )
}

function FindingDetail({ f }: { f: UiFinding }) {
  const app = useApp()
  const toast = useToast()
  const held = f.gate ? f.gate.map((g, i) => ({ g, n: i + 1 })).filter(x => x.g.state !== 'passed') : []
  const [showAll, setShowAll] = useState(false)

  const copy = (text: string, what: string) => {
    if (!navigator.clipboard) { toast('Clipboard is not available here', 'bad'); return }
    navigator.clipboard.writeText(text).then(() => toast(`${what} copied`), () => toast('Copy failed. Select the text and copy it manually.', 'bad'))
  }

  return (
    <section className="pane detail" aria-labelledby="fd-title">
      <div className="scroll detail-scroll">
        <header className="fd-head">
          <div className="fd-tags">
            <SeverityTag severity={f.severity} />
            {f.endpoint && <span className="mono fd-loc">{f.endpoint}{f.parameter && <span className="dim"> ({f.parameter})</span>}</span>}
          </div>
          <h2 className="h2" id="fd-title">{f.title || 'Untitled finding'}</h2>
          {f.description && <p className="fd-desc">{f.description}</p>}
        </header>

        {/* The verdict comes first: what this status means and what held it back. */}
        <section className={`verdict verdict-${f.status === 'Confirmed' ? 'proven' : f.status === 'Informational' ? 'info' : 'open'}`} aria-labelledby="verdict-h">
          <div className="verdict-top">
            <StatusMark status={f.status} label />
            {f.confidence && <span className="mono dim">confidence {f.confidence.toLowerCase()}</span>}
          </div>
          <p className="verdict-line" id="verdict-h">
            {f.status === 'Confirmed' && held.length > 0
              ? 'You verified it. The engine could not prove it on its own, so the gaps below stay on record.'
              : VERDICT[f.status]}
          </p>

          {f.gate ? (
            <>
              {held.length > 0 && (
                <ol className="gate gate-held-list" aria-label="Questions that held this finding back">
                  {held.map(({ g, n }) => <GateRow key={n} n={n} g={g} />)}
                </ol>
              )}
              <button type="button" className="disclose" aria-expanded={showAll} onClick={() => setShowAll(v => !v)}>
                <CaretDown size={12} weight="bold" aria-hidden="true" />
                {showAll ? 'Hide the nine questions' : held.length > 0 ? `Show all nine questions (${9 - held.length} passed)` : 'Show all nine questions'}
              </button>
              {showAll && <ol className="gate">{f.gate.map((g, i) => <GateRow key={i} n={i + 1} g={g} />)}</ol>}
            </>
          ) : (
            <p className="note gate-missing">No gate record is stored for this finding. It was recorded before verification ran, so only its status and confidence are known.</p>
          )}
          <DecisionBar f={f} />
        </section>

        {f.baseline && f.observed && (
          <section className="fd-section" aria-labelledby="delta-h">
            <h3 className="h3 fd-h" id="delta-h">What changed</h3>
            <div className="compare">
              <figure className="well">
                <figcaption>Baseline</figcaption>
                <HttpView text={f.baseline} />
              </figure>
              <figure className="well well-observed">
                <figcaption>Observed</figcaption>
                <HttpView text={f.observed} delta={f.delta} />
              </figure>
            </div>
          </section>
        )}

        {f.reproduction && (
          <section className="fd-section" aria-labelledby="repro-h">
            <div className="fd-section-head">
              <h3 className="h3" id="repro-h">Reproduce it</h3>
              <span className="push-end btn-row">
                <button type="button" className="btn btn-quiet btn-sm" onClick={() => copy(f.reproduction!, 'Request')}>
                  <Copy size={14} aria-hidden="true" /> Copy
                </button>
                <button type="button" className="btn btn-ghost btn-sm" onClick={() => app.toRepeater(f.reproduction!)}>
                  <PaperPlaneRight size={14} aria-hidden="true" /> Send to Repeater
                </button>
              </span>
            </div>
            <div className="well"><HttpView text={f.reproduction} /></div>
          </section>
        )}

        {(f.impact || f.recommendation || f.references.length > 0) && (
          <section className="fd-section fd-prose">
            {f.impact && <><h3 className="h3">Impact</h3><p>{f.impact}</p></>}
            {f.recommendation && <><h3 className="h3">Fix</h3><p>{f.recommendation}</p></>}
            {f.references.length > 0 && (
              <>
                <h3 className="h3">References</h3>
                <ul className="refs mono">{f.references.map(r => <li key={r}>{r}</li>)}</ul>
              </>
            )}
          </section>
        )}
      </div>
    </section>
  )
}

function GateRow({ n, g }: { n: number; g: GateStep }) {
  const Icon = g.state === 'passed' ? Check : g.state === 'failed' ? X : Circle
  return (
    <li className={`gate-row gate-${g.state}`} style={{ '--i': n } as React.CSSProperties}>
      <span className="gate-n mono">{n}</span>
      <span className="gate-q">{g.question}</span>
      <span className="gate-a">{g.answer}</span>
      <span className="gate-state">
        <Icon size={12} weight="bold" aria-hidden="true" />
        {g.state === 'passed' ? 'passed' : g.state === 'failed' ? 'failed' : 'not established'}
      </span>
    </li>
  )
}

/* The tester's call. Stored in the core's finding status, reversible with Reopen. */
function DecisionBar({ f }: { f: UiFinding }) {
  const app = useApp()
  const toast = useToast()
  const [busy, setBusy] = useState<CoreStatus | null>(null)
  const [error, setError] = useState('')

  const decide = async (status: CoreStatus, done: string) => {
    if (!app.selected) return
    setBusy(status)
    setError('')
    try {
      await api.updateFindingStatus(app.selected.id, f.id, status)
      await app.refreshFindings()
      toast(done)
    } catch (e) {
      setError(String(e).replace(/^Error:[ ]*/, ''))
    } finally {
      setBusy(null)
    }
  }

  if (f.decision) {
    return (
      <div className="decision decision-made">
        <span>{f.decision === 'false-positive' ? 'You marked this as a false positive. It stays out of reports.' : 'You accepted this risk. It is reported as accepted.'}</span>
        <button type="button" className="btn btn-quiet btn-sm" disabled={!!busy} onClick={() => decide('Draft', 'Finding reopened')}>
          <ArrowCounterClockwise size={14} aria-hidden="true" /> {busy ? 'Reopening' : 'Reopen'}
        </button>
        {error && <p className="help help-bad decision-error" role="alert">Not saved. {error}</p>}
      </div>
    )
  }
  if (f.status === 'Informational') return null

  const confirmed = f.status === 'Confirmed'
  return (
    <div className="decision">
      <span className="decision-label">{confirmed ? 'Proven by the engine.' : 'Your call'}</span>
      <div className="btn-row">
        {!confirmed && (
          <button type="button" className="btn btn-primary btn-sm" disabled={!!busy} onClick={() => decide('Verified', 'Marked as confirmed')}>
            <SealCheck size={14} aria-hidden="true" /> {busy === 'Verified' ? 'Saving' : 'I verified it'}
          </button>
        )}
        <button type="button" className="btn btn-ghost btn-sm" disabled={!!busy} onClick={() => decide('False Positive', 'Marked as false positive')}>
          <ShieldSlash size={14} aria-hidden="true" /> {busy === 'False Positive' ? 'Saving' : 'False positive'}
        </button>
        <button type="button" className="btn btn-quiet btn-sm" disabled={!!busy} onClick={() => decide('Accepted Risk', 'Risk accepted')}>
          {busy === 'Accepted Risk' ? 'Saving' : 'Accept risk'}
        </button>
      </div>
      {error && <p className="help help-bad decision-error" role="alert">Not saved. {error}</p>}
    </div>
  )
}
