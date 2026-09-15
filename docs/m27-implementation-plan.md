# M27 — Implementation Plan

1. Re-read M5 document-fixture authority and current document resolution/rendering.
2. Add local trim-based blankness validation for every authored `DocumentSource.title`.
3. Preserve valid title text exactly; do not mutate paragraph content.
4. Keep missing-title deserialization behavior and empty paragraph lists unchanged.
5. Add focused regressions for empty/whitespace titles, exact preservation, missing title, empty/whitespace paragraphs, and empty paragraph lists.
6. Verify canonical Reader fixtures resolve unchanged.
7. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
8. No screenshot QA is required if renderer/layout/projection code remains untouched.
9. README publication bookkeeping: record M26 accepted and M27 implementation complete/director audit pending.
10. Comment the M27 issue with implementation evidence, leave it open for director audit, and do not begin M28.
