# M16 — Composition Tree Ownership Fidelity

M1 established that recursive composition remains a **tree**, not an arbitrary scene graph, and explicitly deferred/disallowed multiple-parent semantics unless real pressure earns them.

The current resolver validates child existence and cycles but does not enforce unique placement. A region or nested composition may therefore appear more than once in the authored composition graph.

That is unsafe with M4: `LayoutPlan` stores geometry by semantic id, so repeated placement of the same id overwrites earlier geometry while projections/renderers still traverse authored occurrences.

## Principle

A resolved screen composition has one structural place for each placed region/composition.

For M16:

- a region may be a child of at most one composition;
- a non-root composition may be a child of at most one composition;
- the root composition must not be authored as any composition's child;
- the same child id may not appear twice in one composition;
- existing cycle validation remains required.

This restores the tree invariant already claimed by M1.

## Boundary

M16 does not add scene-graph reuse, reusable components, aliases, cloning semantics, portals, overlays, reachability requirements for unused declarations, layout changes, renderer changes, or new TOML syntax.