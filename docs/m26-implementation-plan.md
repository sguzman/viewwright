# M26 — Implementation Plan

1. Re-read M3 property-sheet authority and current model/egui paths.
2. Add trim-based nonblank validation for every authored `PropertySource.name` before constructing resolved properties.
3. Preserve valid property names exactly and preserve values/order/duplicates unchanged.
4. Add focused regressions for empty/whitespace names, exact preservation, duplicate-name legality, empty value legality, and empty property lists.
5. Verify canonical fixtures still resolve and render semantics are unchanged.
6. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
7. No screenshot QA is required if renderer/layout remain untouched.
8. README bookkeeping: record M25 accepted and M26 implementation complete/director audit pending.
9. Comment the M26 issue with evidence, leave it open for director audit, and do not begin M27.