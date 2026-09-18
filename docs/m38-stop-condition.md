# M38 — Stop Condition

Stop M38 once real renderer output exactly satisfies existing M33 expectations under existing M35 comparison for the required representative canonical cases.

Stop when all of these hold:

- Project Browser / `many_projects` at 1440×900 produces an exact match;
- M31 overlay / `palette_open` at 1440×900 produces an exact match;
- M32 overflow / `long_document` at 1440×900 produces an exact match;
- each Witness comes directly from real AccessKit-enabled `egui::FullOutput` through the pinned ViewWitness adapter;
- each Witness validates;
- each comparison has zero mismatches;
- each comparison has zero evidence gaps;
- each report returns `is_exact_match() == true`;
- no M33 expectation semantics change;
- no M34 identity or renderer behavior change;
- no M35 comparator semantics change;
- no ViewWitness repository change;
- ViewWitness egui remains dev/test-only at the renderer proof boundary;
- `viewwright-compare` remains model-only;
- all M0–M37 regressions remain green.

Do not continue into:

- public verification APIs;
- preview verification UI;
- live observer/network capture;
- external process orchestration;
- CI gating;
- tolerance/fuzzy matching;
- paint/raster evidence;
- responsive layout;
- M39.

If a required case is not an exact match, stop with the real report instead of weakening an accepted layer.
