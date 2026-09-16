# M34 — Implementation Boundary

Keep M34 narrowly inside the egui accessibility projection.

Allowed work:

- use egui's native AccessKit node APIs;
- attach exact ViewWright screen/region/element IDs through AccessKit `author_id`;
- create or reuse non-visual semantic container anchors where one ViewWright object renders as multiple widgets;
- preserve ordinary child widget accessibility semantics;
- ensure region anchor bounds use the already-planned region rectangle;
- add AccessKit-focused egui regression tests;
- test Project Browser, M31 overlay, and M32 overflow pressure;
- update README milestone bookkeeping.

Do not:

- change authored TOML;
- change `ResolvedBlueprint` merely for identity projection;
- change LayoutPlan geometry;
- add intended element bounds;
- add ViewWitness as a dependency;
- modify the ViewWitness repository;
- parse or serialize ViewWitness captures;
- compare observations to M33 expectations;
- add tolerance/fuzzy matching;
- prefix or normalize authored IDs;
- annotate fixture-local item/node IDs as canonical ViewWright identity;
- redesign accessibility roles/names beyond the minimum container structure required for identity;
- change visible rendering or interaction;
- implement responsive layout;
- start M35.

If egui 0.31 cannot expose `author_id` without visible/interaction regression or large accessibility-role redesign, stop and report the constraint rather than inventing another identity channel.