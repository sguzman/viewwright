# M20 — Acceptance Matrix

| Case | Expected |
| --- | --- |
| canonical Project Browser purpose | unchanged |
| canonical Reader purposes | unchanged |
| canonical Dependency Workbench purpose | unchanged |
| canonical density pressure purposes | unchanged |
| `purpose = ""` | validation error |
| whitespace-only purpose | validation error |
| nonblank purpose with leading/trailing whitespace | accepted and preserved exactly |
| missing purpose field | existing parse failure preserved |
| valid semantic/debug projection | unchanged |
| valid concept specification | unchanged |
| fixture state / labels / design strings | unchanged by M20 |
