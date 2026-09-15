# M19 — Implementation Plan

1. Inspect `ColorTokensSource`, `parse_color`, `color_ref`, and `resolve_visual` in `viewwright-model` before editing.
2. Add deterministic validation for every authored color-token literal.
3. Preserve existing visual-role lookup behavior and concrete palette resolution.
4. Avoid introducing duplicate malformed-token diagnostics where a narrow implementation can preserve current referenced-token behavior.
5. Add focused tests for malformed unreferenced tokens both with and without `[visual]`, valid unused tokens, existing malformed referenced tokens, existing missing references, and deterministic ordering when multiple unused malformed tokens are authored.
6. Re-run canonical Reader visual and Dependency Workbench resolution and all existing M2/M6 visual tests.
7. Do not touch layout or egui unless compiler-forced mechanical changes occur; none are expected.
8. During README publication bookkeeping, preserve `# 🟩 ViewWright`, record M18 as accepted, and record M19 as implementation complete / director audit pending.
9. Comment Issue #20, commit, push, leave the issue open for director audit, and stop before M20.

Required checks:

- `cargo fmt --all`
- `cargo test`
- `cargo check -p viewwright-preview`
- `git diff --check`
