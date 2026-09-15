# M15 — Acceptance

M15 is accepted when `design.dominant` is resolved into typed semantic identity rather than surviving as an arbitrary source string.

## Required behavior

- `ResolvedBlueprint.design` no longer uses `DesignSource` directly;
- a resolved design type preserves `character` and `avoid` text unchanged;
- `dominant = <region id>` resolves to a typed region target;
- `dominant = <element id>` resolves to a typed element target;
- omitted `dominant` resolves to `None`;
- missing references still fail with a useful `design.dominant` diagnostic;
- composition, fixture, action, and arbitrary identifiers are not accepted as dominant targets;
- concept projection continues exposing the authored dominant id deterministically;
- accepted canonical screens preserve all existing behavior and output intent.

## Regression pressure

At minimum preserve:

- `project_browser / many_projects`;
- `reader_workspace_visual / reading`;
- `dependency_workbench / healthy`;
- the M9 density pressure pair.

## Tests

Add focused coverage for region target resolution, element target resolution, missing target rejection, omitted dominant, freeform design metadata preservation, and concept-output continuity.

M15 has no intended visible delta, so human screenshot QA is not inherently required.
