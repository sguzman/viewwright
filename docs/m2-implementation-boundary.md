# M2 Implementation Boundary

The implementation milestone should change only what is necessary to support the M2 visual-authoring specimen.

Expected implementation areas:

- source/resolved visual model
- visual token validation and resolution
- region surface resolution
- unknown-field diagnostics
- concept-specification projection
- egui consumption of resolved visual semantics
- preview support for the visual specimen
- tests

Unexpected scope that should trigger review before implementation continues:

- general theme engines
- arbitrary style inheritance
- selectors/classes
- image generation services
- renderer-specific properties in source TOML
- font asset pipelines
- application-specific Reader Workspace styling logic

The human should not be used as a commit/push courier. Codex owns implementation publication; ChatGPT owns architectural review and milestone acceptance.
