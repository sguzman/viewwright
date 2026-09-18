# M38 — Acceptance

M38 is accepted only when the existing M33, M34, M35, and M37 contracts compose exactly against real renderer output.

## Required acceptance evidence

For each required canonical case:

- parse and resolve the existing source;
- build a real M33 `ViewWrightExpectation` at 1440×900;
- render the same `ResolvedBlueprint` through `viewwright_egui::show` in an AccessKit-enabled egui context with the same 1440×900 logical screen rectangle;
- convert the resulting real `egui::FullOutput` directly with the pinned ViewWitness `witness_from_egui_output`;
- require the Witness to validate;
- compare the expectation and Witness with `viewwright_compare::compare`;
- require zero mismatches;
- require zero evidence gaps;
- require `is_exact_match()` to be true.

Required cases:

| Source | Fixture | Pressure |
|---|---|---|
| `examples/project-browser.toml` | `many_projects` | ordinary split |
| `specimens/overlay-command-palette-pressure.toml` | `palette_open` | M31 overlay |
| `specimens/reader-overflow-pressure.toml` | `long_document` | M32 scroll_y |

## Dependency acceptance

- ViewWitness remains pinned to `f1930ab2a70175c46d12dd1e61501c3b4ae09408`.
- The `egui` ViewWitness feature remains dev/test-only at the existing renderer proof boundary.
- `viewwright-compare` remains model-only in its normal dependency graph.
- No observer, showcase, server, network, or eframe ViewWitness feature is enabled for M38.
- No ViewWitness repository change occurs.

## Behavioral acceptance

M38 is not authorized to change visible UI behavior.

Existing M37 renderer/accessibility regressions and all accepted M0–M37 behavior must remain green.

Human screenshot QA is not required if the implementation is truly test/dev-only and makes no renderer behavior change. Any visible renderer change reopens the human-QA requirement and must be justified rather than smuggled into M38.

## Failure rule

If any real case yields mismatch or evidence gap, stop and report the exact `ComparisonReport`.

Do not weaken an accepted contract to force green.
