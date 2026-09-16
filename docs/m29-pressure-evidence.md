# M29 — Pressure Evidence

M15 defines `design.dominant` as explicit hierarchy intent: accepted screens name the region intended to dominate the composition, and resolution types the reference as a region or element.

M16 separately permits unused declarations and explicitly avoids a global reachability requirement.

Current resolver behavior combines those contracts incompletely. It accepts a dominant target whenever its id exists in the region or element namespace, even when:

- the region is not reachable from `screen.root`; or
- the element exists but its owning region is not reachable from `screen.root`.

That source is internally contradictory: the authored target is declared dominant over a screen composition in which it cannot occur.

Canonical pressure is clean: Project Browser `projects`, Reader `reader`, Dependency Workbench `packages`, and density-pressure `content` are all reachable from their respective roots.