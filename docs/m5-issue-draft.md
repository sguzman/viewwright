# M5 issue authority draft

Implement the smallest fixture-content extension justified by the Reader Workspace pressure test.

## Goal

Make the Reader preview honest enough for meaningful visual QA by replacing backend-invented tree/document placeholders and Reader fixture-name heuristics with canonical typed fixture content.

## Required source capability

Add two fixture payload families:

- `nodes` for `tree` elements: stable id, label, optional parent, optional selected node
- `document` for `document` elements: title + ordered plain paragraphs

Continue using existing M3 `properties` and `text` payloads for Reader settings and reading status.

## Validation

Diagnose at least:

- duplicate tree node ids
- missing parent references
- self-parenting
- tree parent cycles
- missing selected node
- tree payload on non-tree element
- document payload on non-document element
- mixed payload families
- unknown fields

Preserve all M0–M4 validation.

## Renderer behavior

The egui backend must render resolved tree/document fixture content without hardcoding Reader ids or prose.

Remove the remaining Reader document fixture-id/state heuristic (`contains("no_document")` or equivalent).

## Pressure specimens

Use the aspirational pressure files:

- `specimens/reader-workspace-m5.toml`
- `specimens/reader-workspace-visual-m5.toml`

The accepted M4 Reader specimens remain valid until implementation supports the new schema. After implementation, migrate the canonical Reader specimens cleanly.

## Runtime / worker rules

Parsing/resolution remains outside the render loop.

Any preview window/process launched for QA must be closed before completion is reported unless explicitly requested otherwise.

## Explicit non-goals

Do not broaden into rich text, Markdown/HTML/EPUB parsing, pagination, scrolling architecture, document persistence, tree expand/collapse runtime state, event/action wiring, TTS semantics, runtime data binding, responsive rules, new visual vocabulary, accessibility ontology, or ViewWitness integration.

## Authority

- `docs/m5-reader-fixture-content.md`
- `docs/m5-acceptance.md`
- `docs/m5-schema-summary.md`
- `docs/m5-implementation-boundary.md`
- `docs/m5-stop-condition.md`
- `docs/m5-pressure-specimens.md`
- `specimens/reader-workspace-m5.toml`
- `specimens/reader-workspace-visual-m5.toml`

M5 stops when loaded and empty Reader states are rendered from typed resolved fixture content for outline, document, settings, and status, accepted M0–M4 behavior remains intact, and the visual Reader can be judged again without placeholder-content confounding.