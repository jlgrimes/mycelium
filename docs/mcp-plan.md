# MCP Plan for Mycelium

Yes, Mycelium should be exposed as MCP so any MCP-compatible client can use it (not just OpenClaw).

## Initial MCP tools

1. `mycelium.solve_debug`
- Input: `{ problem: string, context?: string }`
- Output: `{ abstract_shape, cross_domain_matches, mapping, synthesis }`

2. `mycelium.solve_general`
- Input: `{ problem: string }`
- Output: same schema

3. `mycelium.eval_debug_case`
- Input: `{ case_id: string }`
- Output: eval scores + report snippet

## Runtime
- MCP server wraps `mycelium-engine`
- OpenClaw adapter remains the model backend
- Keep provider credentials server-side only

## Rollout
1. Ship `/solve/debug` HTTP path (done in progress)
2. Add MCP server crate (`mycelium-mcp`)
3. Implement first 2 tools
4. Publish config examples for Claude Desktop / Cursor / VSCode MCP clients
