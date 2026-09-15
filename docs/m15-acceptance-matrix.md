# M15 — Acceptance Matrix

| Concern | Required M15 evidence |
| --- | --- |
| Source compatibility | Existing `[design] dominant = "..."` syntax unchanged |
| Resolved boundary | `ResolvedBlueprint.design` no longer stores `DesignSource` directly |
| Region dominance | Region id resolves to typed region target |
| Element dominance | Element id resolves to typed element target |
| Omission | Missing `dominant` resolves to `None` |
| Invalid target | Missing/non-region/non-element target rejected clearly |
| Freeform metadata | `character` and `avoid` preserved unchanged |
| Projection | Concept output still emits deterministic dominant id |
| Canonical compatibility | Project Browser, Reader, Dependency Workbench, density pair resolve unchanged |
| Visual behavior | No intended egui/layout/style delta |
| Regression | M0–M14 tests remain passing |
| Scope | No dominance styling/layout behavior and no M16 work |
