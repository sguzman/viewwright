# M27 — Implementation Boundary

Implementation belongs in model validation during fixture/document resolution.

Keep M27 local to document titles.

Do not:

- normalize or trim accepted titles;
- validate paragraph blankness;
- require one or more paragraphs;
- add document IDs or title uniqueness;
- introduce rich text, Markdown, HTML, chapters, pagination, or document schemas;
- change egui rendering, layout, concept, ASCII, or preview selection;
- create a generalized visible-string sanitation framework;
- begin M28.

Valid document fixtures must resolve byte-for-byte as before.
