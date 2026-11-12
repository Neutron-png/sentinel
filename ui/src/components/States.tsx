import { ArrowClockwise, WarningCircle } from '@phosphor-icons/react'
import { useState } from 'react'

/* Full-surface error: names what failed, shows the engine's message, offers a retry. */
export function ErrorState({ title, detail, onRetry }: { title: string; detail: string; onRetry?: () => Promise<void> | void }) {
  const [busy, setBusy] = useState(false)
  return (
    <div className="empty-page" role="alert">
      <WarningCircle size={28} className="state-icon-bad" aria-hidden="true" />
      <h1 className="h1">{title}</h1>
      <p className="lead">The desktop core returned an error. Your data has not changed.</p>
      <pre className="error-detail mono">{detail}</pre>
      {onRetry && (
        <button type="button" className="btn btn-primary" disabled={busy}
          onClick={async () => { setBusy(true); try { await onRetry() } finally { setBusy(false) } }}>
          <ArrowClockwise size={16} aria-hidden="true" /> {busy ? 'Trying again' : 'Try again'}
        </button>
      )}
    </div>
  )
}

/* Inline banner for a degraded but usable surface (proxy stopped, partial data). */
export function Banner({ tone = 'warn', children, action }: { tone?: 'warn' | 'bad'; children: React.ReactNode; action?: React.ReactNode }) {
  return (
    <div className={`banner banner-${tone}`} role={tone === 'bad' ? 'alert' : 'status'}>
      <WarningCircle size={16} aria-hidden="true" />
      <span className="banner-text">{children}</span>
      {action}
    </div>
  )
}
