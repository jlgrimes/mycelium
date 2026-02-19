# Sprint Plan — Debugging Wedge V1

## Milestone
Ship a reliable Debugging Wedge demo that beats baseline single-pass quality on targeted cases.

## Track A — Product surface
- [ ] Add `mycelium-debug` mode/entrypoint in server API (`/solve/debug`)
- [ ] Enforce response contract with required verification step
- [ ] Add concise output mode for quick action (short fix plan)

## Track B — Engine quality
- [ ] Add debugging-specific stage prompts (abstract/search/map/synthesize)
- [ ] Add mapping confidence score (0-1) for each proposed fix path
- [ ] Add lightweight fallback when JSON parsing fails

## Track C — Eval and quality gates
- [ ] Create `benchmarks/debugging-v1.json` with 10 representative bugs
- [ ] Add eval metric: `actionability_score` (0-5)
- [ ] Add eval metric: `verification_presence` (boolean)
- [ ] Compare baseline vs staged; target +10% actionability

## Track D — SaaS readiness
- [ ] Add ICP + JTBD into README/docs
- [ ] Add one-page pitch/demo script for this wedge
- [ ] Define first 100 users channels (dev-focused)

## Definition of done
- All tests pass
- Benchmark report committed
- One reproducible demo command documented
