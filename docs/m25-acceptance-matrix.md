# M25 — Acceptance Matrix

| Source case | Expected result |
|---|---|
| `character = []` | valid |
| `avoid = []` | valid |
| `character = [""]` | reject |
| `character = ["   "]` | reject |
| `avoid = [""]` | reject |
| `avoid = ["\t"]` | reject |
| `character = ["  quiet  "]` | valid, exact text preserved |
| duplicate nonblank character entries | valid |
| duplicate nonblank avoid entries | valid |
| omitted design block | valid/default-empty as before |
| valid `design.dominant` | unchanged typed resolution |
| invalid `design.dominant` | unchanged missing-reference diagnostic |
| accepted canonical sources | unchanged |
