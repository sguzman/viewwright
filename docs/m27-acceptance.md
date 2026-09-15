# M27 — Acceptance

M27 is accepted when all of the following hold:

- `DocumentSource.title = ""` is rejected.
- whitespace-only document titles are rejected using Rust trim semantics.
- valid titles are preserved exactly, including intentional surrounding whitespace, punctuation, Unicode, and case.
- missing `title` remains a TOML parse/deserialization error because the field stays required.
- paragraph strings remain unconstrained, including empty and whitespace-only paragraphs.
- empty paragraph lists remain legal.
- valid canonical Reader fixtures resolve unchanged.
- renderer, layout, ASCII, concept, and other projections remain unchanged for valid sources.
- M0–M26 regressions remain green.
