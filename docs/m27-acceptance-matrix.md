# M27 — Acceptance Matrix

| Case | Expected |
|---|---|
| `title = ""` | reject |
| whitespace-only title | reject |
| valid title | accept unchanged |
| valid title with surrounding whitespace | accept and preserve exactly |
| missing title field | TOML parse/deserialization failure |
| `paragraphs = []` | accept |
| paragraph `""` | accept unchanged |
| whitespace-only paragraph | accept unchanged |
| canonical loaded Reader document | accept unchanged |
| canonical empty Reader document titled `No document loaded` | accept unchanged |

No renderer/layout/projection delta is expected for valid sources.
