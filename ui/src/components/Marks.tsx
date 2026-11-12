import { CircleDashed, CircleHalf, Info, Question, SealCheck } from '@phosphor-icons/react'
import type { Icon } from '@phosphor-icons/react'
import type { Severity, VerificationStatus } from '../lib/types'
import { slug, statusClass } from '../lib/format'

/* Each status has its own Phosphor glyph, so it reads in grayscale and for color-vision deficiency. */
const STATUS_ICON: Record<VerificationStatus, { icon: Icon; weight: 'fill' | 'regular' | 'bold' }> = {
  Confirmed: { icon: SealCheck, weight: 'fill' },
  Detected: { icon: CircleHalf, weight: 'fill' },
  'Needs Manual Verification': { icon: Question, weight: 'regular' },
  Potential: { icon: CircleDashed, weight: 'bold' },
  Informational: { icon: Info, weight: 'regular' },
}

export function StatusMark({ status, label = false }: { status: VerificationStatus; label?: boolean }) {
  const { icon: I, weight } = STATUS_ICON[status]
  return (
    <span className="status" title={label ? undefined : status}>
      <I size={15} weight={weight} className={`status-icon si-${slug(status)}`} aria-hidden="true" />
      {label ? <span className="status-label">{status}</span> : <span className="sr-only">{status}</span>}
    </span>
  )
}

/** Severity is a short bar plus the word, never color alone. */
export function SeverityTag({ severity, compact = false }: { severity: Severity; compact?: boolean }) {
  return (
    <span className={`sev sev-${slug(severity)}`}>
      <span className="sev-bar" aria-hidden="true" />
      {compact && severity === 'Informational' ? 'Info' : severity}
    </span>
  )
}

export function Method({ m }: { m: string }) {
  return <span className={`method method-${m.toLowerCase()}`}>{m}</span>
}

export function Code({ code }: { code: number }) {
  return <span className={`code ${statusClass(code)}`}>{code || '-'}</span>
}

/** The project logo (assets/logo.svg): hexagon + block S, same geometry and gradients.
    The glow filter is dropped at UI sizes, where it only blurs the edges. */
export function Logo({ size = 22, flat = false }: { size?: number; flat?: boolean }) {
  return (
    <svg width={size} height={size} viewBox="0 0 140 140" aria-hidden="true" className="logo">
      {!flat && (
        <defs>
          <linearGradient id="lg-top" x1="0" y1="0" x2="1" y2="1">
            <stop offset="0" stopColor="#3b1fb0" /><stop offset="0.55" stopColor="#7a3cf0" /><stop offset="1" stopColor="#c05cf5" />
          </linearGradient>
          <linearGradient id="lg-bottom" x1="0" y1="1" x2="1" y2="0">
            <stop offset="0" stopColor="#4b2bd6" /><stop offset="0.5" stopColor="#8b46f5" /><stop offset="1" stopColor="#d06bf7" />
          </linearGradient>
        </defs>
      )}
      <g transform="translate(70,70)">
        <path d="M0 -54 L46 -27 L46 27 L0 54 L-46 27 L-46 -27 Z" fill="none"
          stroke={flat ? 'currentColor' : 'url(#lg-top)'} strokeWidth="9" strokeLinejoin="round" />
        <path d="M-23 -25 L23 -25 L23 -11 L-6 -11 L-6 -3 L23 -3 L23 25 L-23 25 L-23 11 L6 11 L6 3 L-23 3 Z"
          fill={flat ? 'currentColor' : 'url(#lg-bottom)'} />
      </g>
    </svg>
  )
}

/** Hex lattice built from the logo's hexagon. Static texture behind empty canvases;
    `scanning` runs a slow sweep (CSS only, transform/opacity, off under reduced motion). */
export function Lattice({ scanning = false }: { scanning?: boolean }) {
  return (
    <div className={'lattice' + (scanning ? ' is-scanning' : '')} aria-hidden="true">
      {scanning && <span className="lattice-sweep" />}
    </div>
  )
}

export function Kbd({ children }: { children: React.ReactNode }) {
  return <kbd className="kbd">{children}</kbd>
}
