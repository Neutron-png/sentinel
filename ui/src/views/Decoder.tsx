import { useMemo, useState } from 'react'
import { ArrowDown, Copy, Plus, X } from '@phosphor-icons/react'
import { useToast } from '../components/Toast'

type Op =
  | 'url-decode' | 'url-encode' | 'base64-decode' | 'base64-encode'
  | 'hex-decode' | 'hex-encode' | 'html-decode' | 'html-encode'
  | 'unicode-unescape' | 'json-format' | 'json-minify' | 'jwt-inspect'

const OPS: { id: Op; label: string }[] = [
  { id: 'url-decode', label: 'URL decode' },
  { id: 'url-encode', label: 'URL encode' },
  { id: 'base64-decode', label: 'Base64 decode' },
  { id: 'base64-encode', label: 'Base64 encode' },
  { id: 'hex-decode', label: 'Hex decode' },
  { id: 'hex-encode', label: 'Hex encode' },
  { id: 'html-decode', label: 'HTML decode' },
  { id: 'html-encode', label: 'HTML encode' },
  { id: 'unicode-unescape', label: 'Unicode unescape' },
  { id: 'json-format', label: 'JSON format' },
  { id: 'json-minify', label: 'JSON minify' },
  { id: 'jwt-inspect', label: 'JWT inspect' },
]

const utf8 = { enc: new TextEncoder(), dec: new TextDecoder('utf-8', { fatal: false }) }
const b64d = (s: string) => {
  const n = s.trim().replace(/-/g, '+').replace(/_/g, '/')
  const bin = atob(n + '='.repeat((4 - (n.length % 4)) % 4))
  return utf8.dec.decode(Uint8Array.from(bin, c => c.charCodeAt(0)))
}

function apply(op: Op, s: string): string {
  switch (op) {
    case 'url-decode': return decodeURIComponent(s.replace(/\+/g, ' '))
    case 'url-encode': return encodeURIComponent(s)
    case 'base64-decode': return b64d(s)
    case 'base64-encode': return btoa(String.fromCharCode(...utf8.enc.encode(s)))
    case 'hex-decode': {
      const h = s.replace(/[^0-9a-f]/gi, '')
      if (h.length % 2) throw new Error('Odd number of hex digits.')
      return utf8.dec.decode(Uint8Array.from(h.match(/../g) ?? [], b => parseInt(b, 16)))
    }
    case 'hex-encode': return [...utf8.enc.encode(s)].map(b => b.toString(16).padStart(2, '0')).join('')
    case 'html-decode': { const t = document.createElement('textarea'); t.innerHTML = s; return t.value }
    case 'html-encode': return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#39;')
    case 'unicode-unescape': return s.replace(/\\u([0-9a-f]{4})/gi, (_, h) => String.fromCharCode(parseInt(h, 16)))
    case 'json-format': return JSON.stringify(JSON.parse(s), null, 2)
    case 'json-minify': return JSON.stringify(JSON.parse(s))
    case 'jwt-inspect': {
      const parts = s.trim().split('.')
      if (parts.length < 2) throw new Error('A JWT has at least two dot-separated parts.')
      const header = JSON.parse(b64d(parts[0]))
      const payload = JSON.parse(b64d(parts[1]))
      const notes: string[] = []
      if (String(header.alg).toLowerCase() === 'none') notes.push('alg is "none": the signature is not checked.')
      if (payload.exp && payload.exp * 1000 < Date.now()) notes.push(`Expired ${new Date(payload.exp * 1000).toISOString()}.`)
      if (!parts[2]) notes.push('No signature part.')
      return JSON.stringify({ header, payload, notes }, null, 2)
    }
  }
}

const SAMPLE = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJvcGVyYXRvci0xMDQ0MiIsInJvbGUiOiJ2aWV3ZXIiLCJleHAiOjE3OTAwMDAwMDB9.c2lnbmF0dXJl'

export function Decoder() {
  const toast = useToast()
  const [input, setInput] = useState(SAMPLE)
  const [chain, setChain] = useState<Op[]>(['jwt-inspect'])
  const [adding, setAdding] = useState<Op>('url-decode')

  const steps = useMemo(() => {
    let cur = input
    let failed = false
    return chain.map(op => {
      if (failed) return { op, out: '', error: 'Skipped, a previous step failed.' }
      try { cur = apply(op, cur); return { op, out: cur, error: '' } } catch (e) {
        failed = true
        return { op, out: '', error: e instanceof Error ? e.message : String(e) }
      }
    })
  }, [input, chain])

  return (
    <div className="decoder">
      <div className="view-head">
        <h1 className="h1">Decoder</h1>
        <p className="note">Transforms run top to bottom. Each step reads the output of the one above it. Nothing leaves this machine.</p>
      </div>

      <div className="chain">
        <div className="chain-step">
          <label className="chain-label" htmlFor="dec-in">Input</label>
          <textarea id="dec-in" className="editor mono chain-io" value={input} onChange={e => setInput(e.target.value)} spellCheck={false} />
        </div>

        {steps.map((s, i) => (
          <div className="chain-step" key={i}>
            <div className="chain-arrow" aria-hidden="true"><ArrowDown size={14} /></div>
            <div className="chain-label">
              <span>{OPS.find(o => o.id === s.op)?.label}</span>
              <span className="push-end btn-row">
                {s.out && (
                  <button type="button" className="btn-icon" aria-label="Copy output"
                    onClick={() => navigator.clipboard?.writeText(s.out).then(() => toast('Output copied'), () => {})}>
                    <Copy size={14} />
                  </button>
                )}
                <button type="button" className="btn-icon" aria-label="Remove step" onClick={() => setChain(c => c.filter((_, j) => j !== i))}>
                  <X size={14} />
                </button>
              </span>
            </div>
            {s.error
              ? <p className="inline-error" role="alert">{s.error}</p>
              : <pre className="http chain-io chain-out">{s.out || '(empty)'}</pre>}
          </div>
        ))}

        {chain.length === 0 && <p className="note chain-empty">Add a step to transform the input.</p>}
        {!input.trim() && chain.length > 0 && <p className="note chain-empty">Paste a value into Input to see each step.</p>}
        <div className="chain-add">
          <label className="sr-only" htmlFor="dec-op">Transform</label>
          <select id="dec-op" value={adding} onChange={e => setAdding(e.target.value as Op)}>
            {OPS.map(o => <option key={o.id} value={o.id}>{o.label}</option>)}
          </select>
          <button type="button" className="btn btn-ghost btn-sm" onClick={() => setChain(c => [...c, adding])}>
            <Plus size={14} aria-hidden="true" /> Add step
          </button>
        </div>
      </div>
    </div>
  )
}
