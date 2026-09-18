# M38 — Acceptance Matrix

| Requirement | Required evidence |
|---|---|
| Same source drives intent and observation | one resolved blueprint per case feeds both branches |
| Same logical viewport | expectation and capture both use 1440×900 |
| Real intended artifact | `build_expectation` returns M33 expectation |
| Real renderer output | `viewwright_egui::show` produces the tested frame |
| Real observation adapter | `witness_from_egui_output` consumes that actual `FullOutput` |
| Witness validity | `validation_issues()` empty |
| Real comparator | `viewwright_compare::compare` consumes typed expectation + Witness |
| Exact success | zero mismatches, zero evidence gaps, `is_exact_match()` true |
| Ordinary geometry | Project Browser / `many_projects` passes |
| Overlay geometry | M31 / `palette_open` passes |
| Overflow structure | M32 / `long_document` passes |
| Dependency boundary | ViewWitness egui remains test/dev-only; compare remains model-only |
| No semantic weakening | no M33/M34/M35 contract changes |
| No runtime scope | no server, network, preview loop, CI gate |
| No responsive scope | responsive roadmap untouched |
