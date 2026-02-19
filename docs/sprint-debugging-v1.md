# Sprint Plan — Debugging Wedge V1

## Milestone

Ship a reliable Debugging Wedge demo that beats baseline single-pass quality on targeted cases.

## Track A — Product surface

- [x] Add debugging API entrypoint (`/solve/debug`)
- [x] Enforce response contract with required verification step
- [x] Add concise output mode for quick action (`/solve/debug/concise`)

## Track B — Engine quality

- [x] Add mapping confidence signal for proposed fix paths (staged + debug contract)
- [ ] Add debugging-specific stage prompts for explicit staged provider flow
- [x] Keep robust JSON parsing + extraction fallback in OpenClaw adapter

## Track C — Eval and quality gates

- [x] Create debugging-v1 benchmark suite (10 representative bugs)
- [x] Add eval metric: `actionability_score` (`0..5`)
- [x] Add eval metric: `verification_presence` (boolean)
- [x] Compare baseline vs staged and commit report artifact
- [ ] Drive measurable delta target (+10% actionability) with non-stub providers

## Track D — SaaS readiness

- [ ] Add ICP + JTBD go-to-market summary into dedicated GTM doc
- [ ] Add one-page pitch/demo script for this wedge
- [ ] Define first 100 users channels (dev-focused)

## Definition of done (current)

- [x] All tests pass in CI/local
- [x] Benchmark report committed (`reports/debugging-v1-baseline-vs-staged.txt`)
- [x] Reproducible debug demo commands documented in README

## Next slice

1. Wire staged debug prompts in provider/runtime path (not only route contract shaping).
2. Run eval suite with non-stub provider and capture benchmark delta.
3. Ship GTM plan doc (`docs/gtm-debugging-v1.md`) and link from README.
