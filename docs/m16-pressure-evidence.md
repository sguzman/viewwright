# M16 — Pressure Evidence

M1 already states that recursive composition should remain a tree and that multiple-parent semantics should remain disallowed/deferred unless a real screen requires them.

Current validation checks:

- child existence;
- invalid element-as-composition-child usage;
- explicit root validity;
- composition cycles.

It does not check unique placement ownership.

M4 stores composition and region rectangles in `HashMap<String, Rect>`. Repeated semantic ids therefore have only one addressable rectangle in `LayoutPlan`; later placement overwrites earlier placement. Egui still traverses the authored child occurrences, so repeated placement can collapse into duplicated/ambiguous rendering.

Canonical accepted screens use ordinary trees and do not require multiple-parent semantics.

M16 therefore closes an existing correctness gap without expanding layout capability.