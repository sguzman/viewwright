# M26 — Acceptance Matrix

| Case | Expected |
|---|---|
| `name = ""` | reject |
| `name = "   "` | reject |
| tab/mixed-whitespace name | reject |
| `name = "Status"` | accept unchanged |
| `name = "  Status  "` | accept and preserve exactly |
| duplicate nonblank names | accept |
| `value = ""` with nonblank name | accept |
| empty `properties = []` | accept |
| canonical property fixtures | resolve unchanged |

Diagnostics should identify fixture context and property position/name where practical. Validation must not alter valid resolved output.