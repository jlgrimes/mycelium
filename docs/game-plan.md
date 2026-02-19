# Mycelium — Big Game Plan

## Mission

Eliminate developer reasoning loops by forcing frame pivots to isomorphic problems, then mapping robust solution patterns back to code.

---

## Phase 1 — Wedge Dominance (Current)

**Goal:** Win one use case: developer debugging loop escape.

### 1. Product contract

Status: **in progress (mostly shipped)**

- `/solve/debug` shipped
- `/solve/debug/concise` shipped
- Contract framing shipped: `ABSTRACT/SEARCH/MAP/SYNTHESIZE`
- Verification + fallback pivot enforcement shipped
- Mapping confidence surfaced in output (`high|medium|low`)

Remaining:

- Move more debug-specific logic into explicit staged provider prompts (not just route shaping)

### 2. Loop Escape Engine v1

Status: **in progress**

- Loop-escape framing and pivot rationale enforcement are live in debug route contract
- Staged mapping confidence signal added (`EntityMapping.confidence`)

Remaining:

- Add explicit `pivot` stage artifact to staged engine model
- Add repeated-hypothesis guard with evidence checks

### 3. Eval that matters

Status: **in progress (major slice shipped)**

- Debugging benchmark suite shipped (`debugging-v1`, 10 cases)
- Metrics shipped:
  - `actionability_score`
  - `verification_presence`
- Baseline vs staged report committed (`reports/debugging-v1-baseline-vs-staged.txt`)

Remaining:

- Add loop-specific metrics:
  - `loop_escape_rate`
  - `repeat_failure_suppression`
  - `time_to_first_new_action`
- Run non-stub benchmark passes for meaningful deltas

### 4. Reliability gates

Status: **shipped baseline**

- Robust JSON extraction and fallback parsing in OpenClaw adapter
- Retry/backoff + auth override handling
- CI gates for fmt/clippy/tests

---

## Phase 2 — Developer UX + Daily Use

**Goal:** Make it useful in real debugging sessions, not just demos.

### 1. Interfaces

- Keep HTTP API
- Add practical CLI entrypoints
- Keep concise mode optimized for fast operator action

### 2. Trust layer

- Stage artifacts visible in debug contract
- Confidence visible in mapping output
- Keep assumptions + failure conditions explicit

### 3. User loop

- Run with 5–10 real dev users
- Weekly quality iteration from real transcripts

---

## Phase 3 — MCP Expansion

**Goal:** Make Mycelium available beyond OpenClaw.

### 1. MCP server crate

- `mycelium.solve_debug`
- `mycelium.solve_general`
- (later) `mycelium.eval_debug_case`

### 2. Client integrations

- Claude Desktop MCP config
- Cursor / VS Code MCP config
- Quickstart examples

### 3. Ops controls

- Auth + rate limiting
- Tool-call telemetry
- Stable versioned schema

---

## Phase 4 — SaaS Viability

**Goal:** Prove this is a product-category wedge.

### 1. Positioning

- "Eliminate debugging loops via frame pivoting"

### 2. Packaging

- Individual plan: personal loop-escape debugger
- Team plan: shared traces, policy, auditability

### 3. Distribution

- Dev-first channels (GitHub, HN, Reddit, X)
- Public benchmark deltas and before/after debugging demos

---

## Definition of Success (Wedge)

- Developers get a **new** actionable move quickly
- Repeated failed hypotheses are suppressed
- Every answer includes concrete verification
- Benchmarks show measurable delta over baseline
