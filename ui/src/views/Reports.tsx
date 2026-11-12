import { useMemo, useState } from 'react'
import { DownloadSimple } from '@phosphor-icons/react'
import { SEVERITY_ORDER, STATUS_ORDER } from '../lib/types'
import type { Assessment, UiFinding, VerificationStatus } from '../lib/types'
import { slug } from '../lib/format'
import { useApp } from '../state'
import { StatusMark } from '../components/Marks'
import { Tabs } from '../components/Tabs'

type Format = 'markdown' | 'json'

export function Reports() {
  const app = useApp()
  const [format, setFormat] = useState<Format>('markdown')
  const [include, setInclude] = useState<Record<VerificationStatus, boolean>>({
    Confirmed: true, Detected: false, 'Needs Manual Verification': false, Potential: false, Informational: false,
  })

  const picked = useMemo(() => app.findings
    // False positives never reach a report, whatever is ticked.
    .filter(f => include[f.status] && f.decision !== 'false-positive')
    .sort((a, b) => SEVERITY_ORDER.indexOf(a.severity) - SEVERITY_ORDER.indexOf(b.severity)), [app.findings, include])

  const text = app.selected ? (format === 'markdown' ? toMarkdown(app.selected, picked) : toJson(app.selected, picked)) : ''

  const download = () => {
    if (!app.selected) return
    const blob = new Blob([text], { type: format === 'markdown' ? 'text/markdown' : 'application/json' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = `${slug(app.selected.name)}-report.${format === 'markdown' ? 'md' : 'json'}`
    a.click()
    setTimeout(() => URL.revokeObjectURL(a.href), 1000)
  }

  if (!app.selected && app.findingsLoading) return null
  if (!app.selected) {
    return <div className="empty-page"><h1 className="h1">No project yet</h1><p className="lead">A report is built from one project's findings.</p><button type="button" className="btn btn-primary" onClick={app.openNewProject}>New project</button></div>
  }
  if (!app.findingsLoading && app.findings.length === 0) {
    return <div className="empty-page"><h1 className="h1">Nothing to report yet</h1><p className="lead">Reports contain proven findings. Test the target first.</p><button type="button" className="btn btn-primary" onClick={() => app.go('scanner')}>Start a scan</button></div>
  }

  return (
    <div className="split split-scanner">
      <section className="pane pane-form" aria-labelledby="rep-h">
        <div className="pane-pad">
          <h1 className="h2" id="rep-h">Report</h1>
          <p className="note">Only confirmed findings are included by default. False positives are always left out.</p>

          <div className="field">
            <span className="field-label">Format</span>
            <Tabs label="Report format" value={format} onChange={setFormat} options={[
              { id: 'markdown', label: 'Markdown' },
              { id: 'json', label: 'JSON' },
            ]} />
            <p className="help">HTML and PDF export run from the command line: <code className="mono">sentinel report generate</code></p>
          </div>

          <fieldset className="field">
            <legend>Include</legend>
            {STATUS_ORDER.map(s => {
              const n = app.findings.filter(f => f.status === s && f.decision !== 'false-positive').length
              return (
                <label key={s} className="check-row">
                  <input type="checkbox" checked={include[s]} onChange={e => setInclude({ ...include, [s]: e.target.checked })} />
                  <StatusMark status={s} label />
                  <span className="mono dim push-end">{n}</span>
                </label>
              )
            })}
          </fieldset>

          <button type="button" className="btn btn-primary" onClick={download} disabled={picked.length === 0}>
            <DownloadSimple size={14} aria-hidden="true" /> Export {picked.length} finding{picked.length === 1 ? '' : 's'}
          </button>
        </div>
      </section>
      <section className="pane" aria-label="Report preview">
        <div className="pane-bar"><h2 className="pane-title">Preview</h2><span className="mono dim pane-sub">{text.length.toLocaleString()} characters</span></div>
        <div className="scroll">{picked.length === 0
          ? <div className="empty empty-quiet"><p>No finding matches the statuses you picked. Include at least one status to see a preview.</p></div>
          : <pre className="http report-preview">{text}</pre>}</div>
      </section>
    </div>
  )
}

function toMarkdown(a: Assessment, fs: UiFinding[]): string {
  const lines = [
    `# ${a.name}`,
    '',
    `Target: ${a.target}  `,
    `Environment: ${a.environment}  `,
    `Methodology: ${a.methodology}  `,
    `Generated: ${new Date().toISOString()}`,
    '',
    '## Summary',
    '',
    '| Severity | Status | Title |',
    '|---|---|---|',
    ...fs.map(f => `| ${f.severity} | ${f.status}${f.decision === 'accepted-risk' ? ' (risk accepted)' : ''} | ${f.title.replace(/\|/g, '\\|')} |`),
    '',
  ]
  for (const f of fs) {
    lines.push(`## ${f.title}`, '', `- Severity: ${f.severity}`, `- Verification status: ${f.status}`, `- Confidence: ${f.confidence}`)
    if (f.decision === 'accepted-risk') lines.push('- Decision: risk accepted by the tester')
    if (f.endpoint) lines.push(`- Endpoint: \`${f.endpoint}\``)
    if (f.parameter) lines.push(`- Parameter: \`${f.parameter}\``)
    lines.push('', f.description, '')
    if (f.gate) {
      lines.push('### Verification', '')
      f.gate.forEach((g, i) => lines.push(`${i + 1}. **${g.question}** ${g.answer} (${g.state === 'open' ? 'not established' : g.state})`))
      lines.push('')
    }
    if (f.reproduction) lines.push('### Reproduction', '', '```http', f.reproduction, '```', '')
    if (f.impact) lines.push('### Impact', '', f.impact, '')
    if (f.recommendation) lines.push('### Remediation', '', f.recommendation, '')
    if (f.references.length) lines.push('### References', '', ...f.references.map(r => `- ${r}`), '')
  }
  return lines.join('\n')
}

function toJson(a: Assessment, fs: UiFinding[]): string {
  return JSON.stringify({ project: { name: a.name, target: a.target, environment: a.environment, methodology: a.methodology }, generated_at: new Date().toISOString(), findings: fs }, null, 2)
}
