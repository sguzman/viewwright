# M30 — Implementation Plan

1. Re-read M3 fixture-content authority, M16 unused-declaration boundary, M18 global cycle authority, M29 root-reachability authority, and the current fixture resolver.
2. Reuse the existing M29 root-reachable region set rather than introducing another graph model.
3. After a fixture content record resolves its target element, and only when `screen.root` is valid, reject the record if the element's owning region is not root-reachable.
4. Preserve existing missing-element, duplicate-record, exactly-one-family, selected, and payload-kind diagnostics.
5. Add regressions for direct reachable target, nested reachable target, unreachable target, invalid/missing root cascade suppression, omitted content, and unused non-target declarations.
6. Preserve M18 unused-cycle rejection and M29 dominant behavior.
7. Verify accepted canonical fixture-bearing sources resolve unchanged and every fixture content target's owning region is root-reachable.
8. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
9. Do not run screenshot QA if renderer/layout/projection code is untouched and valid canonical output is unchanged.
10. Update README: M29 accepted; M30 implementation complete / director audit pending.
11. Comment the M30 issue with implementation evidence, leave it open for director audit, and do not begin M31.