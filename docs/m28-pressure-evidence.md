# M28 — Pressure Evidence

M3 introduced fixture-backed textual content so status surfaces no longer rely on renderer-invented placeholder state. Its pressure specimen requires a representative authored status line.

Current resolution accepts `FixtureContentSource.text: Some(String)` for status elements and copies the string directly into `ResolvedFixtureContent::Text`. The egui renderer then labels that string directly.

Therefore a present empty or whitespace-only string survives as a fully authored text payload whose entire visible semantic content is blank.

This differs from M26 property values and M27 document paragraphs:

- a property row still has an authored field name when its value is empty;
- a document still has a title and ordered paragraph structure when one paragraph string is blank;
- a status `text` payload has no second authored field carrying its visible content.

Omitting the status content record already remains the legal way to provide no fixture-backed status content.