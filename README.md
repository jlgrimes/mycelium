# mycelium

Nature-themed, endpoint-agnostic isomorphic problem-solving engine.

## Monorepo layout

- `crates/mycelium-types` — shared schemas
- `crates/mycelium-core` — provider trait
- `crates/mycelium-engine` — orchestration
- `crates/mycelium-providers` — local stub provider
- `crates/mycelium-server` — HTTP API server (`/health`, `/solve`, `/solve/debug`, `/solve/debug/concise`)
- `crates/mycelium-eval` — evaluation harness & benchmarks
- `adapters/openclaw` — real OpenClaw-backed provider
- `skills/mycelium` — OpenClaw skill wrapper docs

## OpenClaw wiring (real)

`openclaw-adapter` calls OpenClaw's Chat Completions endpoint and asks the model to return strict JSON for the Mycelium pipeline fields.

Environment variables:

- `OPENCLAW_BASE_URL` (default: `http://127.0.0.1:18789/v1/chat/completions`)
- `OPENCLAW_TOKEN` (optional bearer token; sent as `Authorization: Bearer ...`)
- `OPENCLAW_AUTH_HEADER` (optional override, either `Header-Name: value` or raw auth value for `Authorization`)
- `OPENCLAW_TIMEOUT_MS` (default: `30000`)
- `OPENCLAW_CONNECT_TIMEOUT_MS` (default: `5000`)
- `OPENCLAW_MAX_RETRIES` (default: `2`; total attempts = retries + 1)
- `OPENCLAW_RETRY_BASE_MS` (default: `250`; exponential backoff base)
- `OPENCLAW_RETRY_MAX_MS` (default: `5000`; backoff cap)
- `MYCELIUM_MODEL` (default: `sonnet`)
- Responses are normalized + quality-gated (non-empty fields and at least 3 cross-domain matches)
- `MYCELIUM_BIND` (default: `127.0.0.1:8787`)
- `MYCELIUM_USE_STUB=1` to force local stub provider

## Run

```bash
cargo run -p mycelium-server
curl -X POST http://127.0.0.1:8787/solve \
  -H 'content-type: application/json' \
  -d '{"input":"How do I practice trumpet better?"}'

# Debugging-focused mode (Loop Escape Protocol prompting)
curl -X POST http://127.0.0.1:8787/solve/debug \
  -H 'content-type: application/json' \
  -d '{"input":"My reducer state is wrong after chained updates"}'

# Concise debugging mode for fast operator action
curl -X POST http://127.0.0.1:8787/solve/debug/concise \
  -H 'content-type: application/json' \
  -d '{"input":"My reducer state is wrong after chained updates"}'
```

### Debug contract

`/solve/debug` and `/solve/debug/concise` enforce output framing across the four pipeline stages:

- `ABSTRACT:` problem shape
- `SEARCH:` cross-domain analog matches
- `MAP:` domain mapping + mapping confidence (`high|medium|low`)
- `SYNTHESIZE:` fix steps + verification + fallback pivot

`/solve/debug/concise` is intentionally terse and returns three SEARCH items max.

## Eval harness

Run benchmark suites (baseline vs staged) and score response quality + actionability:

```bash
cargo run -p mycelium-eval
cargo run -p mycelium-eval -- --suite seed
cargo run -p mycelium-eval -- --suite debugging-v1
cargo run -p mycelium-eval -- --suite debugging-v1 --list
cargo run -p mycelium-eval -- --suite debugging-v1 --filter debug-null-pointer,debug-memory-leak
```

Available suites:

- `seed` (20 general cross-domain reasoning cases)
- `debugging-v1` (10 debugging-loop wedge cases)

Report fields include:

- mean score
- mean actionability (`0..5`)
- verification presence rate

Uses `StubProvider` by default. Swap providers in `main.rs` to evaluate against real endpoints.

Run tests: `cargo test -p mycelium-eval`

## Development checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

CI runs the same checks on pushes/PRs via `.github/workflows/ci.yml`.

## Current Product Focus

- Wedge #1: Debugging loops for software engineers
- Spec: `docs/wedge-debugging.md`
- Loop Escape Protocol: `docs/loop-escape-protocol.md`
- Sprint plan: `docs/sprint-debugging-v1.md`
- Big game plan: `docs/game-plan.md`
- MCP direction: `docs/mcp-plan.md`

## Notes

- Credentials stay in OpenClaw/runtime env, not frontend apps.
- Model selection is alias-based (`sonnet`, `opus`, etc.) for portability.
