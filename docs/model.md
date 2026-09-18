# Blueprint Model

> **Historical / non-authoritative:** This is an M0-era design sketch. Candidate vocabularies and future-tense ideas below are not necessarily part of the current accepted schema. Current behavior is established by the implementation, current README and architecture documentation, and accepted milestone authority. The historical content below is intentionally preserved as design provenance.

This document records the initial conceptual model. It is deliberately smaller than a full UI language.

## Resolution stages

ViewWright should distinguish source syntax from the resolved blueprint used by projections.

```text
source TOML
    ↓
parsed document
    ↓
validated document
    ↓
resolved blueprint
```

Validation answers whether the authored document is coherent. Resolution supplies inherited/defaulted values and converts convenient source expressions into explicit typed relationships.

## Screen

A **screen** is the top-level authored interface unit.

Initial fields:

- `id`
- `purpose`
- `density`
- optional design character
- optional dominant object

A screen owns regions, elements, fixtures, and screen-local design intent.

## Region

A **region** is a major compositional area with semantic purpose.

Candidate roles:

- `navigation`
- `primary_content`
- `secondary_content`
- `inspector`
- `commands`
- `status`
- `preview`
- `tools`

A region may express:

- parent region
- semantic role
- importance
- sizing policy
- growth/shrink behavior
- collapse behavior
- scrolling behavior

A region is not merely a rectangle. Its role should survive projection into different viewports and renderers.

## Element

An **element** is a semantically meaningful interface object inside a region.

Initial kinds should stay intentionally small:

- `text`
- `command`
- `search`
- `collection`
- `property_sheet`
- `tree`
- `preview`
- `status`

M0 should not attempt to encode every possible widget.

Elements may later acquire richer semantic families, but new kinds should be justified by authoring value rather than one-to-one parity with toolkit widgets.

## Composition

Composition describes relationships among regions/elements rather than exact coordinates.

Initial vocabulary:

- `row`
- `column`
- `stack`
- `split`
- `overlay`

The semantic model should additionally carry relationships such as:

- primary / secondary
- dominant / supporting
- adjacent
- nested
- fixed / growing
- collapsible
- scrollable

A split can carry a preferred ratio while still permitting backend-appropriate adaptation.

## Size intent

M0 should support a deliberately constrained sizing model:

- fixed logical size, e.g. `240px`
- minimum / maximum size
- proportional growth weight
- preferred split ratio
- intrinsic / content size

The source syntax may remain friendly while the resolved model uses explicit typed variants.

## Importance

Importance is semantic hierarchy, not a color choice.

Initial levels:

- `primary`
- `secondary`
- `tertiary`

Backends and style layers may use importance to influence typography, contrast, spacing, surface treatment, or layout priority.

## Design intent

Some design requirements are meaningful but intentionally underdetermined.

Examples:

```toml
[design]
character = ["quiet", "technical", "crafted"]
dominant = "projects"
avoid = [
  "pill_soup",
  "nested_card_soup",
  "border_everything",
  "uniform_visual_weight",
]
```

These values may guide human review, AI-assisted concept generation, linting, and eventual renderer heuristics.

They should not be discarded merely because M0 cannot mechanically enforce all of them.

## Tokens

Tokens centralize reusable visual values.

Initial token families may include:

- spacing
- typography scale
- corner radii
- border widths
- surface levels

M0 should avoid recreating CSS. Tokens exist to preserve intentional consistency and to prevent arbitrary values from proliferating through screen files.

## Fixtures

A fixture describes a representative screen state suitable for isolated preview.

Examples:

- empty
- populated
- dense
- selected
- loading
- error

Fixtures should eventually bind semantic test data into elements without requiring the entire host application runtime.

M0 only needs enough fixture structure to establish the concept and permit static preview specimens.

## Projection invariants

Every projection must preserve stable semantic IDs wherever feasible.

A projection may add derived implementation structure, but it must not silently reinterpret authorial intent.

When a backend cannot honor an authored constraint, that mismatch should be diagnosable.

## Open questions intentionally deferred

The following are real questions, but M0 should not solve them prematurely:

- complete responsive/breakpoint language
- animation ontology
- accessibility semantics beyond stable roles/labels
- reusable component authoring
- data-binding language
- theme inheritance across applications
- detailed interaction state machines
- bidirectional design-tool import
- backend-independent text measurement
- concept-art round-tripping

The project should earn these abstractions by encountering real screens.
