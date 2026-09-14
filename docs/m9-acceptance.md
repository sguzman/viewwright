# M9 — Screen Density Acceptance

M9 is accepted when authored screen density survives validation/resolution and produces a real, bounded difference in backend micro-layout without altering major layout or typography.

## Source / model

Required:

- `density = "comfortable"` resolves to a typed Comfortable density;
- `density = "dense"` resolves to a typed Dense density;
- omitted density resolves to Comfortable;
- unknown density values are rejected with useful diagnostics;
- resolved screen semantics no longer require renderers to branch on arbitrary density strings.

## Semantic inspection

Semantic/debug output must expose resolved density.

Concept output must expose resolved density.

## EgUI behavior

Density should affect local rhythm inside the authored screen.

At minimum the egui projection should make the Comfortable and Dense pressure specimens observably different through a small deterministic density policy covering appropriate micro-layout values such as:

- region content inset;
- item spacing;
- control padding / minimum interaction height;
- element-local gaps.

The exact egui values are backend policy, not new TOML syntax.

### Comfortable

Comfortable should preserve breathing room and remain close to the existing ordinary treatment.

### Dense

Dense should fit more local content into the same major region rectangles while remaining legible and usable.

## Major-layout invariance

For the A/B density pressure pair:

- M4 composition rectangles must be identical;
- M4 region rectangles must be identical;
- font sizes must be identical;
- fixture content must be identical;
- palette/surface semantics must be identical;
- collection presentation must be identical;
- only micro-density should differ.

## Canonical regression

Verify exact selectors:

- `project_browser / many_projects` remains comfortable and legible;
- `reader_workspace_visual / reading` remains comfortable and preserves its accepted M6 palette;
- `dependency_workbench / healthy` remains dense and legible;
- `dependency_workbench / advisory` remains dense and list-oriented.

## A/B human QA

Use exact pressure specimen selectors once implemented:

- `density_pressure_comfortable / populated`
- `density_pressure_dense / populated`

Human QA should be able to identify Dense as locally tighter without seeing a different screen structure, typography, or palette.

## Tests

Add meaningful tests covering at least:

- comfortable parsing/resolution;
- dense parsing/resolution;
- omitted density defaults to Comfortable;
- unknown density rejection;
- Project Browser resolves Comfortable;
- visual Reader resolves Comfortable;
- Dependency Workbench resolves Dense;
- semantic/debug output exposes density;
- concept output exposes density;
- backend density policy maps Comfortable and Dense to distinct micro-layout metrics;
- A/B pressure pair has identical M4 layout plans for the same viewport;
- A/B pressure pair has identical type scale / visual profile where applicable;
- accepted M0–M8 regressions remain passing.

## Non-goals

M9 does not require:

- new density values beyond Comfortable and Dense;
- per-region density overrides;
- per-element density overrides;
- responsive density switching;
- global blueprint scaling;
- font-size scaling;
- authored density metric tables;
- a generic spacing system rewrite;
- M4 layout changes;
- virtualization;
- ViewWitness integration.

## Worker hygiene

Any preview/process launched for QA must be closed before completion is reported unless explicitly requested otherwise.
