# M5 — Reader Fixture Acceptance

M5 is accepted when the Reader Workspace no longer depends on backend-invented outline/document placeholders or fixture-name heuristics for representative preview state.

## Required source/resolved capability

Support typed fixture content for:

- tree nodes with stable ids, labels, optional parent ids, and optional selected node
- plain documents with a title and ordered paragraphs

Existing M3 fixture content for collections, properties, and text remains unchanged.

Resolved fixture content must be backend-independent.

## Required validation

Add actionable diagnostics for at least:

- duplicate tree node id
- missing tree parent reference
- self-parenting tree node
- tree parent cycle
- selected tree node that does not exist
- tree content on an incompatible element kind
- document content on an incompatible element kind
- mixed payload families in one fixture-content record
- unknown fields inside tree/document payload structures

Existing missing-element, duplicate-content, collection/property/status, visual, layout, root, and cycle validation must remain passing.

## Reader fixture behavior

### Loaded reading fixture

The structural and visual Reader specimens should be able to render fixture-authored:

- a hierarchical outline
- selected chapter
- document title
- multiple document paragraphs
- reading settings through existing property content
- reading status through existing text content

No chapter labels, document prose, settings values, or status wording may be hardcoded in the egui backend.

### Empty fixture

The empty state must be authored canonically.

The egui renderer must not use fixture-id or fixture-state substring checks such as `contains("no_document")` to decide what the document surface says.

## EgUI projection

- tree content renders recognizably with indentation derived from parent relationships;
- selected tree node receives restrained existing visual emphasis;
- document content renders title + ordered paragraphs inside the document region;
- property/status rendering continues to use M3 behavior;
- no Reader-specific element ids are special-cased.

M5 does not require production typography or scrolling.

## Semantic/debug projection

Resolved tree/document fixture content should be inspectable in the semantic/debug output at a concise level.

Do not dump arbitrary raw TOML.

## Tests

Add meaningful tests covering at least:

- valid tree fixture resolution
- valid document fixture resolution
- duplicate tree node rejection
- missing parent rejection
- self-parent rejection
- tree parent-cycle rejection
- missing selected node rejection
- tree payload on incompatible element rejection
- document payload on incompatible element rejection
- mixed payload-family rejection
- unknown tree/document fields rejected
- Reader loaded and empty fixtures remain distinct after resolution
- Project Browser remains valid
- Dependency Workbench remains valid
- M4 layout tests remain valid
- all existing model/visual/fixture tests remain passing

## Runtime QA

Human QA should inspect at least the visual Reader `reading` and `empty` fixtures and confirm:

- the reader is no longer an almost-empty placeholder canvas;
- the outline is real authored fixture content rather than `tree element`;
- the document is real authored fixture content rather than backend placeholder prose;
- selected outline state is visible;
- settings/status are populated from fixture content;
- empty state changes from canonical fixture content;
- M4 layout fidelity remains intact.

After this QA, the visual Reader palette can be judged again on its own merits.

## Worker hygiene

Any preview/process launched during QA must be closed before completion is reported unless explicitly requested otherwise.

## Acceptance boundary

M5 does not require rich text, scrolling architecture, pagination, EPUB support, tree interaction state, event/action wiring, TTS semantics, responsive rules, new visual tokens, or ViewWitness integration.