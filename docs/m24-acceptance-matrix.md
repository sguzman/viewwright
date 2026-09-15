# M24 — Acceptance Matrix

| Case | Expected |
| --- | --- |
| spacing token name `""` | reject |
| spacing token whitespace-only name | reject |
| corner token blank/whitespace name | reject |
| color token blank/whitespace name | reject |
| valid unused named token | accept |
| valid name with surrounding whitespace plus exact matching reference | accept unchanged |
| existing missing-token reference | reject as before |
| malformed color-token value | reject as M19 requires |
| canonical specimens | unchanged |