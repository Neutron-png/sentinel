# Phase 16 — Capability Audit and Plan

Date: 2026-10-02
Status: Active

This document maps Sentinel's current state against the Phase 16
production-grade requirements (Burp-class capability parity, no fake
results, precision-first detection) and defines the work plan.

---

## 1. Governing principle (non-negotiable)

**NO FAKE RESULTS.** A finding is created only when deterministic technical
evidence establishes the vulnerability. Anything weaker is classified as
`Informational`, `Potential`, or `NeedsManualVerification`. Never mixed,
never inflated.

---

## 2. Capability parity matrix

Reference: publicly documented Burp Suite Pro capability list. Sentinel
implements its own original architecture; nothing proprietary is copied.

Legend: complete | partial | gap | n/a

| Capability               | Sentinel module(s)                                   | State   | Notes                                                        |
|--------------------------|------------------------------------------------------|---------|--------------------------------------------------------------|
| Dashboard/tasks          | `orchestrator`, `workflow`, `resources`              | partial | scheduler + pipeline exist; unified task dashboard pending   |
| Target/scope             | `scope`, `sitemap`, `crawler`                        | partial | include/exclude rules, scope enforcement; no scope visualization UI yet |
| Proxy (intercept)        | `proxy`, `intercept`, `browser/proxy`                | partial | HTTP/1.1 + TLS; history, match/replace, WS traffic pending   |
| Browser                  | `browser` (wry backend, automation, DOM)             | partial | recorded-login via `auth/recorder`                           |
| Repeater                 | `repeater`                                           | partial | tabs, edits, history, resend; response diff pending          |
| Intruder (fuzzing)       | `intruder`                                           | done    | positions, sets, encoders, 3 modes, cap+rate+concurrency+cancel, scope enforcement, baseline anomaly analysis |
| Sequencer (token entropy)| `sequencer`                                          | done    | Shannon per-char + per-position entropy, repeats, sequential/prefix/suffix, methodology + statistical confidence; never labels weak from length |
| Decoder                  | `decoder`                                            | done    | URL/base64/hex/HTML/unicode, JSON format/minify, JWT decode, hash ID, chaining |
| Comparer (diff)          | `comparer`                                           | done    | line diff (LCS + coarse fallback), header diff, structural JSON diff |
| Scanner (passive)        | `scanner/passive` + rules (security headers, JWT, OAuth, gRPC, WebSocket) | partial | header rules are now context-gated (HTML 2xx only, HSTS over HTTPS); JWT/OAuth/gRPC/WS rules existing |
| Scanner (active)         | `scanner/rules/active` (SQLi, XSS, path traversal)   | partial | baseline→mutate→compare→verify implemented in analyzer SDK    |
| Verification engine      | `findings` (confidence, severity, dedup, verification), `services/verification`, `scanner/analyzer` | partial | the defining feature; needs an explicit false-positive gate |
| OOB / Collaborator       | **none**                                             | gap     | payload-sent vs interaction-observed distinction required    |
| Logger                   | `history` (filter, search, sort)                     | partial | persistent activity log; redaction mechanism pending        |
| Organizer                | `evidence`, `findings`, db                           | partial | notes/tags/highlights/collections pending                   |
| Extensions/plugins       | `plugins`, `sdk` (permissions, manifest, lifecycle)  | partial | capability checks exist; runtime isolation pending          |
| Authenticated testing    | `session`, `auth` (detector, refresh, re-auth), `network/cookie_jar` | partial | credential storage protections pending        |
| Projects                 | `db` (SQLite), assessment models, export/import      | partial | scope/target/findings persisted; evidence hashing exists    |
| Reporting                | `reporting` (HTML/JSON/MD/PDF), `reports`            | partial | professional template compliance review pending             |
| HTTP/2, HTTP/3, WS, gRPC, GraphQL | `http2`, `http3`, `websocket`, `grpc`, `graphql` | partial | HPACK was broken — fixed in Phase 16 (see 3.1)         |
| Resource safety          | `resources` (rate limit, concurrency, retry, circuit breaker) | partial | must be wired into every request engine          |

---

## 3. Phase 16 work plan (priority order)

### 3.0 Restore code health (done)
- Fixed 38 compile errors (Windows backslash paths in `use` statements
  inside `src/scanner/rules/active/xss/*`).
- Replaced the broken HPACK implementation with an RFC 7541-compliant
  codec (real integer/string encoding, 1-based static table indexing,
  bounded decode: Huffman rejection, truncation and overflow errors,
  header-list size limit, dynamic-table decode support).

### 3.1 Detection quality gate (defsining feature)
- One shared `DetectionQuality` framework module every rule must use:
  baseline evidence, mutation delta, independent verification result,
  false-positive conditions checked, confidence computed from evidence
  quality — never from match count.
- No finding is `Confirmed` unless verification completed with
  independent confirmation and plausible benign explanations ruled out.
- Add regression tests: vulnerable fixtures must be found, safe
  fixtures must produce zero confirmed findings.

### 3.2 Intruder / fuzzing engine (implemented: `src/intruder`)
- Payload positions (`§...§` markers) and insertion points; payload sets:
  list, file (wordlist), numeric range, charset; encoders (URL, base64,
  hex, none).
- Attack modes: Sniper, Pitchfork, ClusterBomb.
- Response analysis: status, body length, word/line count, timing delta
  vs baseline; anomaly flagging from a computed baseline.
- Engine constraints: bounded concurrency (`Semaphore`), monotonic rate
  gate (`RateGate`), hard request ceiling (cap exceeded is an error, not
  a silent truncation), per-request timeout, cooperative cancellation.
- Scope enforcement: every materialized request is resolved to a URL and
  checked before send; out-of-scope mutations are skipped and counted,
  never sent.

### 3.3 Decoder toolbox (implemented: `src/decoder`)
- URL (encode/decode/recursive decode), base64 + base64url, hex, HTML
  entities (named + numeric + hex), unicode escapes (`\uXXXX`, surrogate
  pairs, `\xXX`), JSON format/minify, JWT decode, hash identification.
- Transform chaining with explicit pipeline order.

### 3.4 Comparer (implemented: `src/comparer`)
- Line-level text diff (LCS with a coarse prefix/suffix fallback for very
  large inputs), case-insensitive header diff, structural JSON diff with
  JSON-path reporting.

### 3.5 Sequencer (implemented: `src/sequencer`)
- Collect token samples; analysis: Shannon entropy per character and per
  position, character frequency distribution, exact-duplicate detection,
  sequential-numeric detection, fixed prefix/suffix, small-alphabet note.
- Output is statistical evidence with an explicit methodology string and
  a statistical-power confidence score. A token is never labelled "weak"
  from length or appearance alone; low-entropy observations require a
  measured sub-1-bit position with at least 16 samples.

### 3.6 OOB architecture (planned)
- Self-hosted interaction record service (HTTP/DNS hooks) integrated
  with the scanner; payloads embed unique per-injection correlation
  IDs. **Payload sent ≠ interaction observed.** A finding is only
  created from an observed, correlated interaction.

### 3.7 Remaining hardening
- Proxy: malformed traffic test matrix (chunked, multipart, binary,
  large bodies, timeouts, resets); WS interception.
- Logger: sensitive-field redaction.
- Benchmark suite: intentionally vulnerable + intentionally safe local
  fixtures; measure detection rate, false-positive rate, time to
  detection, evidence quality (see 5).

### 3.8 Detection-quality fixes applied
- Security-header rules previously reported on **any** response with a
  missing header, at `Confirmed` confidence — a textbook false positive
  (JSON APIs, redirects, error pages, HTTP responses for HSTS). They now
  require a served HTML document with a 2xx status, non-empty body, and a
  derivable HTML content type; HSTS additionally requires HTTPS. Findings
  are downgraded to `Medium` confidence and the description/evidence state
  explicitly that this is a configuration observation requiring manual
  confirmation of exploitability. (`src/scanner/passive/context.rs`,
  `src/scanner/rules/macros.rs`.)
- `tests/bench_security_headers.rs` is the first benchmark fixture set:
  vulnerable HTML fixtures must be detected; hardened, JSON-API,
  redirect, error-page, HTTP-HSTS, and missing-content-type fixtures must
  yield **zero** findings.
- **XSS**: all seven techniques previously flagged HTML-encoded and
  partial reflections, or mere keywords (`alert(1)`, `<script>`), at
  `High` confidence. They now require a **verbatim, unescaped** reflection
  of the injected payload plus payload-relevant evidence (tag/attribute/
  scheme breakout), report `Potential` (never Confirmed), and use honest
  confidence. `tests/bench_xss.rs` enforces this.
- **SQLi**: error-based now requires a database-error signature that is
  **new relative to the baseline**; time-based requires both an absolute
  and a relative delay; boolean/union require material, directional change
  and report `Low`. `tests/bench_sqli.rs` enforces this.
- **Baseline wiring (fixed)**: the pipeline never sent a baseline, and
  `SqliRule` used the injected response as its own baseline, making all
  differential techniques inert. `ScanRuleContext` now carries a
  `baseline`, `PipelineExecutor` sends the unmutated target request once
  and attaches it, and `SqliRule` compares against it (or produces no
  finding when no baseline is available). `tests/bench_sqli_e2e.rs` runs
  the real rule end-to-end against local vulnerable and safe servers.
- **Critical network bug (fixed)**: `HttpClient::execute` never serialized
  `HttpRequest::query_params` into the URL, so every active rule that
  injected into a query parameter (SQLi, reflected XSS, path traversal,
  …) sent no payload at all. Query parameters are now appended with proper
  percent-encoding (`src/network/client.rs`).

---

## 4. Architecture invariants

1. Every request engine (scanner, intruder, crawler, proxy) resolves
   scope through `scope::engine` before any network I/O.
2. Every long-running test session is owned by `resources` limits.
3. Findings are created only through `findings::engine`, which applies
   the dedup + confidence + verification pipeline; no module may insert
   findings directly into storage.
4. AI (if ever added) reads evidence; it never writes verdicts.

---

## 5. Benchmark definition

`tests/bench/` (planned): local fixtures with known-good and known-bad
behaviors:

| Fixture class        | Vulnerable variant              | Safe variant (must yield zero findings) |
|----------------------|---------------------------------|------------------------------------------|
| Reflected XSS        | echoes unescaped param          | escapes + context-aware encoding          |
| SQLi (error-based)   | echoes SQL error, breaks syntax | parameterized store                       |
| Path traversal       | serves arbitrary files          | canonicalized whitelist                   |
| Security headers     | missing CSP/HSTS on HTML        | correct headers                           |

Metrics per fixture: TP, FP, FN, time-to-detect, request count,
evidence completeness. CI gate: FP rate on safe fixtures must be 0.
