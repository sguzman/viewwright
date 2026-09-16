# M29 — Implementation Boundary

Keep M29 as a model-resolution validation change.

Allowed implementation work:

- derive the set of region ids reachable from the validated root composition;
- validate a typed dominant region against that set;
- validate a typed dominant element through its existing owning region;
- add focused model regressions;
- update milestone bookkeeping.

Do not:

- require every declaration to be reachable;
- delete or warn on unrelated unused declarations;
- change M16 ownership/cycle semantics;
- alter layout allocation;
- make renderers style or size the dominant target;
- add focus/navigation/accessibility behavior;
- change `DominantTarget` public semantics;
- introduce a generic graph framework merely for this check;
- start M30.

The reachability computation should happen during resolution, outside the render loop.