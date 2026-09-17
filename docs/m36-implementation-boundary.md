# M36 — Implementation Boundary

M36 is a source-validation fidelity milestone.

## In scope

- validation in `viewwright-model` that rejects `screen.id` collisions with region IDs;
- validation in `viewwright-model` that rejects `screen.id` collisions with element IDs;
- deterministic diagnostics;
- regression tests for reachable and unused collisions;
- regression tests proving screen/composition and screen/fixture equality remain legal;
- README milestone bookkeeping.

## Out of scope

- renderer changes;
- AccessKit changes;
- M33 expectation changes;
- M35 comparator changes;
- ViewWitness dependency/repository changes;
- identifier rewriting/prefixing;
- new TOML syntax;
- new resolved fields;
- composition/fixture namespace changes;
- local fixture-ID changes;
- responsive layout;
- M37.

## Runtime boundary

This work occurs during parse/resolve validation only. It adds no recurring render-thread work and no runtime bridge machinery.

## Stop condition

If implementation appears to require rewriting author IDs, changing M34 anchor semantics, or changing M35 matching, stop and report the contradiction. The source validator should be sufficient.
