# M34 — Acceptance

M34 is accepted when all of the following hold:

- the egui projection emits an AccessKit author-identity anchor for the rendered screen;
- every root-reachable rendered region emits exactly one anchor with exact region ID;
- every root-reachable rendered semantic element emits exactly one anchor with exact element ID;
- unused regions/elements do not emit anchors;
- local fixture item/tree-node IDs are not promoted into canonical ViewWright author identity;
- region anchor bounds match the already-planned region rectangle;
- native child widget accessibility nodes remain available beneath semantic anchors;
- author IDs survive ordinary redraws and fixture changes for the same semantic object;
- M31 overlay base/floating regions retain correct identity/hierarchy;
- M32 scrollable regions and elements retain identity through scrolling;
- no authored ID is derived from label text or backend-generated egui IDs;
- no visible layout, styling, interaction, fixture, overflow, or command behavior changes;
- no ViewWitness dependency, capture parsing, comparator, tolerance, or heuristic matching is introduced;
- direct AccessKit regression coverage proves the identity contract;
- M0–M33 regressions remain green.

No screenshot QA is required if renderer visuals/layout are byte/behaviorally unchanged and AccessKit-focused tests prove the new bridge.