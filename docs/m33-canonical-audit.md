# M33 — Canonical Audit

Accepted ViewWright sources were reviewed for the intended expectation-export slice.

## Project Browser

`examples/project-browser.toml` provides the baseline linear pressure:

- screen author ID `project_browser`;
- dominant target `projects`;
- root composition `workspace`;
- reachable regions `navigation`, `projects`, `inspector`;
- explicit roles/importances;
- fixed + grow geometry resolved through LayoutPlan;
- reachable elements with stable authored IDs and exact labels.

At 1440×900, region bounds are fully determined by the accepted M4/M21 layout contract. M33 should read those bounds from LayoutPlan rather than duplicating allocation logic.

## Overlay pressure

`specimens/overlay-command-palette-pressure.toml` proves M31 topology can be represented as intended geometry without flattening semantics:

- base workspace regions remain root-reachable;
- floating region `palette_surface` is root-reachable and dominant;
- its planned centered fixed rectangle should export exactly as LayoutPlan reports it;
- no fake horizontal/vertical axis is required in the expectation schema.

## Overflow pressure

`specimens/reader-overflow-pressure.toml` proves M32 intent:

- region `reader` is reachable and dominant;
- `overflow = "scroll_y"` is resolved intent and should export;
- runtime scroll position is not canonical and must not export;
- surrounding region geometry remains LayoutPlan-owned.

## ViewWitness boundary audit

Current ViewWitness authority says:

- basic `Witness` documents observed interface state;
- `Intended` means source/specification-level claims and is not part of a basic witness;
- `author_id` is optional author-supplied identity evidence distinct from witness-local observed node ID;
- the current public API exposes witnesses, capture, diffs, geometry, and egui epistemic assessments, but no canonical expectation document/comparator.

M33 therefore must create a separate ViewWright expectation artifact and must not import/emit a ViewWitness witness as a shortcut.

## Migration result

No accepted ViewWright TOML source requires modification for M33.

If implementation requires source migration, renderer behavior changes, or invented element geometry, stop and report the contradiction.
