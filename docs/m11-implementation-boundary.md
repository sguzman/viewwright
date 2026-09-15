# M11 — Implementation Boundary

M11 fixes only the continuity of Search text in the egui projection.

## In scope

- introduce a small explicit egui renderer state object, or equivalent explicit host-owned backend-local state;
- retain Search text across frames;
- key Search text by stable element identity;
- let the preview host clear/rebind ephemeral state when specimen or fixture changes;
- preserve all authored labels and accepted projection semantics;
- add focused regression tests.

## Preferred state ownership

Prefer explicit mutable state passed at the renderer boundary rather than process globals or hidden static storage.

A conceptual shape such as:

```text
RenderState
  search_inputs: map<element_id, string>
```

is sufficient.

Exact Rust naming and container choice are implementation details.

The state is backend-local and ephemeral. It does not belong in `ResolvedBlueprint`.

## No source/schema change

Do not add Search query/value fields to TOML.

Do not add Search query/value fixture content.

Do not serialize live query state through semantic/debug, ASCII, or concept projections.

## No search execution

M11 does not filter or mutate:

- collections;
- adaptive cards;
- trees;
- documents;
- property sheets;
- commands;
- status text.

Do not add query parsing, matching, ranking, highlighting, or search indexes.

## No application event contract

Do not widen M7 interaction semantics with SearchChanged events in M11.

The preview is only proving that an editable field can sustain editing.

## Reset boundary

The preview host should prevent ephemeral search values from bleeding between different specimen/fixture selections.

A simple reset on specimen or fixture change is sufficient and preferred over a generalized state-routing framework.

## Preserve accepted milestones

- M4 major geometry remains unchanged;
- M6 visual semantics remain unchanged;
- M7 command actions/states remain unchanged;
- M8 collection presentation remains unchanged;
- M9 density remains unchanged;
- M10 authored chrome/label behavior remains unchanged.

## Render-thread rule

Search text state lookup/update is tiny UI-thread work and is appropriate in-frame.

Do not introduce parsing, I/O, filtering work, indexing, or other heavy computation on the render thread.
