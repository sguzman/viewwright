# M3 — Fixture Content Acceptance

M3 implements the smallest preview-data layer justified by the dependency-workbench pressure specimen.

## Goal

Representative isolated preview state should come from canonical fixture content rather than backend-invented placeholders or fixture-name heuristics.

```text
TOML fixture content
    ↓
validated / resolved fixture content
    ↓
semantic projections
    ↓
recognizable preview state
```

## Required source capability

A fixture may contain content records that reference existing semantic elements.

M3 must support:

- collection items with stable ids and labels
- optional selected collection item id
- ordered property name/value pairs
- simple textual content for compatible semantic elements

## Required resolved capability

By projection time, fixture content must be typed and element references validated.

Renderers should not need to infer payload meaning from arbitrary TOML maps or fixture names.

## Required validation

Diagnose at least:

- missing fixture-content element reference
- duplicate content record for one element in one fixture
- duplicate collection item ids
- selected collection item that does not exist
- collection payload attached to an incompatible element kind
- property payload attached to an incompatible element kind
- text payload attached to an incompatible element kind
- record containing incompatible payload families simultaneously
- unknown authored fields

Existing structural and visual validation must remain intact.

## Projection requirements

For `specimens/dependency-workbench.toml`:

- selecting `healthy` shows the authored healthy dependency items
- `serde` is visibly selected
- the property sheet shows the authored serde metadata
- the status surface shows the authored package/advisory summary
- selecting `advisory` changes the representative collection, selection, properties, and status

The egui backend must not hardcode dependency names or workbench-specific ids.

ASCII may remain primarily structural, but if fixture content is included it must come from resolved fixture content rather than invented examples.

## Existing specimen migration

Where current Project Browser fixture behavior relies on renderer heuristics such as fixture-id substring checks for empty/selection/dense state, M3 should migrate the relevant canonical fixtures to explicit content if doing so is straightforward.

Do not preserve duplicate competing behavior merely for convenience.

The Reader Workspace may remain minimally populated if its current fixture state does not require the new content families. Do not invent rich document data merely to use the new machinery everywhere.

## Runtime rule

Parsing and resolving fixture content occurs before normal frame rendering.

Fixture selection and rendering resolved static content are lightweight UI work.

No host-app data loading, file I/O, network I/O, database access, or other heavy work belongs in the render loop.

## Worker hygiene

If implementation QA launches the preview application or another temporary process/window, the worker must close what it launched before reporting completion unless explicitly instructed to leave it running.

## Explicitly outside M3

- general data binding
- reactive state graphs
- event/action semantics
- editable form schemas
- arbitrary table/grid schemas
- tree-data ontology
- rich text/document content
- persistence
- runtime application integration
- fixture scripting
- application-specific dependency logic
- new visual style vocabulary
- ViewWitness integration

## Acceptance

M3 is accepted when the dependency workbench can switch between at least two representative states whose collection items, selection, property sheet, and status all originate from canonical resolved fixture content, while existing accepted milestones remain intact and the renderer no longer needs fixture-name heuristics for the element families M3 supports.