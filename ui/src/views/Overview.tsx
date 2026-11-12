import { useState } from 'react'
import { Check, Copy, Plus, Trash } from '@phosphor-icons/react'
import type { Assessment } from '../lib/types'
import { Modal } from '../components/Modal'
import * as api from '../lib/bridge'
import { hostnameOf, inScope, isProven, needsDecision, relative } from '../lib/format'
import type { View } from '../lib/types'
import { useApp } from '../state'
import { Lattice } from '../components/Marks'
import { useToast } from '../components/Toast'

/* The project home is the journey: scope, capture, test, decide, report.
   The first unfinished step carries the only primary action on the screen. */

interface Step {
  id: string
  title: string
  done: boolean
  detail: React.ReactNode
  action?: { label: string; view: View }
  extra?: React.ReactNode
}

export function Overview() {
  const app = useApp()
  const toast = useToast()
  const { assessments, selected, findings, traffic } = app
  const [toDelete, setToDelete] = useState<Assessment | null>(null)
  const [deleting, setDeleting] = useState(false)
  const [deleteError, setDeleteError] = useState('')

  if (assessments.length === 0 && app.findingsLoading) return null
  if (!selected) {
    return (
      <div className="empty-page">
        <Lattice />
        <h1 className="h1">Start with a target</h1>
        <p className="lead">A project holds one target and its scope. Nothing outside scope is ever sent.</p>
        <button type="button" className="btn btn-primary" onClick={app.openNewProject}>
          <Plus size={16} aria-hidden="true" /> New project
        </button>
      </div>
    )
  }

  // Proxy history is shared across projects; only in-scope traffic counts toward this one.
  const host = hostnameOf(selected.target) || selected.target
  const scoped = traffic.filter(t => inScope(t.host, host))
  const outside = traffic.length - scoped.length
  const hosts = new Set(scoped.map(t => t.host)).size
  const port = app.proxy?.port ?? 8080
  const proxyDown = !!app.proxy && !app.proxy.listening
  const undecided = findings.filter(needsDecision).length
  const proven = findings.filter(isProven).length
  const detected = findings.filter(f => f.status === 'Detected' && !f.decision).length

  const steps: Step[] = [
    {
      id: 'scope',
      title: 'Scope',
      done: true,
      detail: <><span className="mono">{host}</span> and its subdomains. Engines skip everything else{outside > 0 ? <>, including {outside} captured request{outside === 1 ? '' : 's'} to other hosts</> : null}.</>,
    },
    {
      id: 'capture',
      title: 'Capture',
      done: scoped.length > 0,
      detail: proxyDown
        ? <span className="code-bad">The proxy is not running, so nothing can be captured. Check Settings.</span>
        : scoped.length > 0
          ? <>{scoped.length} in-scope request{scoped.length === 1 ? '' : 's'} from {hosts} host{hosts === 1 ? '' : 's'}.</>
          : <>Set your browser proxy to <span className="mono">127.0.0.1:{port}</span> and browse <span className="mono">{host}</span>.</>,
      action: proxyDown ? { label: 'Open settings', view: 'settings' } : { label: scoped.length > 0 ? 'Open traffic' : 'Open Proxy', view: 'proxy' },
      extra: scoped.length === 0 && !proxyDown && (
        <button type="button" className="btn btn-quiet btn-sm" onClick={() => {
          navigator.clipboard?.writeText(`127.0.0.1:${port}`).then(() => toast('Proxy address copied'), () => {})
        }}><Copy size={14} aria-hidden="true" /> Copy address</button>
      ),
    },
    {
      id: 'test',
      title: 'Test',
      done: findings.length > 0,
      detail: findings.length > 0
        ? <>{findings.length} result{findings.length === 1 ? '' : 's'} recorded.</>
        : <>Replay a request in Repeater, or scan the target.</>,
      action: { label: 'Start a scan', view: 'scanner' },
    },
    {
      id: 'decide',
      title: 'Decide',
      done: findings.length > 0 && undecided === 0,
      detail: findings.length === 0
        ? <>Results the engine cannot prove come to you.</>
        : undecided > 0
          ? <>{undecided} result{undecided === 1 ? '' : 's'} could not be proven automatically. Confirm, dismiss or accept each one.</>
          : <>Nothing is waiting on you.</>,
      action: { label: undecided > 0 ? `Review ${undecided}` : 'Open findings', view: 'findings' },
    },
    {
      id: 'report',
      title: 'Report',
      done: false,
      detail: proven > 0
        ? <>{proven} confirmed finding{proven === 1 ? '' : 's'} ready.{detected > 0 ? <> {detected} detected one{detected === 1 ? ' needs' : 's need'} verifying first.</> : null}</>
        : <>Only confirmed findings go into a report by default.</>,
      action: { label: 'Export report', view: 'reports' },
    },
  ]
  const current = steps.findIndex(s => !s.done)

  return (
    <div className="overview">
      <section className="journey-wrap" aria-labelledby="ov-title">
        <Lattice />
        <header className="journey-head">
          <h1 className="h1" id="ov-title">{selected.name}</h1>
          <p className="mono dim">{selected.target}</p>
        </header>

        <ol className="journey">
          {steps.map((s, i) => {
            const state = s.done ? 'done' : i === current ? 'now' : 'later'
            return (
              <li key={s.id} className={`jstep jstep-${state}`} aria-current={state === 'now' ? 'step' : undefined}>
                <span className="jmark" aria-hidden="true">{s.done ? <Check size={12} weight="bold" /> : i + 1}</span>
                <div className="jbody">
                  <h2 className="jtitle">{s.title}{state === 'now' && <span className="jnext">Next</span>}</h2>
                  <p className="jdetail">{s.detail}</p>
                </div>
                {s.action && (
                  <div className="jactions">
                    {s.extra}
                    <button
                      type="button"
                      className={'btn btn-sm ' + (state === 'now' ? 'btn-primary' : 'btn-quiet')}
                      onClick={() => app.go(s.action!.view)}
                    >
                      {s.action.label}
                    </button>
                  </div>
                )}
              </li>
            )
          })}
        </ol>
      </section>

      <aside className="ov-side" aria-labelledby="ov-projects">
        <div className="side-head">
          <h2 className="h3" id="ov-projects">Projects</h2>
          <button type="button" className="btn-icon" aria-label="New project" onClick={app.openNewProject}><Plus size={16} /></button>
        </div>
        <ul className="project-list">
          {assessments.map(a => (
            <li key={a.id}>
              <button
                type="button"
                className="project-row"
                aria-current={a.id === selected.id ? 'true' : undefined}
                onClick={() => app.select(a.id)}
              >
                <span className="project-name">{a.name}</span>
                <span className="project-meta">{a.environment}, updated {relative(a.updated_at)}</span>
              </button>
              <button
                type="button"
                className="btn-icon project-delete"
                aria-label={`Delete ${a.name}`}
                onClick={() => { setDeleteError(''); setToDelete(a) }}
              >
                <Trash size={14} />
              </button>
            </li>
          ))}
        </ul>
      </aside>

      <Modal
        open={!!toDelete}
        onClose={() => !deleting && setToDelete(null)}
        title="Delete this project?"
        footer={<>
          <button type="button" className="btn btn-ghost" onClick={() => setToDelete(null)} disabled={deleting}>Keep it</button>
          <button type="button" className="btn btn-danger" disabled={deleting} onClick={async () => {
            if (!toDelete) return
            setDeleting(true)
            setDeleteError('')
            try {
              await api.deleteAssessment(toDelete.id)
              await app.refreshAssessments()
              toast(`Deleted "${toDelete.name}"`)
              setToDelete(null)
            } catch (e) { setDeleteError(String(e).replace(/^Error:[ ]*/, '')) } finally { setDeleting(false) }
          }}>{deleting ? 'Deleting' : 'Delete project'}</button>
        </>}
      >
        <p className="modal-text"><strong>{toDelete?.name}</strong> and all of its findings will be removed. This cannot be undone. Captured proxy history is shared and stays.</p>
        {deleteError && <p className="inline-error inline-error-flush" role="alert"><strong>Not deleted.</strong> {deleteError}</p>}
      </Modal>
    </div>
  )
}
