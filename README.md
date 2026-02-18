# mycelium

Nature-themed, endpoint-agnostic isomorphic problem-solving engine.

## Monorepo layout

- `crates/mycelium-types` — shared schemas
- `crates/mycelium-core` — provider trait
- `crates/mycelium-engine` — orchestration
- `crates/mycelium-providers` — local stub provider
- `crates/mycelium-server` — HTTP API server (`/health`, `/solve`)
- `adapters/openclaw` — real OpenClaw-backed provider
- `skills/mycelium` — OpenClaw skill wrapper docs

## OpenClaw wiring (real)

`openclaw-adapter` calls OpenClaw's Chat Completions endpoint and asks the model to return strict JSON for the Mycelium pipeline fields.

Environment variables:

- `OPENCLAW_BASE_URL` (default: `http://127.0.0.1:18789/v1/chat/completions`)
- `OPENCLAW_TOKEN` (optional bearer token)
- `MYCELIUM_MODEL` (default: `sonnet`)
- `MYCELIUM_BIND` (default: `127.0.0.1:8787`)
- `MYCELIUM_USE_STUB=1` to force local stub provider

## Run

```bash
cargo run -p mycelium-server
curl -X POST http://127.0.0.1:8787/solve \
  -H 'content-type: application/json' \
  -d '{"input":"How do I practice trumpet better?"}'
```

## Notes

- Credentials stay in OpenClaw/runtime env, not frontend apps.
- Model selection is alias-based (`sonnet`, `opus`, etc.) for portability.
