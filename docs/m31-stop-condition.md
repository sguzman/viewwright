# M31 — Stop Condition

Stop M31 when the overlay pressure specimen can be authored, resolved, laid out, projected, and previewed with the following narrow behavior:

- root overlay is typed and axisless;
- base workspace fills the overlay inner rectangle;
- fixed-size palette region is centered above the base without changing base geometry;
- palette paints after the base;
- overlapping pointer interaction belongs to the floating layer;
- base interaction remains available outside the floating rectangle;
- overlay topology is visible in semantic/debug, ASCII, and concept projections;
- existing split/row/column screens remain unchanged.

Do not continue M31 into:

- modal semantics;
- backdrop treatment;
- dismissal/focus management;
- anchored popovers;
- arbitrary coordinates;
- multiple overlay layers;
- stack composition;
- conditional fixture structure;
- window management.

Those require separate future pressure and authority.
