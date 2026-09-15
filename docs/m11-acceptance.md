# M11 — Acceptance

M11 is accepted when canonical Search controls retain typed text across normal egui redraws while remaining explicitly outside application/business semantics.

## Required behavior

- Search text persists across frames while the same specimen/fixture remains selected;
- separate Search elements have independent values;
- preview specimen/fixture changes do not leak stale search text into the newly selected design state;
- Search labels remain authored and visible;
- no canonical TOML field is added;
- no fixture content is added for Search text;
- no filtering or search execution occurs;
- no search-change application event is added;
- `ResolvedBlueprint` remains free of mutable runtime query state;
- M0–M10 behavior remains accepted.

## Primary pressure

Human QA should primarily inspect:

- `project_browser / many_projects`

Type a short string such as `abc` into `Search projects`.

Expected:

- the typed string remains visible while frames continue rendering;
- the surrounding Project Browser remains unchanged;
- project cards are not filtered;
- navigation/cards/inspector remain M8-authoritative.

A screenshot showing a successfully retained multi-character query is normally sufficient human proof because entering multiple characters necessarily spans redraws.

## Secondary regressions

Briefly preserve:

- `reader_workspace_visual / reading`;
- `dependency_workbench / healthy`;
- `density_pressure_comfortable / populated`;
- `density_pressure_dense / populated`.

## Tests

Add focused automated coverage for:

- renderer state retaining one Search value across repeated accesses/frames;
- independent values for two element ids;
- explicit clearing/rebinding behavior used on specimen/fixture changes;
- Search state not entering resolved model/fixture semantics;
- existing M7 actions, M8 collection presentation, M9 density, M10 label/chrome behavior, and all prior regressions.

Avoid pixel-perfect GUI tests.
