# M8 — Collection Presentation Acceptance

M8 is accepted when authored collection presentation survives parsing, validation, resolution, semantic inspection, and egui projection without introducing a generic responsive/component system.

## Source / model

Required:

- `presentation = "list"` resolves to a typed list presentation;
- `presentation = "adaptive_cards"` resolves to a typed adaptive-card presentation;
- unknown presentation values are rejected with useful diagnostics;
- `presentation` on a non-collection element is rejected;
- collection elements without an explicit presentation resolve conservatively to `list`;
- accepted canonical collection presentations remain explicit and unchanged in source.

## Semantic inspection

Semantic/debug output must make collection presentation inspectable.

Concept output should preserve the distinction well enough that an agent/human can tell that a collection is a list versus adaptive cards.

ASCII may remain structural if expanding it would add little value.

## EgUI behavior

### List

A list collection:

- remains a compact linear vertical sequence;
- does not acquire card framing merely because card support exists elsewhere;
- preserves existing fixture-backed selected-state treatment.

### Adaptive cards

An adaptive-card collection:

- renders items as visually discrete peer units;
- uses the available collection width to place multiple cards per row where practical;
- wraps to additional rows as needed;
- remains visibly different from the list renderer;
- preserves fixture-backed selected-state visibility;
- works when no ViewWright visual profile is authored by falling back to ordinary egui theme semantics.

The backend may choose practical card dimensions. M8 does not standardize exact pixel geometry.

## Canonical human QA

Use exact preview selectors.

### `project_browser / many_projects`

Verify:

- `navigation_items` is a vertical list;
- `project_collection` is a wrapping card collection;
- Project A remains visibly selected;
- Project A/B/C/D are not rendered as one plain vertical label list.

### `project_browser / selected_project`

Verify:

- the single project is still represented as a card;
- selected-state treatment remains visible.

### `project_browser / empty`

Verify:

- no project cards are fabricated;
- navigation remains a list.

### `dependency_workbench / healthy`

Verify:

- Scope remains a list;
- Dependencies remains a list;
- no accidental card conversion occurred.

### `dependency_workbench / advisory`

Verify list behavior remains intact with the advisory fixture.

## Tests

Add meaningful tests covering at least:

- valid `list` parsing/resolution;
- valid `adaptive_cards` parsing/resolution;
- unknown presentation rejection;
- presentation on non-collection rejection;
- omitted collection presentation defaults to list;
- Project Browser resolves one list collection and one adaptive-card collection;
- Dependency Workbench collection presentations remain list;
- semantic/debug output exposes presentation;
- concept output distinguishes list from adaptive cards;
- adaptive-card rendering behavior has backend-level coverage where practical;
- list rendering remains distinct from adaptive-card rendering where practical;
- selected collection state remains supported;
- accepted M0–M7 tests remain passing.

## Non-goals

M8 does not require:

- card-specific authored styling tokens;
- arbitrary nested card content;
- per-card actions;
- drag/reorder;
- collection-selection events;
- breakpoints;
- media queries;
- percentage sizing;
- masonry layout;
- virtualized lists/grids;
- reusable component authoring;
- a general grid/constraint system;
- ViewWitness integration.

## Worker hygiene

Any preview/process launched for QA must be closed before completion is reported unless explicitly requested otherwise.
