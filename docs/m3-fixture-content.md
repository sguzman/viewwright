# M3 — Fixture-Backed Semantic Preview Content

M0 proved the basic source → resolved → projection pipeline. M1 proved nested composition. M2 proved that one coherent visual profile can survive into concept specification and egui.

M3 addresses the next place where authorial intent currently disappears: **representative element content**.

The current preview can identify an element as a collection, property sheet, status surface, tree, command, or document, but the backend still invents much of what appears inside those elements. That produces placeholder output such as `Item 1`, `tree element`, and `Representative properties`, and fixture behavior is partly inferred from fixture-id substrings.

That is acceptable for an embryo, but it is not a durable authoring contract.

## Goal

Allow fixtures to provide small, typed, static representative content to semantic elements for isolated preview.

The intended authority flow is:

```text
canonical TOML
    ↓
source blueprint + fixture content
    ↓
validation / resolution
    ↓
resolved blueprint + resolved fixture content
    ├── ASCII / semantic inspection
    └── egui preview
```

Fixture content is **preview/test data**, not application runtime state and not a general data-binding language.

## Pressure specimen

`specimens/dependency-workbench.toml` describes a dense dependency-inspection workspace using only existing structural and visual concepts.

The screen requires representative content for:

- a collection of dependencies
- a selected dependency
- a property sheet for the selected dependency
- a status line

Without fixture-backed content, the backend has to invent those details itself.

## Proposed source shape

A fixture may own zero or more content records:

```toml
[[fixture]]
id = "healthy"
state = "ready"

[[fixture.content]]
element = "packages"
items = [
  { id = "serde", label = "serde 1.0.210" },
  { id = "toml", label = "toml 0.8.19" },
]
selected = "serde"

[[fixture.content]]
element = "package_details"
properties = [
  { name = "Version", value = "1.0.210" },
  { name = "Scope", value = "direct" },
]

[[fixture.content]]
element = "workspace_status"
text = "24 packages · 0 advisories"
```

The element reference supplies the semantic kind. The content record should not repeat toolkit/widget types.

## Initial content families

M3 should support only what the pressure specimen earns.

### Collection content

- ordered `{ id, label }` items
- optional selected item id

This is sufficient for representative list-like collections.

M3 does **not** need tables, arbitrary columns, cell schemas, sorting, filtering, virtualization, or runtime selection state.

### Property-sheet content

- ordered `{ name, value }` pairs

This is representative metadata, not a form schema or editable settings system.

### Textual content

A simple `text` payload may support semantic status/text surfaces where the authored element kind makes that relationship coherent.

M3 does not need rich text, markup, localization, or arbitrary documents.

## Validation

Fixture content should be validated before projection.

Diagnose at least:

- content referencing a missing element
- duplicate content records for the same element inside one fixture
- collection content attached to an incompatible element kind
- property content attached to an incompatible element kind
- textual content attached to an incompatible element kind
- selected collection id that does not exist in that content record
- duplicate collection item ids
- malformed records containing incompatible payload families at once
- unknown authored fields under the existing strict-source rule

Errors should identify both fixture and element where practical.

## Resolved model

Renderers should consume typed resolved fixture content rather than repeatedly interpret source tables.

A reasonable conceptual model is similar to:

```text
ResolvedFixtureContent
  Collection { element, items, selected }
  Properties { element, properties }
  Text { element, text }
```

The exact Rust representation is an implementation choice.

Do not introduce egui types into the model.

## Projection behavior

When the selected fixture provides content for an element, the egui renderer should use that content instead of inventing placeholder data.

For the new pressure specimen this should mean:

- dependency items come from the fixture
- selected dependency is visibly selected using existing visual semantics
- property values come from the fixture
- status text comes from the fixture

Existing specimens may be explicitly migrated to fixture content where doing so removes current renderer heuristics cleanly.

The long-term direction is to eliminate fixture-id substring behavior such as `contains("empty")`, `contains("selection")`, and `contains("dense")` from semantic rendering.

M3 should remove such heuristics for the element families it now models rather than preserve two competing fixture systems.

## Runtime boundary

Fixture parsing and resolution happen before the normal egui frame loop.

Selecting a resolved fixture in the preview may be lightweight UI state. Rendering static resolved fixture content is normal immediate-mode work.

Do not introduce heavy parsing, file I/O, or application-data loading into the render thread.

## Explicit non-goals

M3 is not:

- arbitrary application data binding
- reactive state management
- event/action wiring
- editable forms
- persistence
- a table/grid engine
- a generalized tree-data format
- runtime host-app integration
- networking
- database access
- fixture scripting
- reusable component authoring
- responsive-layout work
- additional visual-style vocabulary
- ViewWitness integration

## Stop condition

Stop M3 once the dependency-workbench specimen can render representative collection, selection, properties, and status from canonical fixture content without renderer-invented placeholder data or fixture-name heuristics for those supported families.

Any richer preview-data abstraction must be earned by another real specimen.