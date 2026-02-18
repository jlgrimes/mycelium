---
name: mycelium
description: "Isomorphic problem-solving via OpenClaw runtime. Use when: user wants cross-domain mapping (abstract/search/map/synthesize) and solution transfer between domains."
metadata:
  {
    "openclaw":
      {
        "emoji": "🍄",
        "requires": { "bins": ["cargo"] }
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

This skill is intended to call the OpenClaw-backed adapter once wired.

## Local dev

```bash
cargo run -p isomorph-server
# POST http://127.0.0.1:8787/solve
```

## Notes

- Keep provider credentials in OpenClaw, not app-level env files.
- Use model aliases (`sonnet`, `opus`, `gemini-flash`) rather than provider-specific hardcoding.
