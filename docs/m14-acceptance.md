# M14 — Acceptance

M14 is accepted when visible element labels are guaranteed to be authored rather than synthesized from internal ids.

## Required behavior

- every current element kind requires an explicit authored `label`;
- missing labels fail validation with an element-specific diagnostic;
- empty or whitespace-only labels fail validation;
- successful resolution never derives visible label text from `element.id`;
- `ResolvedElement.label` remains a non-optional `String` containing authored text;
- semantic/debug, ASCII, concept, and egui projections continue consuming the resolved authored label;
- accepted canonical specimens resolve and render with no intended visible change.

## Regression pressure

At minimum preserve:

- `project_browser / many_projects`;
- `reader_workspace_visual / reading`;
- `dependency_workbench / healthy`;
- the M9 density pressure pair.

## Tests

Add focused coverage for missing and blank labels, authored-label preservation, and canonical specimen compatibility. Preserve all M0–M13 regression tests.

Human visual QA is not inherently required because M14 is intended to produce no visible delta.