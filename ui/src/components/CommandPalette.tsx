import { useEffect, useMemo, useRef, useState } from 'react'
import { MagnifyingGlass } from '@phosphor-icons/react'
import { Kbd } from './Marks'

export interface Command {
  id: string
  label: string
  group: string
  hint?: string
  run: () => void
}

/* Opened hundreds of times a day, so it has no open/close animation (Emil's frequency rule). */
export function CommandPalette({ open, onClose, commands }: { open: boolean; onClose: () => void; commands: Command[] }) {
  const [q, setQ] = useState('')
  const [i, setI] = useState(0)
  const input = useRef<HTMLInputElement>(null)

  const list = useMemo(() => {
    const n = q.trim().toLowerCase()
    return n ? commands.filter(c => c.label.toLowerCase().includes(n) || c.group.toLowerCase().includes(n)) : commands
  }, [q, commands])

  useEffect(() => {
    if (open) { setQ(''); setI(0); input.current?.focus() }
  }, [open])
  useEffect(() => { setI(0) }, [q])

  if (!open) return null

  const run = (c?: Command) => { if (!c) return; onClose(); c.run() }

  return (
    <div className="scrim is-open palette-scrim" onMouseDown={onClose}>
      <div className="palette floating" role="dialog" aria-label="Command palette" onMouseDown={e => e.stopPropagation()}>
        <div className="palette-input">
          <MagnifyingGlass size={16} aria-hidden="true" />
          <input
            ref={input}
            value={q}
            onChange={e => setQ(e.target.value)}
            placeholder="Go to a view or run a command"
            aria-label="Command"
            aria-controls="palette-list"
            aria-activedescendant={list[i] ? `cmd-${list[i].id}` : undefined}
            onKeyDown={e => {
              if (e.key === 'ArrowDown') { e.preventDefault(); setI(x => Math.min(x + 1, list.length - 1)) }
              if (e.key === 'ArrowUp') { e.preventDefault(); setI(x => Math.max(x - 1, 0)) }
              if (e.key === 'Enter') { e.preventDefault(); run(list[i]) }
              if (e.key === 'Escape') onClose()
            }}
          />
          <Kbd>Esc</Kbd>
        </div>
        <ul className="palette-list" id="palette-list" role="listbox">
          {list.length === 0 && <li className="palette-empty">No command matches "{q}".</li>}
          {list.map((c, idx) => (
            <li
              key={c.id}
              id={`cmd-${c.id}`}
              role="option"
              aria-selected={idx === i}
              className="palette-item"
              onMouseMove={() => setI(idx)}
              onClick={() => run(c)}
            >
              <span className="palette-group">{c.group}</span>
              <span className="palette-label">{c.label}</span>
              {c.hint && <Kbd>{c.hint}</Kbd>}
            </li>
          ))}
        </ul>
      </div>
    </div>
  )
}
