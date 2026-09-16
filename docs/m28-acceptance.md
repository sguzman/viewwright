# M28 — Acceptance

M28 is accepted when:

- `text = ""` on a status fixture payload fails resolution;
- whitespace-only status text fails using Rust `trim()` semantics;
- valid status text is preserved byte-for-byte;
- omission of a status content record remains legal;
- existing fixture-family and element-kind compatibility remains unchanged;
- property values, document paragraphs, and command reasons remain outside M28;
- accepted canonical specimens require no migration;
- valid renderer/layout/projection behavior is unchanged;
- M0–M27 regressions pass.