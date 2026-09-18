# M40 — Implementation Boundary

M40 may change:

- `specimens/` by adding the Lantern Leaf baseline;
- v0.2 documentation for baseline findings;
- tests/projection fixtures needed to prove the baseline passes existing APIs;
- README bookkeeping if needed.

M40 should not change production Rust semantics.

If a tiny test harness needs to include the new specimen, that is in scope.

If the baseline exposes a production crash in accepted v0.1 behavior, stop and report rather than broadening M40.

No schema or renderer extensions are authorized.
