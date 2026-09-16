# M31 — Implementation Boundary

Keep M31 focused on one new topology projection: a two-layer overlay composition.

Allowed implementation work:

- add typed `Overlay` composition kind;
- represent overlay as axisless after resolution;
- validate the narrow overlay child/axis/gap/floating-region constraints;
- extend backend-independent layout planning with overlapping base/floating rectangles;
- extend semantic/debug, ASCII, concept, and egui projections to preserve overlay topology;
- ensure egui overlap draw/interaction order matches authored layer order;
- add focused model/layout/backend regression coverage;
- add the M31 pressure specimen to the preview selector for QA;
- update milestone bookkeeping.

Do not:

- re-admit `stack`;
- create arbitrary layer counts;
- add z-index authoring;
- add absolute x/y coordinates;
- add anchoring or edge placement;
- add backdrop/dimming semantics;
- add modal focus or focus trapping;
- add dismissal behavior;
- add fixture-driven visibility/structure;
- add new region roles or element kinds merely for the pressure specimen;
- build a general windowing/docking system;
- change application event execution semantics;
- start M32.

Parsing, validation, and layout planning remain outside the hot render path. Rendering should consume the resolved model and LayoutPlan rather than reinterpret raw TOML.
