import { createContext, useCallback, useContext, useEffect, useRef, useState } from 'react'
import { X } from '@phosphor-icons/react'

type Tone = 'neutral' | 'bad'
interface Toast { id: number; text: string; tone: Tone; open: boolean }

const Ctx = createContext<(text: string, tone?: Tone) => void>(() => {})
export const useToast = () => useContext(Ctx)

/* transitions.dev toast: rises 16px with a 2px blur, 350ms in, 250ms out.
   Timers pause while the window is hidden so a toast is never missed. */
export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([])
  const seq = useRef(0)

  const dismiss = useCallback((id: number) => {
    setToasts(ts => ts.map(t => (t.id === id ? { ...t, open: false } : t)))
    setTimeout(() => setToasts(ts => ts.filter(t => t.id !== id)), 260)
  }, [])

  const push = useCallback((text: string, tone: Tone = 'neutral') => {
    const id = ++seq.current
    setToasts(ts => [...ts.slice(-2), { id, text, tone, open: false }])
    requestAnimationFrame(() => requestAnimationFrame(() =>
      setToasts(ts => ts.map(t => (t.id === id ? { ...t, open: true } : t)))))
  }, [])

  return (
    <Ctx.Provider value={push}>
      {children}
      <div className="toasts" role="status" aria-live="polite">
        {toasts.map(t => <ToastItem key={t.id} t={t} onDone={() => dismiss(t.id)} />)}
      </div>
    </Ctx.Provider>
  )
}

function ToastItem({ t, onDone }: { t: Toast; onDone: () => void }) {
  useEffect(() => {
    if (!t.open) return
    let left = t.tone === 'bad' ? 7000 : 3600
    let start = Date.now()
    let timer = setTimeout(onDone, left)
    const vis = () => {
      if (document.hidden) { clearTimeout(timer); left -= Date.now() - start }
      else { start = Date.now(); timer = setTimeout(onDone, left) }
    }
    document.addEventListener('visibilitychange', vis)
    return () => { clearTimeout(timer); document.removeEventListener('visibilitychange', vis) }
  }, [t.open, t.tone, onDone])

  return (
    <div className={`t-toast floating toast-${t.tone}` + (t.open ? ' is-open' : '')}>
      <span>{t.text}</span>
      <button type="button" className="btn-icon" aria-label="Dismiss" onClick={onDone}><X size={14} /></button>
    </div>
  )
}
