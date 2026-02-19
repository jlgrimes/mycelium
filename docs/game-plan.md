# Mycelium — Big Game Plan

## Mission
Eliminate developer reasoning loops by forcing frame pivots to isomorphic problems, then mapping robust solution patterns back to code.

---

## Phase 1 — Wedge Dominance (Now)
**Goal:** Win one use case: developer debugging loop escape.

### 1. Product contract
- Lock `/solve/debug` response contract
- Required fields: pivot rationale, verification step, fallback pivot
- No output ships without executable next action

### 2. Loop Escape Engine v1
- Add explicit `pivot` stage to pipeline
- Add loop signature + repeated hypothesis guard
- Enforce: no reusing failed frame without new evidence

### 3. Eval that matters
- Build debugging-only benchmark pack (10–20 realistic bug cases)
- Track:
  - `loop_escape_rate`
  - `actionability_score`
  - `verification_presence`
  - `repeat_failure_suppression`

### 4. Reliability gates
- Robust JSON handling and fallback extraction
- Deterministic mode knobs where possible
- Tests as release blockers

---

## Phase 2 — Developer UX + Daily Use
**Goal:** Make it useful in real debugging sessions, not just demos.

### 1. Interfaces
- Keep HTTP API
- Add practical CLI entrypoints
- Output copy-paste patch + verify steps

### 2. Trust layer
- Show stage artifacts (abstract/search/pivot/map/synthesize)
- Show confidence + assumptions + failure conditions

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
**Goal:** Prove this is a product category wedge.

### 1. Positioning
- "Eliminate debugging loops via frame pivoting"

### 2. Packaging
- Individual plan: personal loop-escape debugger
- Team plan: shared traces, policy, auditability

### 3. Distribution
- Dev-first channels (GitHub, HN, Reddit, X)
- Public benchmark deltas and before/after debugging demos

---

## 7-Day Execution Plan
1. Add `pivot` stage + schema updates
2. Add debugging benchmark suite + loop metrics
3. Scaffold `mycelium-mcp` + `solve_debug` tool
4. Generate first baseline-vs-LEP report
5. Publish one polished demo narrative

---

## Definition of Success (Wedge)
- Developers get a **new** actionable move quickly
- Repeated failed hypotheses are suppressed
- Every answer includes concrete verification
- Benchmarks show measurable delta over baseline
