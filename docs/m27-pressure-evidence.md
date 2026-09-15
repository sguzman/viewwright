# M27 — Pressure Evidence

M5 made document fixture content authoritative and intentionally plain: a document contains a required `title` plus ordered plain-text `paragraphs`.

The current renderer consumes resolved document content directly and renders the title as the strong visible document heading. A blank title therefore survives all the way to visible output as missing authored heading content.

Canonical Reader fixtures demonstrate the intended semantics:

- loaded documents use meaningful titles such as `The Quiet Machine`;
- the empty Reader state still authors an explicit title, `No document loaded`;
- paragraph lists may legitimately be empty.

This earns title blankness validation without earning paragraph sanitation or richer document semantics.
