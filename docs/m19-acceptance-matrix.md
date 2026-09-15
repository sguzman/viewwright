# M19 — Acceptance Matrix

| Case | Expected |
| --- | --- |
| referenced valid `#RRGGBB` token | resolves unchanged |
| referenced malformed color token | validation error |
| unreferenced valid color token | remains legal |
| unreferenced malformed token with `[visual]` | validation error |
| malformed token with no `[visual]` | validation error |
| missing token referenced by visual role | existing validation error |
| lowercase valid hex accepted by current parser | remains accepted |
| unused token exists | no warning/error merely for being unused |
| unusual token name | unchanged by M19 |
| canonical Reader visual palette | unchanged |
| canonical Dependency Workbench palette | unchanged |
| valid concept/egui projection | unchanged |