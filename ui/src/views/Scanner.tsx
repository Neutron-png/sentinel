import { useState } from 'react'
import { Play } from '@phosphor-icons/react'
import * as api from '../lib/bridge'
import { asSeverity, asStatus, hostnameOf, inScope } from '../lib/format'
import type { ScanFinding } from '../lib/types'
import { useApp } from '../state'
import { Lattice, SeverityTag, StatusMark } from '../components/Marks'
import { useToast } from '../components/Toast'

const STAGES = ['baseline', 'mutate', 'observe', 'compare', 'verify'] as const
const RULES = [
  { id: 'sqli', name: 'SQL injection', note: 'Error, boolean, union and time-based. Time needs absolute and relative delay.' },
  { id: 'xss', name: 'Cross-site scripting', note: 'Only verbatim, unescaped reflections in an HTML context count.' },
  { id: 'path-traversal', name: 'Path traversal', note: 'Requires file content, not only a status change.' },
]

interface RuleRun { stage: string; done: boolean; found: number }

export function Scanner() {
  const app = useApp()
  const toast = useToast()
  const [target, setTarget] = useState('')
  const [runs, setRuns] = useState<Record<string, RuleRun>>({})
  const [results, setResults] = useState<ScanFinding[]>([])
  const [running, setRunning] = useState(false)
  const [error, setError] = useState('')
  const [finished, setFinished] = useState(false)
  const [cancelling, setCancelling] = useState(false)

  const effectiveTarget = target.trim() || app.selected?.target || ''
  const scopeHost = app.selected ? hostnameOf(app.selected.target) : ''
  const targetHost = (() => { try { return new URL(effectiveTarget).hostname.toLowerCase() } catch { return '' } })()
  const targetInvalid = !!target.trim() && targetHost === ''
  const noParams = !!targetHost && !/[?&][^=&]+=/.test(effectiveTarget)
  // Domain-boundary match: xportal.example.test is NOT in scope for portal.example.test.
  const outOfScope = !!effectiveTarget && !!scopeHost && targetHost !== '' && !inScope(targetHost, scopeHost)

  const start = async () => {
    if (!app.selected || !effectiveTarget || outOfScope || targetInvalid) return
    setRunning(true)
    setError('')
    setFinished(false)
    setCancelling(false)
    setRuns({})
    setResults([])
    try {
      await api.startScan(app.selected.id, effectiveTarget, {
        onProgress: p => setRuns(r => ({
          ...r,
          [p.rule_id]: { stage: p.stage, done: p.done, found: r[p.rule_id]?.found ?? 0 },
        })),
        onFinding: f => {
          setResults(x => [...x, f])
          setRuns(r => ({ ...r, [f.rule_id]: { ...(r[f.rule_id] ?? { stage: 'verify', done: false }), found: (r[f.rule_id]?.found ?? 0) + 1 } }))
        },
      })
      toast('Scan finished')
      setFinished(true)
      app.refreshFindings()
    } catch (e) {
      setError(String(e).replace(/^Error:\s*/, ''))
    } finally {
      setRunning(false)
      setCancelling(false)
    }
  }

  return (
    <div className="split split-scanner">
      <section className="pane pane-form" aria-labelledby="scan-h">
        <div className="pane-pad">
          <h1 className="h2" id="scan-h">Active scan</h1>
          <p className="note">Each rule runs baseline, mutate, observe, compare, verify. A result that cannot be verified is reported as Potential, not as a vulnerability.</p>

          <div className="field">
            <label htmlFor="scan-target">Target URL</label>
            <input
              id="scan-target"
              className="mono"
              value={target}
              onChange={e => setTarget(e.target.value)}
              placeholder={app.selected?.target ?? 'https://'}
              aria-describedby="scan-target-help"
              aria-invalid={outOfScope || targetInvalid}
              spellCheck={false}
            />
            <p id="scan-target-help" className={'help' + (outOfScope || targetInvalid ? ' help-bad' : '')}>
              {targetInvalid
                ? 'Enter a full URL with http:// or https://.'
                : outOfScope
                  ? `${targetHost} is outside this project's scope (${scopeHost}). Nothing will be sent.`
                  : noParams
                    ? 'This URL has no query parameters, so there is little to test. Add one, for example ?id=1.'
                    : 'Leave empty to scan the project target. Each query parameter is tested.'}
            </p>
          </div>

          <div className="field">
            <span className="field-label">Rules in this scan</span>
            <ul className="rule-list">
              {RULES.map(r => (
                <li key={r.id}>
                  <span className="check-title">{r.name}</span>
                  <span className="check-note">{r.note}</span>
                </li>
              ))}
            </ul>
          </div>

          <button
            type="button"
            className="btn btn-primary"
            disabled={!app.selected || !effectiveTarget || running || outOfScope || targetInvalid}
            onClick={start}
          >
            <Play size={14} aria-hidden="true" /> {running ? 'Scanning' : 'Start scan'}
          </button>
          {running && (
            <button type="button" className="btn btn-ghost" disabled={cancelling} onClick={async () => {
              setCancelling(true)
              try { await api.cancelScan() } catch (e) { setError(String(e).replace(/^Error:[ ]*/, '')) }
            }}>{cancelling ? 'Stopping after this request' : 'Cancel scan'}</button>
          )}
          {!app.selected && <p className="help">Create a project first. A scan needs a scope.</p>}
          {running && <p className="help">You can leave this page. The scan keeps running, and results land in Findings.</p>}
        </div>
      </section>

      <section className="pane" aria-label="Scan progress and results" aria-busy={running}>
        <div className="pane-bar"><h2 className="pane-title">Pipeline</h2></div>
        <div className="scroll">
          <div className="scan-stage">
          <Lattice scanning={running} />
          <ol className="pipeline">
            {RULES.map(r => {
              const run = runs[r.id]
              const at = run ? (run.done ? STAGES.length : Math.max(0, STAGES.indexOf(run.stage as typeof STAGES[number]))) : -1
              const indeterminate = run && !run.done && run.stage === 'running'
              return (
                <li key={r.id} className={'pipe-row' + (indeterminate ? ' is-running' : '')}>
                  <span className="pipe-name">{r.name}</span>
                  <span className="pipe-stages" aria-label={run ? (run.done ? 'complete' : `stage ${run.stage}`) : 'waiting'}>
                    {STAGES.map((s, i) => (
                      <span key={s} className={'pipe-seg' + (i < at || (i === at && !run?.done) ? ' is-on' : '') + (i === at && !run?.done ? ' is-now' : '')}>
                        <span className="pipe-label">{s}</span>
                      </span>
                    ))}
                  </span>
                  <span className="mono pipe-count">
                    {run?.done ? `${run.found} result${run.found === 1 ? '' : 's'}` : run ? 'running' : 'waiting'}
                  </span>
                </li>
              )
            })}
          </ol>
          </div>

          <h3 className="h3 pad">Results</h3>
          {error ? (
            <p className="inline-error" role="alert"><strong>Scan stopped.</strong> {error}</p>
          ) : results.length === 0 ? (
            <p className="note pad">
              {running ? 'Results appear here as each rule finishes.'
                : finished ? 'Nothing could be shown to be vulnerable. That is a valid result, not a failure.'
                : 'No scan has run yet.'}
            </p>
          ) : (
            <ul className="result-list">
              {results.map((f, i) => (
                <li key={i} className="result-row">
                  <StatusMark status={asStatus(f.status, f.confidence)} label />
                  <span className="result-title">{f.title}</span>
                  <SeverityTag severity={asSeverity(f.severity)} />
                  <span className="mono dim result-ev">{f.evidence}</span>
                </li>
              ))}
            </ul>
          )}
          {finished && results.length > 0 && (
            <div className="pad scan-next">
              <button type="button" className="btn btn-primary btn-sm" onClick={() => app.go('findings')}>Review in Findings</button>
              <span className="note">Confirmed results are proven. The rest wait for your decision.</span>
            </div>
          )}
        </div>
      </section>
    </div>
  )
}
