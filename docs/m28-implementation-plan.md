# M28 — Implementation Plan

1. Re-read M28 authority plus M3 fixture-content authority and current status rendering.
2. Add local trim-based nonblank validation for present status `text` payloads during fixture resolution.
3. Preserve valid status strings exactly.
4. Keep omission of status content records legal.
5. Preserve fixture-family exclusivity and status-kind compatibility.
6. Add focused regressions for empty/whitespace rejection, exact preservation, omission, and boundaries against property values/document paragraphs/command reasons.
7. Verify accepted Reader and Dependency Workbench status fixtures remain unchanged.
8. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
9. No screenshot QA is required if renderer/layout/projection code remains untouched.
10. README publication bookkeeping: record M27 accepted and M28 implementation complete/director audit pending.
11. Comment the M28 issue with implementation evidence, leave it open for director audit, and do not begin M29.