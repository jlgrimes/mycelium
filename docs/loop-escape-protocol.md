# Loop Escape Protocol (LEP)

Mycelium core principle: eliminate reasoning loops by pivoting to isomorphic problem frames.

## Why
Developers get stuck when they retry inside the same failing frame.
LEP enforces a frame shift before repeated hypothesis retries.

## Stages
1. Detect loop signal
2. Abstract current problem shape
3. Search isomorphic problem classes
4. Pivot to best adjacent frame
5. Map solution pattern back to source code context
6. Synthesize fix steps + verification checks

## Developer-facing guarantees
- No output without at least one verification step
- Explicit pivot rationale (why this frame)
- Fallback pivot path when first mapping fails

## Planned metrics
- loop_escape_rate
- repeat_failure_suppression
- time_to_first_new_action
- verification_presence
