# M39 — Canonical Audit

M39 should not modify canonical ViewWright authoring sources.

Expected unchanged:

- `examples/*.toml`;
- `specimens/*.toml`;
- source schema;
- resolved model;
- LayoutPlan semantics;
- egui rendering;
- AccessKit identity;
- M33 expectations;
- M35 comparator;
- ViewWitness pin.

## Historical authority

Do not rewrite M0–M38 authority documents to make old future-tense boundaries sound current.

Those documents record the sequence of accepted decisions.

Only current-facing project documentation should be reconciled for release.

## Release candidate

The release candidate should consist primarily of:

- README/current-doc corrections;
- historical-label clarification;
- changelog;
- v0.1.0 release note;
- LICENSE;
- any strictly non-behavioral lint/rustdoc correction required by the release checks.

If canonical TOML or production behavior needs to change, stop and escalate.
