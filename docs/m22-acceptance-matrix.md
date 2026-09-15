# M22 — Acceptance Matrix

| Case | Expected |
|---|---|
| `state = ""` | reject |
| whitespace-only state | reject |
| nonblank ordinary state | accept |
| nonblank state with intentional surrounding whitespace | accept and preserve exactly |
| missing `state` field | TOML deserialization error |
| canonical fixtures | unchanged |
| renderer/layout | unchanged |

No state vocabulary or grammar is added.