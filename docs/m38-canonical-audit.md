# M38 — Canonical Audit

M38 should require no canonical-source migration.

## Canonical proof sources

The milestone consumes existing accepted sources unchanged:

- `examples/project-browser.toml`;
- `specimens/overlay-command-palette-pressure.toml`;
- `specimens/reader-overflow-pressure.toml`.

Fixtures remain:

- `many_projects`;
- `palette_open`;
- `long_document`.

## Why no source migration is expected

M38 adds no authoring syntax and changes no resolved semantics.

It composes existing outputs:

- M33 expectation;
- M34 renderer identity/bounds;
- M37 ViewWitness conversion;
- M35 exact comparison.

Any need to edit canonical TOML merely to make the integration test pass is a warning that the existing accepted layers disagree. Such a change is not automatically authorized by M38.

## README

README may change only for milestone bookkeeping and the concise M38 summary.

The root heading must remain exactly:

`# 🟩 ViewWright`
