# Mycelium Wedge #1 — Debugging Loops for Software Engineers

## ICP
- Solo devs and small teams shipping TypeScript/JavaScript backends/frontends
- Pain: recurring bug loops, state mutation bugs, race conditions, and flaky fixes
- Context: they can describe the bug, but struggle to map to a robust fix pattern quickly

## Core JTBD
"When my code is failing in a messy way, help me identify the underlying problem shape and give me an immediately actionable fix plan."

## Product promise
Mycelium maps a concrete bug report into a cross-domain structure and returns:
1. Abstract problem shape
2. Cross-domain matches (at least 3)
3. Explicit mapping back to code entities
4. Actionable fix sequence with verification checks

## Killer workflow (v1)
Input:
- Freeform debugging problem (stack traces, symptoms, snippets)

Output:
- `ABSTRACT` (domain-agnostic shape)
- `SEARCH` (cross-domain isomorphic patterns)
- `MAP` (symbol-to-symbol mapping back to code)
- `SYNTHESIZE` (step-by-step fix + tests to run)

## Non-goals (this wedge)
- General life coaching
- Broad strategy brainstorming
- Non-actionable analogy essays
- Full IDE integration (later)

## Acceptance criteria (v1)
- 10/10 debug benchmark prompts produce concrete fix steps
- Every answer includes at least one verification action (test/log/assertion)
- Median response should suggest a first action in < 30 seconds runtime

## Why this wedge fits SAAS-FRAMEWORK
- Specific problem with immediate value
- UI/flow can be optimized around real debugging behavior
- Architecture stays modular (providers/pipeline/eval isolated)
- Quality can be enforced through benchmark/e2e cases
