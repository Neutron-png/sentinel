import { useEffect, useRef, useState } from 'react'

/* transitions.dev modal: 250ms scale 0.96 -> 1 open, 150ms close.
   The element stays mounted through the close so the exit can play. */
export function Modal({
  open,
  onClose,
  title,
  children,
  footer,
}: {
  open: boolean
  onClose: () => void
  title: string
  children: React.ReactNode
  footer?: React.ReactNode
}) {
  const [mounted, setMounted] = useState(open)
  const [phase, setPhase] = useState<'pre' | 'open' | 'closing'>('pre')
  const panel = useRef<HTMLDivElement>(null)
  const lastFocus = useRef<HTMLElement | null>(null)

  useEffect(() => {
    if (open) {
      lastFocus.current = document.activeElement as HTMLElement
      setMounted(true)
      setPhase('pre')
      const r = requestAnimationFrame(() => requestAnimationFrame(() => setPhase('open')))
      return () => cancelAnimationFrame(r)
    }
    if (!mounted) return
    setPhase('closing')
    const t = setTimeout(() => {
      setMounted(false)
      lastFocus.current?.focus()
    }, 150)
    return () => clearTimeout(t)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open])

  useEffect(() => {
    if (phase === 'open') panel.current?.querySelector<HTMLElement>('input, select, textarea, button')?.focus()
  }, [phase])

  useEffect(() => {
    if (!mounted) return
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') onClose() }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [mounted, onClose])

  if (!mounted) return null
  const cls = phase === 'open' ? ' is-open' : phase === 'closing' ? ' is-closing' : ''
  return (
    <div className={'scrim' + cls} onMouseDown={onClose}>
      <div
        ref={panel}
        className={'t-modal floating' + cls}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
        onMouseDown={e => e.stopPropagation()}
      >
        <h2 id="modal-title" className="modal-title">{title}</h2>
        <div className="modal-body">{children}</div>
        {footer && <div className="modal-footer">{footer}</div>}
      </div>
    </div>
  )
}
