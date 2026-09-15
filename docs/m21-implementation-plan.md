# M21 — Implementation Plan

1. Re-read M4 authority and current `viewwright-layout` allocation helpers.
2. Keep source/resolved model unchanged.
3. Correct region growth-weight projection so positive `ResolvedRegion.grow` participates regardless of same-axis fixed width/height.
4. Preserve fixed-size reservation and existing `main = fixed + proportional growth share` allocation structure.
5. Add focused deterministic horizontal and vertical fixed+grow tests with exact expected rectangles.
6. Keep regression coverage for fixed-only, grow-only, cross-axis fill, nested composition growth, gaps/padding, canonical Reader, Dependency Workbench, Project Browser, and density pair.
7. Do not alter egui or `LayoutPlan` API unless an unexpected blocker appears; if it does, stop and report before broadening scope.
8. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
9. No human screenshot QA is expected because canonical geometry should be unchanged.
10. Update README bookkeeping: M20 accepted; M21 implementation complete/director audit pending.
11. Comment Issue #22 with implementation evidence, leave it open for director audit, commit/push, ensure clean/synced, and stop before M22.