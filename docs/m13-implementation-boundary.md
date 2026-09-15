# M13 — Implementation Boundary

M13 is a resolved-ontology and validation milestone, not a layout expansion.

## Allowed

- introduce typed composition kind and axis enums;
- validate supported kinds and axes;
- reject unimplemented `stack` and `overlay` source values;
- reject contradictory row/column axis declarations;
- update semantic/debug and concept projections to consume typed values;
- update M4 layout to consume typed topology;
- migrate tests that manually construct `ResolvedComposition` values;
- add focused regressions.

## Forbidden

- implementing actual stack layout;
- implementing overlay/z-order layout;
- adding alignment, justification, wrapping, constraints, breakpoints, percentages, grids, tabs, docking, or absolute positioning;
- changing M4 padding/gap/grow/fixed-size behavior;
- changing visual semantics, fixtures, actions, density, Search state, or authored chrome;
- adding new TOML fields;
- M14 work.

M13 must make the currently implemented topology truthful, not broaden the topology surface.