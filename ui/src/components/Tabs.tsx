import { useLayoutEffect, useRef } from 'react'

/* Sliding segmented control (transitions.dev "tabs sliding").
   The pill is positioned without a transition on first paint and on resize,
   so it never animates in from 0. */
export function Tabs<T extends string>({
  value,
  options,
  onChange,
  label,
}: {
  value: T
  options: { id: T; label: string; count?: number }[]
  onChange: (v: T) => void
  label: string
}) {
  const bar = useRef<HTMLDivElement>(null)
  const pill = useRef<HTMLSpanElement>(null)
  const first = useRef(true)

  useLayoutEffect(() => {
    const place = (animate: boolean) => {
      const b = bar.current
      const p = pill.current
      const active = b?.querySelector<HTMLButtonElement>('[aria-selected="true"]')
      if (!b || !p || !active) return
      if (!animate) p.style.transition = 'none'
      p.style.width = `${active.offsetWidth}px`
      p.style.transform = `translateX(${active.offsetLeft - 2}px)`
      if (!animate) {
        void p.offsetWidth
        p.style.transition = ''
      }
    }
    place(!first.current)
    first.current = false
    const ro = new ResizeObserver(() => place(false))
    if (bar.current) ro.observe(bar.current)
    return () => ro.disconnect()
  }, [value, options.length])

  return (
    <div className="t-tabs" role="tablist" aria-label={label} ref={bar}>
      <span className="t-tabs-pill" ref={pill} aria-hidden="true" />
      {options.map(o => (
        <button
          key={o.id}
          role="tab"
          type="button"
          className="t-tab"
          aria-selected={o.id === value}
          onClick={() => onChange(o.id)}
        >
          {o.label}
          {o.count !== undefined && <span className="t-tab-count">{o.count}</span>}
        </button>
      ))}
    </div>
  )
}
