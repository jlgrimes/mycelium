# Loop Escape Protocol (LEP)

Mycelium core principle: stop repeated failed hypotheses by forcing a controlled frame pivot before retrying fixes.

## Why

Developers get stuck when they keep searching inside the same failing frame.
LEP enforces a shift to an adjacent isomorphic frame, then maps that pattern back to code with explicit verification.

## Protocol stages

1. **Detect loop signal**
   - Recognize repeated failure pattern (same symptom, same attempted fix family).
2. **Abstract problem shape**
   - Represent the bug in domain-agnostic terms (state drift, contention, stale read, etc.).
3. **Search isomorphic frames**
   - Pull analogous systems with similar constraints and failure dynamics.
4. **Pivot**
   - Select the best adjacent frame and state why it is better than the current stuck frame.
5. **Map back to code**
   - Map source entities to target entities and include confidence on each mapping path.
6. **Synthesize action**
   - Produce concrete fix steps, verification steps, and a fallback pivot if the first path fails.

## Current implementation mapping

The debug API modes currently enforce LEP-shaped output:

- `POST /solve/debug`
- `POST /solve/debug/concise`

Required output framing:

- `ABSTRACT:` problem shape
- `SEARCH:` cross-domain analogs (concise mode keeps max 3)
- `MAP:` explicit mapping + confidence signal (`high|medium|low`)
- `SYNTHESIZE:` fix steps + verification + fallback pivot

## Developer-facing guarantees

- No debug output without verification guidance.
- Explicit fallback pivot path is always present.
- Mapping confidence is surfaced for staged/contracted flows.
- Concise mode prioritizes first actionable move and short operator flow.

## Metrics

Implemented in eval/reporting:

- `verification_presence`
- `actionability_score` (`0..5`)

Planned follow-ons:

- `loop_escape_rate`
- `repeat_failure_suppression`
- `time_to_first_new_action`

## Source of truth docs

- Wedge spec: `docs/wedge-debugging.md`
- Sprint plan: `docs/sprint-debugging-v1.md`
- Game plan: `docs/game-plan.md`
- MCP plan: `docs/mcp-plan.md`
