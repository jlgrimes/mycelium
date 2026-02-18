# mycelium

Nature-themed, endpoint-agnostic isomorphic problem-solving engine.

## Monorepo layout

- `crates/isomorph-types` — shared schemas
- `crates/isomorph-core` — pure pipeline interfaces
- `crates/isomorph-engine` — orchestration logic (abstract/search/map/synthesize)
- `crates/isomorph-providers` — provider trait + adapters
- `crates/isomorph-server` — API server
- `adapters/openclaw` — OpenClaw-backed provider adapter
- `skills/mycelium` — OpenClaw skill wrapper

## Current status

Scaffold complete. Next step is wiring the OpenClaw adapter to the runtime model interface so the skill can run with existing OpenClaw auth profiles and token routing.
