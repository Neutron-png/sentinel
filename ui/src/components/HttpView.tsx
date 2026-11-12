/* Raw HTTP rendered as a document: start line, headers, body.
   Optional `delta` highlights the substring that differs from the baseline. */
export function HttpView({ text, delta, empty }: { text: string; delta?: string; empty?: string }) {
  if (!text) return <pre className="http http-empty">{empty ?? '(empty)'}</pre>
  const norm = text.replace(/\r\n/g, '\n')
  const split = norm.indexOf('\n\n')
  const head = split === -1 ? norm : norm.slice(0, split)
  const body = split === -1 ? '' : norm.slice(split + 2)
  const [start, ...headers] = head.split('\n')

  return (
    <pre className="http">
      <span className="http-start">{mark(start, delta)}</span>
      {headers.map((h, i) => {
        const c = h.indexOf(':')
        if (c === -1) return <span key={i}>{'\n'}{mark(h, delta)}</span>
        return (
          <span key={i}>
            {'\n'}<span className="http-name">{h.slice(0, c)}</span>
            <span className="http-value">{mark(h.slice(c), delta)}</span>
          </span>
        )
      })}
      {body && <>{'\n\n'}<span className="http-body">{mark(body, delta)}</span></>}
    </pre>
  )
}

function mark(s: string, delta?: string): React.ReactNode {
  if (!delta) return s
  const i = s.indexOf(delta)
  if (i === -1) return s
  return (
    <>
      {s.slice(0, i)}
      <mark className="delta">{delta}</mark>
      {s.slice(i + delta.length)}
    </>
  )
}
