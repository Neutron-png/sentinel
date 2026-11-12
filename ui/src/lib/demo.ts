/* Demo data, used only when the UI runs outside Tauri (browser preview, design review).
   Hosts use the reserved .test TLD so nothing here can be mistaken for a real target. */
import type { Assessment, GateStep, HistoryEntry, UiFinding } from './types'

const now = Date.now()
const ago = (min: number) => new Date(now - min * 60_000).toISOString()

export const DEMO_ASSESSMENTS: Assessment[] = [
  {
    id: 'demo-halden',
    name: 'Halden Freight customer portal',
    target: 'https://portal.halden-freight.test',
    environment: 'Staging',
    scope: 'Web Application',
    methodology: 'OWASP',
    status: 'Active',
    created_at: ago(60 * 26),
    updated_at: ago(12),
  },
  {
    id: 'demo-orbis',
    name: 'Orbis Pay partner API',
    target: 'https://api.orbis-pay.test/v2',
    environment: 'Staging',
    scope: 'API',
    methodology: 'OWASP',
    status: 'Active',
    created_at: ago(60 * 24 * 4),
    updated_at: ago(60 * 5),
  },
  {
    id: 'demo-kestrel',
    name: 'Kestrel Health intake forms',
    target: 'https://intake.kestrel-health.test',
    environment: 'Production',
    scope: 'Web Application',
    methodology: 'PTES',
    status: 'Draft',
    created_at: ago(60 * 24 * 9),
    updated_at: ago(60 * 24 * 2),
  },
]

export const GATE_QUESTIONS = [
  'What is the baseline?',
  'What exactly changed?',
  'Why does the change indicate a vulnerability?',
  'Can it be reproduced?',
  'Can it be independently verified?',
  'Is there a plausible benign explanation?',
  'What evidence supports the conclusion?',
  'What is the confidence?',
  'Can another tester reproduce it?',
]

function gate(answers: [string, GateStep['state']][]): GateStep[] {
  return GATE_QUESTIONS.map((question, i) => ({
    question,
    answer: answers[i]?.[0] ?? 'Not established.',
    state: answers[i]?.[1] ?? 'open',
  }))
}

const SQLI_BASELINE = `HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 412

{"order":{"id":"10442","status":"in_transit","eta":"2026-11-14"}}`

const SQLI_OBSERVED = `HTTP/1.1 500 Internal Server Error
Content-Type: application/json
Content-Length: 233

{"error":"ERROR: unterminated quoted string at or near \\"'10442''\\" LINE 1: SELECT * FROM orders WHERE id = '10442''"}`

export const DEMO_FINDINGS: Record<string, UiFinding[]> = {
  'demo-halden': [
    {
      id: 'f-sqli-orders',
      title: 'SQL injection in order lookup',
      severity: 'Critical',
      status: 'Confirmed',
      confidence: 'Confirmed',
      endpoint: 'GET /api/orders',
      parameter: 'id',
      rule: 'sqli.error-based',
      created_at: ago(38),
      description:
        'The id parameter is concatenated into a PostgreSQL query. A single quote produces a new database syntax error that is absent from the baseline response.',
      impact:
        'An attacker can read any row in the orders schema and, depending on role grants, other schemas in the same database.',
      recommendation:
        'Use parameterized queries for the order lookup. Reject non-numeric ids at the route before they reach the data layer.',
      references: ['CWE-89', 'OWASP WSTG-INPV-05'],
      baseline: SQLI_BASELINE,
      observed: SQLI_OBSERVED,
      delta: `ERROR: unterminated quoted string at or near \\"'10442''\\"`,
      reproduction: `GET /api/orders?id=10442' HTTP/1.1
Host: portal.halden-freight.test
Cookie: session=7f3c0b9e...
Accept: application/json`,
      gate: gate([
        ['200 OK, 412 bytes, JSON order body. Stable across 3 baseline requests.', 'passed'],
        ["Status 200 to 500. Body now contains a PostgreSQL syntax error naming the injected quote.", 'passed'],
        ['The error echoes the raw value inside a SQL string literal, so input reaches the query unescaped.', 'passed'],
        ['Reproduced 3 of 3 times with the same error.', 'passed'],
        ["Boolean pair id=10442' AND '1'='1 and AND '1'='2 returns the order and an empty result respectively.", 'passed'],
        ['No. The error is absent on the baseline and on a numeric control value (id=10443).', 'passed'],
        ['Baseline and mutated exchanges, error string, boolean pair, timing within normal range.', 'passed'],
        ['Confirmed. Two independent techniques agree.', 'passed'],
        ['Yes. Reproduction request attached; no session-specific state required beyond a valid login.', 'passed'],
      ]),
    },
    {
      id: 'f-xss-search',
      title: 'Reflected cross-site scripting in search',
      severity: 'High',
      status: 'Detected',
      confidence: 'High',
      endpoint: 'GET /search',
      parameter: 'q',
      rule: 'xss.reflected',
      created_at: ago(34),
      description:
        'The q parameter is reflected verbatim and unescaped into an HTML text context of a 200 text/html document.',
      impact: 'Script execution in the portal origin for any user who follows a crafted link.',
      recommendation: 'HTML-encode q on output. Add a Content-Security-Policy that disallows inline script.',
      references: ['CWE-79', 'OWASP WSTG-INPV-01'],
      baseline: `HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

<h2>Results for <span>pallet</span></h2>`,
      observed: `HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

<h2>Results for <span><svg onload=sn7x4()></span></h2>`,
      delta: '<svg onload=sn7x4()>',
      reproduction: `GET /search?q=%3Csvg%20onload%3Dsn7x4()%3E HTTP/1.1
Host: portal.halden-freight.test`,
      gate: gate([
        ['200 OK, text/html, the search term is reflected inside a span.', 'passed'],
        ['The payload is reflected byte for byte, with no HTML escaping.', 'passed'],
        ['An unescaped tag in an HTML text context is parsed as markup by the browser.', 'passed'],
        ['Reproduced 3 of 3 times.', 'passed'],
        ['Not yet. Execution in a real browser has not been observed.', 'open'],
        ['No. Escaped and partial reflections were ruled out by the benchmark controls.', 'passed'],
        ['Baseline and mutated exchanges, reflection offset 1184.', 'passed'],
        ['High. One technique, no independent confirmation.', 'open'],
        ['Yes, with the attached request.', 'passed'],
      ]),
    },
    {
      id: 'f-time-invoices',
      title: 'Possible time-based SQL injection in invoice sort',
      severity: 'High',
      status: 'Needs Manual Verification',
      confidence: 'Low',
      endpoint: 'GET /api/invoices',
      parameter: 'sort',
      rule: 'sqli.time-based',
      created_at: ago(31),
      description:
        'A sleep payload produced a 5.2 s delay once, but the baseline itself varies between 0.4 s and 3.9 s under load.',
      impact: 'If real, the same impact as the order lookup injection.',
      recommendation: 'Re-test outside peak load, or ask the client for a quiet window. Compare with a 10 s payload.',
      references: ['CWE-89'],
      baseline: `HTTP/1.1 200 OK
X-Response-Time: 412ms ... 3911ms (8 samples)`,
      observed: `HTTP/1.1 200 OK
X-Response-Time: 5214ms (1 of 3 samples)`,
      delta: '5214ms (1 of 3 samples)',
      reproduction: `GET /api/invoices?sort=created_at;SELECT+pg_sleep(5)-- HTTP/1.1
Host: portal.halden-freight.test`,
      gate: gate([
        ['Unstable. 8 samples between 412 ms and 3911 ms.', 'failed'],
        ['One of three mutated requests exceeded the 5 s threshold.', 'open'],
        ['A delay matching the injected sleep suggests the payload executed.', 'passed'],
        ['Reproduced 1 of 3 times.', 'failed'],
        ['Not attempted while the baseline is unstable.', 'open'],
        ['Yes. Server load alone explains delays of this size.', 'failed'],
        ['Timing samples only.', 'open'],
        ['Low.', 'open'],
        ['Not reliably.', 'failed'],
      ]),
    },
    {
      id: 'f-traversal-download',
      title: 'Path traversal in document download',
      severity: 'High',
      status: 'Potential',
      confidence: 'Medium',
      endpoint: 'GET /documents/download',
      parameter: 'file',
      rule: 'path-traversal',
      created_at: ago(29),
      description:
        'A traversal sequence changes the response from 404 to 200, but the body is the portal login page, not a system file.',
      impact: 'Unknown until the 200 response is shown to contain file content.',
      recommendation: 'Check whether the 200 is an authentication redirect rendered server-side.',
      references: ['CWE-22'],
      baseline: `HTTP/1.1 404 Not Found
Content-Length: 0`,
      observed: `HTTP/1.1 200 OK
Content-Type: text/html

<title>Sign in | Halden Freight</title>`,
      delta: '<title>Sign in | Halden Freight</title>',
      reproduction: `GET /documents/download?file=..%2F..%2Fetc%2Fpasswd HTTP/1.1
Host: portal.halden-freight.test`,
      gate: gate([
        ['404, empty body.', 'passed'],
        ['Status 404 to 200, body is the login page.', 'passed'],
        ['Not established. No file content is present.', 'failed'],
        ['Reproduced 3 of 3 times.', 'passed'],
        ['Not attempted.', 'open'],
        ['Yes. The session check may run before the file lookup and render the login page.', 'failed'],
        ['Status change only.', 'open'],
        ['Medium.', 'open'],
        ['Yes, the status change reproduces.', 'passed'],
      ]),
    },
    {
      id: 'f-hsts',
      title: 'Strict-Transport-Security missing on HTML documents',
      severity: 'Low',
      status: 'Confirmed',
      confidence: 'Confirmed',
      endpoint: 'GET /',
      rule: 'headers.hsts',
      created_at: ago(40),
      description: 'HTML 2xx documents served over HTTPS do not set Strict-Transport-Security.',
      impact: 'A network attacker can downgrade a first visit to plain HTTP.',
      recommendation: 'Set Strict-Transport-Security: max-age=31536000; includeSubDomains on HTML responses.',
      references: ['CWE-319', 'RFC 6797'],
      baseline: `HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
X-Frame-Options: DENY`,
      observed: `(header absent on 14 of 14 HTML 2xx documents over HTTPS)`,
      delta: '14 of 14',
      gate: gate([
        ['HTML 2xx documents served over HTTPS.', 'passed'],
        ['The header is absent.', 'passed'],
        ['Without HSTS, the first request can be downgraded.', 'passed'],
        ['14 of 14 documents.', 'passed'],
        ['Checked on a second session.', 'passed'],
        ['No. JSON, redirects and error pages were excluded.', 'passed'],
        ['Response headers of 14 documents.', 'passed'],
        ['Confirmed.', 'passed'],
        ['Yes.', 'passed'],
      ]),
    },
    {
      id: 'f-server-banner',
      title: 'Server version disclosed in response headers',
      severity: 'Informational',
      status: 'Informational',
      confidence: 'Confirmed',
      endpoint: 'GET /',
      rule: 'headers.server',
      created_at: ago(40),
      description: 'Responses include Server: nginx/1.24.0.',
      impact: 'None directly. Useful context for version-specific research.',
      recommendation: 'Optional: set server_tokens off.',
      references: ['CWE-200'],
    },
  ],
}

const HOSTS = ['portal.halden-freight.test', 'api.halden-freight.test', 'cdn.halden-freight.test']

const SEED: [string, number, string, number, number, number][] = [
  ['GET', 0, '/', 200, 18_432, 84],
  ['GET', 2, '/assets/app.3f9c2.js', 200, 241_118, 41],
  ['GET', 2, '/assets/app.3f9c2.css', 200, 38_204, 22],
  ['POST', 1, '/auth/session', 200, 512, 212],
  ['GET', 1, '/api/me', 200, 731, 66],
  ['GET', 1, '/api/orders?id=10442', 200, 412, 71],
  ['GET', 1, "/api/orders?id=10442'", 500, 233, 63],
  ['GET', 1, '/api/orders?id=10443', 200, 409, 69],
  ['GET', 0, '/search?q=pallet', 200, 22_871, 133],
  ['GET', 0, '/search?q=%3Csvg%20onload%3Dsn7x4()%3E', 200, 22_902, 128],
  ['GET', 1, '/api/invoices?sort=created_at', 200, 6_114, 412],
  ['GET', 1, '/api/invoices?sort=created_at;SELECT+pg_sleep(5)--', 200, 6_114, 5214],
  ['GET', 0, '/documents/download?file=bol-10442.pdf', 200, 88_310, 97],
  ['GET', 0, '/documents/download?file=..%2F..%2Fetc%2Fpasswd', 200, 9_812, 88],
  ['GET', 0, '/robots.txt', 200, 112, 19],
  ['GET', 0, '/sitemap.xml', 404, 0, 21],
  ['OPTIONS', 1, '/api/shipments', 204, 0, 18],
  ['GET', 1, '/api/shipments?page=2', 200, 14_207, 154],
  ['PUT', 1, '/api/profile', 403, 96, 58],
  ['GET', 0, '/account/settings', 302, 0, 34],
]

function rawRequest(method: string, host: string, path: string): string {
  return `${method} ${path} HTTP/1.1
Host: ${host}
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)
Accept: ${path.startsWith('/api') ? 'application/json' : 'text/html,application/xhtml+xml'}
Cookie: session=7f3c0b9e1d0a44c2
Connection: keep-alive`
}

function rawResponse(code: number, size: number, path: string): string {
  const text: Record<number, string> = { 200: 'OK', 204: 'No Content', 302: 'Found', 403: 'Forbidden', 404: 'Not Found', 500: 'Internal Server Error' }
  const type = path.startsWith('/api') ? 'application/json' : path.endsWith('.js') ? 'text/javascript' : path.endsWith('.css') ? 'text/css' : 'text/html; charset=utf-8'
  return `HTTP/1.1 ${code} ${text[code] ?? ''}
Server: nginx/1.24.0
Content-Type: ${type}
Content-Length: ${size}
Cache-Control: no-store
X-Request-Id: ${Math.random().toString(16).slice(2, 10)}`
}

export function demoEntry(i: number, minutesAgo: number): HistoryEntry {
  const [method, h, path, code, size, dur] = SEED[i % SEED.length]
  const host = HOSTS[h]
  const [p, q = ''] = path.split('?')
  return {
    id: `demo-${i}-${minutesAgo}`,
    timestamp: ago(minutesAgo),
    method,
    scheme: 'https',
    host,
    port: 443,
    path: p,
    query: q,
    url: `https://${host}${path}`,
    protocol: h === 2 ? 'HTTP/2' : 'HTTP/1.1',
    status_code: code,
    request_size: 380 + path.length,
    response_size: size,
    duration_ms: dur,
    tls_enabled: true,
    request_headers: rawRequest(method, host, path),
    response_headers: rawResponse(code, size, path),
  }
}

export const DEMO_TRAFFIC: HistoryEntry[] = SEED.map((_, i) => demoEntry(i, 40 - i * 1.5)).reverse()
