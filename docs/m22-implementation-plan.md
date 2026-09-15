# M22 — Implementation Plan

1. Add focused `fixture.state` nonblank validation during resolution.
2. Use `trim().is_empty()` only for validity detection; preserve accepted source text exactly.
3. Keep `FixtureSource.state` and `ResolvedFixture.state` as required `String` values.
4. Add tests for empty, whitespace-only, exact preservation, and missing-field behavior.
5. Verify accepted canonical fixtures resolve unchanged.
6. Do not touch renderer/layout/projection code unless an unexpected blocker appears.
7. Update README bookkeeping: M21 accepted; M22 implementation complete / director audit pending.
8. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
9. Comment the M22 issue with implementation evidence and leave it open for director audit.