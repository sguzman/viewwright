# M12 — Acceptance

M12 is accepted when the existing `region.role` field is typed and validated end-to-end without changing accepted visible behavior.

## Required behavior

- add a typed resolved region-role representation;
- support exactly `commands`, `controls`, `navigation`, `primary_content`, `inspector`, and `status`;
- reject unknown authored role strings with a useful validation error;
- `ResolvedRegion.role` is typed rather than an arbitrary `String`;
- semantic/debug projection still exposes roles deterministically;
- concept projection still exposes roles deterministically;
- egui uses the typed role for role-specific behavior instead of string comparison;
- canonical M0–M11 specimens continue resolving and rendering unchanged in intent.

## Primary regression

`reader_workspace_visual / reading` must retain horizontal command regions and the accepted M10 chrome behavior.

`project_browser / many_projects`, `dependency_workbench / healthy`, and the M9 density pressure pair must remain valid.

## Tests

Add focused tests for every supported role and at least one rejected unknown role. Preserve all prior regression tests.

Human visual QA is not inherently required because M12 is intended to have no visible delta; request it only if implementation produces an uncertain visual regression.
