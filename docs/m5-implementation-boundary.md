# M5 — Implementation Boundary

M5 is a preview-honesty milestone for the Reader specimen. It extends fixture content narrowly; it does not broaden ViewWright into a document or interaction framework.

## In scope

- typed source/resolved tree fixture content
- typed source/resolved plain-document fixture content
- tree parent/selection validation
- document compatibility validation
- egui rendering of resolved tree/document representative content
- migration of Reader fixtures to canonical content
- deletion of Reader document fixture-name heuristics
- concise semantic/debug representation
- tests preserving M0–M4 behavior

## Model/backend boundary

The model owns:

- authored tree/document representative content
- semantic compatibility
- parent/selection validity

The egui backend owns:

- indentation/presentation of already-resolved tree nodes
- simple title/paragraph rendering inside a document region

No egui types belong in source/resolved fixture content.

## No generalized runtime state

A selected tree node is static representative fixture state, exactly like M3's selected collection item.

It does not imply:

- click handling
- selection mutation
- expand/collapse mutation
- application state
- action routing

## No document engine

Plain paragraphs exist only to make the semantic document surface previewable.

Do not add:

- spans/styles
- document blocks
- layout measurement
- pagination
- scrolling architecture
- EPUB/HTML/Markdown parsing
- images
- annotations

## Existing M3 families

Use existing property/text fixture content for Reader settings and status.

Do not invent Reader-specific payload structures for information already representable.

## Runtime discipline

Fixture parsing/resolution remains outside the frame hot path.

Rendering a small already-resolved tree/document specimen is lightweight.

No file/network/database/document parsing work is permitted in the renderer.

## Worker hygiene

Any native preview or temporary QA process launched during implementation must be closed before reporting completion unless explicitly requested otherwise.

## Stop rather than widen

If the pressure specimen seems to require rich text, scrolling, runtime interactions, or generalized document/tree state, stop and report the pressure instead of silently implementing those systems.