# Sentinel Design System: "Evidence Grade"

Version 1.1 · Revised after review: brand violet from the project logo, the logo itself, a lattice background system, and an Awwwards-grade marketing site.

Applies to the desktop platform (`ui/`, Tauri) and the marketing site (`site/`).

---

## 1. Design read

**Reading this as:** a professional security instrument for people who sit in it six hours a day, plus a marketing site for technical buyers who distrust marketing. Instrument direction, quiet and exact, leaning toward native CSS tokens + IBM Plex + border-led elevation.

### Positioning interview (assumed answers, change any and the spec moves with it)

| Question | Assumed answer | Consequence |
|---|---|---|
| Price and buyer | Open-source core, the buyer is the tester or the security lead who signs off on their reports | Credibility beats persuasion. No hype copy, real numbers only. |
| Session length | 2 to 8 hours a day, often split across two monitors | Dark default in the app, crisp fast motion, no decorative animation on repeated actions. |
| Data density | Tables (traffic, findings) and raw HTTP editors | Compact 32px rows, mono figures, right-aligned numbers, hairline rules. |
| Existing constraints | Tauri 2 + React 19 + Vite, plain CSS, Rust core emitting events | Token CSS, no Tailwind in the app, self-hosted fonts (the app must work offline). |
| Reference rejection | Burp Suite (dense but dated, gray Swing chrome) and the "black + neon green hacker" look | Neither the legacy-Java look nor the category costume. |

### The idea

Sentinel's promise is that a result is not a vulnerability until it is proven. The interface makes that promise visible:

1. **Verification status is the primary axis, severity is secondary.** Every list sorts and groups by status first. A Critical that is only *Potential* never looks louder than a Medium that is *Confirmed*.
2. **Baseline vs. observed is a visual primitive.** Wherever Sentinel compares, the UI shows two things side by side with the delta marked, never a lone verdict.
3. **Color means something or it is absent.** The chrome is near-monochrome violet-ink. The brand violet (from `assets/logo.svg`) marks focus, selection, live state and the logo. Severity hues appear only on severity marks.
4. **The logo is the background.** The logo's hexagon tiles into a lattice. A sweep across it lights cells and lets most fade back: the precision idea drawn as texture, not decoration.

---

## 2. Direction: Instrument

One direction, not mixed. Violet-ink graphite (hue 290, the logo's hue), hairline rules, mono figures, the brand violet used structurally and sparingly in the app, committed on the marketing site.

### Domain-reflex test

> Could someone name the category from the visuals alone?

The security-tool reflex is near-black with neon green or purple glow (this repo's own `assets/logo.svg` and `design/concepts.html` concepts 1, 5, 6 sit there). Sentinel avoids it by:

- **Violet is the brand, not an AI glow.** It comes from the existing logo and is used as a solid structural color (logo, lattice, selection), never as a blurred orb, gradient text or neon outline.
- **The accent is not an action color in the app.** Primary actions are ink-filled (solid foreground color). Violet marks focus, selection, live state and links.
- **The background is domain-specific.** A hex lattice made from the logo with a verification sweep, instead of mesh gradients or terminal rain.
- **No terminal costume.** Raw HTTP is shown as a document, not as a fake console.

### Transpositions from the references (at least three per source)

| Dimension | Linear | Vercel | teenage engineering | Sentinel |
|---|---|---|---|---|
| Neutral hue | blue-black | pure gray | mist green-gray | graphite, hue 170, chroma 0.006 to 0.012 |
| Accent | acid lime on primary action | terminal green, links only | none (amber for warnings) | ink-filled primary in the app, brand violet (from the logo) for focus, selection, live |
| Type | Inter Variable + Berkeley Mono | Geist + Geist Mono | te-20 / te-40 | IBM Plex Sans + IBM Plex Mono |
| Radii | 6 / 12 / pill | 6 / pill | 0 | 0 panes / 4 controls / 8 floating |
| Density | comfortable marketing | compact | comfortable | compact app (32px rows), airy site |
| Elevation | inset hairlines | rings | tonal cut | 1px hairlines + one shadow level for floating surfaces only |

---

## 3. Tokens

All colors are defined in OKLCH (perceptually even steps). Hex values below are approximations for reference only.

### 3.1 Neutrals (12 roles)

| Role | Dark | Light | Use |
|---|---|---|---|
| `--bg-0` canvas | `oklch(0.165 0.006 170)` ≈ #101413 | `oklch(0.975 0.004 170)` ≈ #f4f7f6 | App and page background |
| `--bg-1` surface | `oklch(0.195 0.007 170)` | `oklch(0.993 0.002 170)` | Panes, sidebar, cards |
| `--bg-2` raised | `oklch(0.225 0.008 170)` | `oklch(0.958 0.005 170)` | Inputs, code wells, table header |
| `--bg-3` hover | `oklch(0.255 0.009 170)` | `oklch(0.935 0.006 170)` | Row hover, ghost hover |
| `--bg-4` selected | `oklch(0.285 0.012 175)` | `oklch(0.915 0.012 175)` | Selected row |
| `--line-1` hairline | `oklch(0.275 0.008 170)` | `oklch(0.905 0.006 170)` | Dividers between rows and panes |
| `--line-2` rule | `oklch(0.345 0.009 170)` | `oklch(0.84 0.008 170)` | Control borders, focusable edges |
| `--fg-1` primary | `oklch(0.955 0.004 170)` | `oklch(0.22 0.01 170)` | Headings, body, data |
| `--fg-2` secondary | `oklch(0.80 0.008 170)` | `oklch(0.40 0.01 170)` | Labels, secondary data |
| `--fg-3` tertiary | `oklch(0.66 0.01 170)` | `oklch(0.52 0.01 170)` | Metadata, placeholders (AA 4.5:1 on `--bg-1` verified) |
| `--fg-4` disabled | `oklch(0.50 0.01 170)` | `oklch(0.70 0.008 170)` | Disabled only, never body text |
| `--ink` primary fill | `--fg-1` | `--fg-1` | Primary button background (text uses `--bg-0`) |

### 3.2 Accent and semantic hues

| Token | Dark | Light | Rule |
|---|---|---|---|
| `--acc` brand violet | `oklch(0.74 0.16 293)` | `oklch(0.50 0.22 292)` | Focus ring, selected-row marker, links. Derived from logo `#7a3cf0`. Never a button fill in the app. |
| `--acc-solid` | `oklch(0.55 0.23 293)` | `oklch(0.50 0.22 292)` | Switch on-state, checkbox fill |
| `--acc-wash` | violet at 16% | violet at 10% | Selected nav item, palette selection, fresh-row flash |
| `--lattice` | violet at 11% | violet at 9% | Hex lattice background lines |
| `--sev-critical` | `oklch(0.68 0.19 25)` | `oklch(0.55 0.20 27)` | Severity mark only |
| `--sev-high` | `oklch(0.75 0.14 52)` | `oklch(0.60 0.15 50)` | Severity mark only |
| `--sev-medium` | `oklch(0.84 0.12 88)` | `oklch(0.66 0.13 80)` | Severity mark only |
| `--sev-low` | `oklch(0.74 0.06 235)` | `oklch(0.55 0.08 240)` | Severity mark only |
| `--sev-info` | `--fg-3` | `--fg-3` | Severity mark only |
| `--ok` | `oklch(0.78 0.09 150)` | `oklch(0.52 0.10 150)` | 2xx status codes, proxy up |
| `--warn` | `--sev-medium` | `--sev-medium` | 4xx status codes |
| `--bad` | `--sev-critical` | `--sev-critical` | 5xx, errors, destructive confirm |

Severity colors are desaturated relative to a typical dashboard and never fill large areas. A severity is always shown as a **6px bar + text label**, never color alone.

### 3.3 Verification status marks

Status is encoded by **shape and fill**, so it survives grayscale and color-vision deficiency. Marks are 10px CSS squares, drawn with borders and backgrounds (no icon font).

| Status | Mark | Meaning in the UI |
|---|---|---|
| Confirmed | solid square, `--fg-1` | Passed the full Verification Gate |
| Detected | square, left half filled | Signal observed and reproduced, not independently verified |
| Potential | dashed square outline | Signal observed, benign explanation not excluded |
| Needs Manual Verification | square outline with center dot | Engine stopped, a human must decide |
| Informational | small circle outline | Not a vulnerability |

The five values are never merged, renamed or abbreviated, per the product rule.

### 3.4 Typography

Two families, one superfamily. Both are OFL, licence cost 0, self-hosted via `@fontsource` in the app and Google Fonts on the site.

- **IBM Plex Sans** 400 and 500 only. Never 600+.
- **IBM Plex Mono** 400 and 500. Used for: every number, every HTTP artifact, hosts, paths, IDs, keyboard shortcuts.
- OpenType: `font-variant-numeric: tabular-nums` on all data; `"zero"` (slashed zero) in mono.

**App scale** (ratio about 1.2, tight body band, decisive page-title jump):

| Token | Size / line | Weight | Tracking | Use |
|---|---|---|---|---|
| `--t-micro` | 11 / 16 | 500 | +0.04em, uppercase allowed only in table headers | Table headers, kbd |
| `--t-small` | 12 / 16 | 400 | 0 | Metadata, mono data |
| `--t-body` | 13 / 20 | 400 | 0 | Default UI text |
| `--t-strong` | 14 / 20 | 500 | -0.005em | Row titles, buttons |
| `--t-h3` | 16 / 22 | 500 | -0.01em | Pane titles |
| `--t-h2` | 20 / 26 | 500 | -0.015em | Detail titles |
| `--t-h1` | 26 / 32 | 500 | -0.02em | View title |

**Site scale** (ratio 1.333+ at the display end): 15/24 body, 17/28 lead, 22/30, 30/36, 44/48, 68/68 display at -0.035em, weight 500.

### 3.5 Space, radius, layout

- Base 4px. Scale: `2 4 8 12 16 24 32 48 64 96 128`.
- Radius vocabulary (three values): **0** panes and tables, **4px** controls, chips, inputs, **8px** floating surfaces (modal, popover, toast, command palette). Nested radius: inner = outer minus padding.
- App shell: 216px sidebar, 44px top bar, content fills. Panes are separated by 1px `--line-1`, never by gaps with cards.
- Table rows: compact 32px (default), comfortable 40px (findings list). Cell padding 0 12px.
- Site container 1240px, reading measure 64ch.

### 3.6 Elevation

Border-led. Two levels only.

- **Level 0:** canvas and panes, separated by 1px hairlines.
- **Level 1 (floating):** modal, palette, popover, toast. 1px `--line-2` border plus a two-layer shadow tinted to the canvas hue: `0 1px 2px oklch(0.1 0.01 170 / .25), 0 12px 32px -8px oklch(0.1 0.01 170 / .45)`. In light theme the opacities halve.
- No glassmorphism on any data surface. The scrim behind a modal is a flat `oklch(0.1 0.01 170 / .55)`, no blur.

### 3.7 Motion

Motion tokens come from transitions.dev; values are the shared scale.

| Token | Value | Use |
|---|---|---|
| `--d-press` | 120ms | Button press scale (0.97) and release |
| `--d-quick` | 150ms | Close of modal/popover, hover color |
| `--d-fast` | 250ms | Modal open, tabs pill slide, detail pane enter |
| `--d-toast-in` / `--d-toast-out` | 350ms / 250ms | Toast |
| `--ease-out` | `cubic-bezier(0.22, 1, 0.36, 1)` | All enters, slides, pill |
| `--ease-move` | `cubic-bezier(0.77, 0, 0.175, 1)` | On-screen morphs (site only) |

Rules (Emil Kowalski's frequency test):

- **No animation** on view switching, row selection, keyboard navigation, or the command palette. These happen hundreds of times a day.
- **Animate** only state changes the user should notice: a modal or toast, the tab pill, a newly captured traffic row (a 600ms background fade from `--acc-wash` to transparent, opacity only), and a scan stage completing.
- Only `transform` and `opacity` (plus `filter: blur(2px)` on toasts). Never `transition: all`.
- Hover effects gated behind `@media (hover: hover) and (pointer: fine)`.
- `prefers-reduced-motion: reduce` removes all movement; opacity fades remain.

### 3.8 Icons, copy, imagery

- **Phosphor**, Regular weight, 16px in the app, 20px on the site. One family.
- Sentence case everywhere. No exclamation marks, no emoji, no em-dashes in UI copy.
- Errors name the cause and the next step: "Proxy could not bind 127.0.0.1:8080. Another process is using the port. Change the port in Settings."
- **Imagery:** real screenshots of the product only. No AI imagery, no stock, no animated mesh background.
- **Logo:** the project logo `assets/logo.svg` (hexagon + block S, violet gradient). Used as-is at large sizes. At UI sizes (16 to 24px) the same geometry and gradients are used without the glow filter, which only blurs edges when small. A flat single-color variant (`currentColor`) exists for monochrome contexts. The geometry is never redrawn.
- **Lattice:** the logo's hexagon tiled as a 1px stroke pattern (27.71 x 48px tile), masked by a radial fade so it never fills a whole surface. In the app it sits behind the Overview project list, empty pages and the Scanner pipeline, and sweeps only while a scan runs.

---

## 4. Platform (desktop app)

### 4.1 Information architecture

The sidebar follows the product's own pipeline order, grouped into three verbs. Group labels are the only uppercase in the sidebar.

```
Overview
CAPTURE   Proxy · Target & scope · Site map
TEST      Repeater · Intruder · Scanner · Decoder · Comparer · Sequencer
PROVE     Findings · Reports
──────────
Settings
Proxy status block (live state, address, upstream)
```

Built in this pass (wired to Tauri commands, or fully client-side): Overview, Proxy, Repeater, Scanner, Findings, Decoder, Reports, Settings. Items without a backend command yet (Target & scope, Site map, Intruder, Comparer, Sequencer) are not shown in the nav until they have one. A nav item that leads to a placeholder is worse than no item.

### 4.2 Shell

- **Top bar (44px):** project switcher (left), scope indicator `In scope: 3 rules` (opens scope), command palette trigger `Ctrl K` (center-left), theme toggle and demo-data notice (right).
- **Proxy status block** pinned to the bottom of the sidebar: live dot (the one permitted decorative-looking dot, because it is real state), `127.0.0.1:8080`, upstream `direct` or `socks5h via Tor`.
- **Command palette (Ctrl K):** opens instantly, no animation. Jump to any view, run "Send selected to Repeater", "Start scan", "Toggle theme".

### 4.3 Screen specs

**Overview.** No metric-tile hero row. A two-column page: left, the project list (name, target in mono, environment, status, finding count by verification status); right, "What needs you": findings in *Needs Manual Verification*, then recent scans. Empty state teaches the first action: "Create a project. Sentinel needs a target and scope before any engine sends a request."

**Proxy.** Three panes: traffic table (method, host, path, status, size, time; all numerics right-aligned mono), and a message inspector with a sliding Request / Response tab. Toolbar: filter input, "Intercept off" toggle, "Send to Repeater" (primary when a row is selected). New rows enter with the accent wash fade. Empty state shows the exact proxy address to configure, with a copy button.

**Repeater.** Two editors side by side, request left, response right. Send is the single primary action (`Ctrl Enter`). Response header strip: status, protocol, time, size, all mono. Tabs for multiple requests.

**Scanner.** Left: target, rule set (SQL injection, XSS, path traversal) with checkboxes, request cap and concurrency. Right: a live pipeline readout per rule: `baseline → mutate → observe → compare → verify`, each stage a hairline segment that fills when done. Results stream in as rows grouped by verification status. Out-of-scope mutations are counted, never hidden: "14 mutations skipped, out of scope."

**Findings.** The signature screen. List (comfortable rows, grouped by status, then severity) and a detail pane:
1. Title, severity bar, status mark, confidence.
2. **The Verification Gate:** the nine questions as a ledger. Each row: question, answer, and a state (`passed`, `failed`, `not established`). A finding is Confirmed only if every row passed; the UI shows which row held it back.
3. **Baseline vs. observed:** two code wells side by side with the delta highlighted.
4. Reproduction request (copy, send to Repeater), remediation, references.

**Decoder.** Input well, a chain of transforms (URL, Base64, Hex, HTML, Unicode, JWT inspect), each step's output shown below the previous, so a chain reads top to bottom. Fully client-side.

**Reports.** Pick format (HTML, JSON, Markdown, PDF), include statuses (Confirmed on by default, Potential off by default), preview pane. Generates Markdown and JSON client-side today.

**Settings.** Proxy port, upstream (direct or SOCKS5/Tor), theme. Each setting has a one-line consequence statement under it.

### 4.4 State table (every surface)

| Surface | Empty | Loading | Error | Partial | Offline / no backend |
|---|---|---|---|---|---|
| Projects | First-project prompt with the scope rule | Skeleton rows after 300ms delay only | Inline banner with cause + retry | n/a | Demo data with a visible "Demo data" notice |
| Proxy table | Proxy address + copy + browser setup line | n/a (live) | "Proxy not listening" banner + Settings link | Shows "500 most recent" cap | Demo data notice |
| Repeater | "No response yet. Ctrl Enter sends." | Send button label swaps to "Sending" | Error replaces response pane with cause | Body truncated at 256 KB, stated | Demo response |
| Scanner | Target prompt | Live stage readout | Per-rule error row, scan continues | Cancelled scans keep results | Demo run |
| Findings | "No findings. That can be the right answer." | Skeleton list | Inline | n/a | Demo data |

### 4.5 Keyboard

`Ctrl K` palette · `Ctrl Enter` send in Repeater · `J/K` move in tables · `Ctrl R` send selection to Repeater · `Esc` closes any floating surface. Every interactive element shows a 2px `--acc` focus ring with 2px offset.

---

## 5. Marketing site

One page, built as static HTML (`site/index.html`). Awwwards-grade, revised after review: dark-first (light theme first-class), committed brand violet, the logo as the hero object, a live canvas hex lattice with a verification sweep and cursor response, film grain at about 3%, Clash Display for headlines only (Fontshare, free) with IBM Plex Sans and Mono, masked line reveals, a pinned scroll sequence for the Verification Gate, a horizontal pinned pan for Capture / Test / Prove, one marquee (protocols), a magnetic primary CTA. Every motion has a reduced-motion static path. No orbs, no gradient text, no glass cards, no bounce, no animated counters.

| # | Section | Layout family | Job |
|---|---|---|---|
| 1 | Nav | floating detached pill, single line | Logo lockup, 4 links, "Download" |
| 2 | Hero | asymmetric: kinetic headline left, the logo large on the right over the live lattice | "Findings you can defend." + subtext under 20 words + Download (magnetic) + "View source" text link |
| 2b | Product reveal | full-bleed screenshot with a clip-path wipe on scroll | The real Findings screen |
| 3 | Verification Gate | sticky question list + live evidence panel driven by scroll | The signature feature, shown working |
| 4 | Precision | ledger of benchmarks: vulnerable fixture detected, safe fixture silent | Proof, with the real test counts |
| 5 | One platform | three grouped columns: Capture / Test / Prove, with the request path drawn through them | Breadth without a 15-tile grid |
| 6 | Scope | split: rule editor component + skipped-mutation counter | "Scope is enforced, not suggested" |
| 7 | Protocols strip | single marquee-free row of protocol names in mono | HTTP/2, HTTP/3, WebSocket, GraphQL, gRPC, JWT, OAuth |
| 8 | Ethical use + closing CTA | centered manifesto block (the one centered section) | Responsibility, then Download |
| 9 | Footer | columns | Docs, source, licence |

No eyebrows anywhere on the page (no small uppercase labels above headings). Headlines stand alone. One CTA label per intent: "Download" for install, "View source" for GitHub.

---

## 6. Evidence log

| Decision | Value | Source | Type | Confidence |
|---|---|---|---|---|
| Elevation by hairline, no card shadows | 1px rules, shadow only on floating | Linear, Vercel, teenage engineering style pages on styles.refero.design (all three agree) | Observed | High |
| Weight cap | 500 | Linear 510/590, Vercel 450/500 | Observed | High |
| Display tracking | -0.02em app, -0.035em site display | Linear -0.022em at 48px+, Vercel -0.06em at 64px | Observed, adapted | Medium |
| One chromatic action, rest monochrome | ink primary, accent for focus/live | Linear (lime only on primary action), Vercel (chromatic ~0%) | Observed, transposed | High |
| Small radii | 0 / 4 / 8 | Vercel 6 max, TE 0, Linear 6/12 | Observed, adapted | High |
| Section gap on site | 96 to 128px | Linear 96, Vercel 96 to 128 | Observed | High |
| Compact 32px rows | 32 / 40 | Carbon and Atlassian density modes (luxury skill floor, 7.5) | Documented | Medium |
| Motion durations and easing | 120 / 150 / 250ms, `cubic-bezier(0.22,1,0.36,1)` | transitions.dev token scale, Emil Kowalski frequency rules | Documented | High |
| Status encoded by shape | 5 marks | WCAG 1.4.1 (use of color) | Reasoned | High |
| Verdigris hue 172 to 182 | accent | Chosen to avoid severity hues (25 to 88, 235) and the neon-green reflex | Reasoned | Medium |
| Light-first marketing site | | Domain-reflex test (section 2) | Reasoned | Medium |

---

## 7. Quality gate

- [x] Research ran, evidence log attached.
- [x] Slop screen: no purple glow, no neon, no glass on data, no gradient text, no 3-equal-card rows, no fake terminal, no AI imagery.
- [x] Domain-reflex test run, answer is no (logo-derived lattice background, ink-filled actions, no neon, no hacker costume).
- [x] Typeface: IBM Plex Sans + Mono, OFL, free, outside the Inter/Roboto/system default set. Weights 400/500 only.
- [x] Scale: tight app band (11 to 16), decisive jump to 26; site display 68.
- [x] Palette in OKLCH, 12 neutral roles, two first-class themes.
- [x] Accent under 5%, never a button fill.
- [x] Three radii; two elevation levels, border-led.
- [x] Motion under 300ms for UI, named curves, reduced-motion path defined, none on high-frequency actions.
- [x] Tabular mono figures on all numeric data.
- [x] Empty, loading, error, partial and offline states specified per surface.
- [x] Focus ring and keyboard path specified.

## 8. Handoff

- App: `ui/src/styles/tokens.css` is the single source for every value above. Components never use raw colors.
- Site: `site/index.html` mirrors the same tokens inline.
- Not done in this pass: Tauri window icons still use the old PNGs (regenerate from `assets/logo.svg` with `npx tauri icon assets/logo.svg`); nav items for Intruder, Comparer, Sequencer, Target & scope, Site map (they appear once their Tauri commands exist).

---

## 9. Platform rules after the architecture review (v1.2)

These are commitments. Changing one means changing it everywhere it is listed.

| Rule | Where it is enforced |
|---|---|
| Scope matches on a domain boundary: the host itself or a real subdomain. Never a plain suffix. | `ui/src/lib/format.ts` `inScope`, used by Overview, Proxy, Scanner, sidebar badges |
| Proxy history is shared across projects. Every count shown for a project uses in-scope traffic only. | Overview Capture step, Proxy "In scope" default, sidebar badge |
| Only Confirmed is "proven". Detected is not. | `isProven` in `format.ts`, Overview Report step, Reports default (Confirmed only) |
| Verification status (engine) and decision (tester) are separate. Decisions use the core's existing `FindingStatus`: Verified, Draft (reopen), False Positive, Accepted Risk. | `update_finding_status` in `src-tauri/src/main.rs`, `DecisionBar` in Findings |
| False positives never reach a report. Accepted risks are reported and labelled. | Reports filter and Markdown output |
| The proxy status shown is the real bind result, never assumed. | `ProxyState.listening` in `src-tauri/src/main.rs` |
| A scan result that passed verification is stored as Verified, so the verdict survives a reload. | `src-tauri/src/scan.rs` |

Known gaps, not solved in the UI because they need the core:
- The Verification Gate record (nine answers, baseline, observed delta) is not persisted. Real findings show "No gate record is stored". Only demo data shows the full gate.
- Scans cannot be cancelled from the app; the engine has a cancel channel that is not exposed.
- Manual findings cannot be created from Repeater.
- Scope rules cannot be edited; the project target is the only include rule the app knows.
