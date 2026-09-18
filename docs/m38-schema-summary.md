# M38 — Schema Summary

M38 adds no authoring or wire schema.

## TOML

No new fields.
No migrations.
Canonical TOML remains unchanged.

## Resolved model

No new `ResolvedBlueprint` fields.

## M33 expectation

`ViewWrightExpectation` 0.1 remains unchanged.

M38 consumes the existing typed expectation directly.

## ViewWitness

ViewWitness Witness 0.1 remains unchanged.

M38 consumes the real Witness returned by the already-pinned ViewWitness egui adapter.

## M35 comparison

`ComparisonReport`, `Mismatch`, `EvidenceGap`, and `ComparisonError` remain unchanged.

M38 requires the existing report to be an exact match; it does not add a new verification result schema.

## Dependency-only composition

The expected implementation change is dev/test composition between existing crates, not a model/schema extension.
