# M31 — Acceptance

M31 is accepted when all of the following hold:

- `kind = "overlay"` is a supported typed composition kind;
- `stack` remains rejected;
- overlay has no resolved horizontal/vertical axis and no fake axis is projected;
- overlay requires exactly two children;
- child 0 is a base composition;
- child 1 is a floating region;
- overlay `axis` is rejected when authored;
- overlay `gap` is rejected when authored;
- the floating region requires fixed width and fixed height;
- positive floating-region `grow` under an overlay parent is rejected rather than silently ignored;
- the base composition fills the overlay inner rectangle;
- the floating region is centered in that same inner rectangle without consuming base layout space;
- authored child order defines z-order: base first, floating region second;
- pointer interaction in overlapping geometry prefers the floating layer;
- outside the floating region, base interaction remains available;
- ordinary overlay padding remains deterministic;
- linear split/row/column geometry and M21 fixed-plus-grow behavior remain unchanged;
- composition ownership, cycles, root reachability, dominant reachability, and fixture-content reachability remain intact;
- semantic/debug, ASCII, and concept projections identify overlay topology without inventing an axis;
- the M31 pressure specimen resolves and previews recognizably;
- existing canonical specimens require no migration;
- no modality, backdrop, anchoring, arbitrary positioning, conditional structure, stack semantics, or window-manager behavior is introduced;
- all M0–M30 regressions remain green.

M31 has an intended visible delta only for the new overlay pressure specimen. Existing accepted screens should render unchanged.
