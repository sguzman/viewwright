# M25 — Implementation Plan

1. Re-read M25 authority plus M0/M15 design-intent authority and the current concept projection.
2. Add narrow validation for every `design.character` and `design.avoid` entry using trim-based blankness detection only.
3. Preserve valid strings exactly, including ordering, duplicates, punctuation, Unicode, and intentional surrounding whitespace.
4. Keep omitted/default-empty lists legal and keep `design.dominant` resolution unchanged.
5. Add focused regressions for empty/whitespace entries, exact preservation, duplicate legality, and empty/omitted lists.
6. Verify accepted canonical sources still resolve and concept output for valid sources is unchanged.
7. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
8. No screenshot QA is required if renderer/layout/concept code remains untouched.
9. README publication bookkeeping: record M24 accepted and M25 implementation complete/director audit pending.
10. Comment the M25 issue with implementation evidence, leave it open for director audit, and do not begin M26.