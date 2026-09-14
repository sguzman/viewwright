# M10 — Acceptance

M10 is accepted when the egui authored projection stops leaking semantic/debug scaffolding while preserving authored element labels in kind-appropriate form.

## Required behavior

- region ids are not automatically painted into authored UI;
- region roles are not automatically painted into authored UI;
- command labels appear once, on the command control itself;
- search, collection, tree, property-sheet, document, and status labels remain available where they identify authored content;
- semantic/debug output still exposes region identity and role;
- concept output remains semantically informative;
- no new source field is introduced.

## Canonical pressure

Human QA should primarily inspect:

- `reader_workspace_visual / reading`

Expected:

- no visible `app_commands / commands`, `library / navigation`, `reader / primary_content`, `inspector / inspector`, or `transport / commands` scaffolding;
- `Open`, `Play / pause`, `Voice`, and `Speed` are not duplicated as text plus button;
- `Search library`, `Contents`, `Document`, `Reading settings`, and `Reading status` remain meaningful authored labels;
- M6 palette, M7 actions, M9 density, M4 geometry, and M5 fixture content remain intact.

Secondary regressions:

- `project_browser / many_projects` remains legible and preserves M8 list/cards distinction;
- `dependency_workbench / healthy` remains dense and list-oriented;
- density pressure specimens remain valid.

## Tests

Add meaningful coverage for:

- semantic/debug region output retaining id + role;
- command label presentation policy not producing a separate pre-label;
- non-command authored labels remaining available;
- accepted M0–M9 regressions.

Avoid brittle pixel-perfect tests.

## Human QA

One screenshot of `reader_workspace_visual / reading` should normally be enough unless it exposes a regression.