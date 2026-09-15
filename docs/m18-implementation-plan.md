# M18 Implementation Plan

1. Inspect the existing composition cycle helper and tests in `viewwright-model`.
2. Preserve the existing resolved composition representation.
3. Change cycle validation so deterministic traversal covers every resolved composition, not only the root-reachable set.
4. Retain the explicit `screen.root` contract and M16 ownership pass unchanged unless a tiny mechanical adjustment is required.
5. Add focused regressions for unreachable two-node/longer cycles and unreachable self-cycle.
6. Retain/verify a reachable-cycle regression.
7. Retain/verify that unused acyclic compositions and regions remain allowed.
8. Run canonical compatibility plus existing M4/M16/M17 regressions.
9. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
10. Update README bookkeeping so M17 is recorded accepted and M18 is implementation complete; director audit pending.
11. Comment Issue #19 with implementation evidence, commit/push, leave issue open for director audit, and stop before M19.

No preview QA is expected because M18 has no intended visible delta.
