# M18 Pressure Evidence

The current resolver builds a `by_id` map for all resolved compositions, but invokes cycle traversal only from the resolved `screen.root`.

Therefore cycle validity is accidentally coupled to reachability.

A source can declare a valid root tree plus an unused cycle such as:

```text
root -> region_a, region_b

orphan_a -> orphan_b, region_c
orphan_b -> orphan_a, region_d
```

M16 ownership does not necessarily catch this: `orphan_a` and `orphan_b` each have one parent, so the topology can satisfy unique-parent ownership while remaining cyclic.

This contradicts M1's cycle-rejection contract. M16 deliberately preserved unused declarations, but did not grant unused declarations permission to violate structural invariants.

The correct pressure response is to validate cycles across all declared compositions while preserving unused acyclic declarations.
