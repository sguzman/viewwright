# M24 — Implementation Plan

1. Add a small token-name validation path for `tokens.spacing`, `tokens.corners`, and `tokens.color`.
2. For each map, validate names in deterministic sorted-key order with `name.trim().is_empty()`.
3. Preserve every valid key exactly; do not normalize references.
4. Add focused tests for empty/whitespace names in all three families, exact valid-name preservation/reference behavior, and valid unused tokens.
5. Keep M19 color-value validation and existing missing-reference tests green.
6. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
7. No screenshot QA is required if renderer/layout/projections remain untouched.
8. README publication bookkeeping: record M23 accepted and M24 implementation complete/director audit pending.