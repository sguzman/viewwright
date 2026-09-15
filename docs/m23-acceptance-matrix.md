# M23 — Acceptance Matrix

| Case | Expected |
|---|---|
| collection label `""` | reject |
| collection label whitespace-only | reject |
| tree label `""` | reject |
| tree label whitespace-only | reject |
| valid label with surrounding spaces | accept and preserve exactly |
| duplicate labels with distinct IDs | accept |
| missing required label field | TOML parse error |
| canonical fixtures | unchanged |
| renderer/layout | unchanged |