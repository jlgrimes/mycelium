---
name: mycelium
description: "Isomorphic problem-solving via OpenClaw runtime. Use when: user wants cross-domain mapping (abstract/search/map/synthesize) and solution transfer between domains."
metadata:
  {
    "openclaw":
      {
        "emoji": "🍄"
      }
  }
---

# Mycelium Skill

Mycelium treats domains as skins over shared problem structure.

## Pipeline

1. **Abstract** — strip domain details to structural form
2. **Search** — find cross-domain analogues
3. **Map** — align entities/processes between domains
4. **Synthesize** — produce practical action in user’s original domain

## Runtime target

Use the `mycelium-server` crate with `openclaw-adapter` to route inference through OpenClaw's model runtime (alias-based), not app-level provider keys.

## Local dev

```bash
cargo run -p mycelium-server
# POST http://127.0.0.1:8787/solve
```

## Environment

- `OPENCLAW_BASE_URL` (default `http://127.0.0.1:18789/v1/chat/completions`)
- `OPENCLAW_TOKEN` (optional)
- `MYCELIUM_MODEL` (default `sonnet`)
- `MYCELIUM_USE_STUB=1` for offline stub mode
