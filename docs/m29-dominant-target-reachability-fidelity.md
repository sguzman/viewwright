# M29 — Dominant Target Reachability Fidelity

M15 made `design.dominant` a typed resolved reference to a region or element. M16 preserved a separate rule: unused declarations remain legal and ViewWright does not require every authored region/composition to be reachable from the root.

Those rules currently permit a contradiction: `design.dominant` may name a real region or element that cannot appear anywhere in the screen rooted at `screen.root`.

## Principle

A target described as dominant must belong to the rendered composition it is intended to dominate.

For M29:

- `design.dominant` remains optional;
- it may still target a region or element exactly as M15 allows;
- a dominant region must be reachable from `screen.root` through the resolved composition tree;
- a dominant element is reachable when its owning region is reachable from `screen.root`;
- valid target identity is preserved exactly;
- unrelated unused declarations remain legal.

## Boundary

M29 does not add layout priority, styling, focus, z-order, accessibility semantics, responsive behavior, mandatory global reachability, composition reuse, or new TOML syntax.

It only rejects the self-contradictory case where authored hierarchy intent names a target that is outside the authored root composition.