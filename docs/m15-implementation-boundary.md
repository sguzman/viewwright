# M15 — Implementation Boundary

M15 is deliberately narrow.

## In scope

- introduce a resolved design type rather than storing `DesignSource` inside `ResolvedBlueprint`;
- introduce typed dominant-target identity distinguishing region vs element;
- preserve existing validation semantics for region/element targets;
- preserve omission as `None`;
- preserve freeform `character` and `avoid` values unchanged;
- update concept and any other compile-identified downstream consumers to use the resolved design contract;
- add focused tests.

## Out of scope

- dominance-driven styling;
- dominance-driven layout or grow behavior;
- focus or navigation behavior;
- accessibility hierarchy;
- semantic ranking systems;
- multiple dominant targets;
- weighted dominance;
- composition dominance;
- fixture dominance;
- responsive dominance;
- new visual tokens;
- changing `importance` semantics;
- general ID-newtype migration;
- M16 work.

## Preservation

M15 must preserve M0–M14 behavior, including M4 geometry, M6 visuals, M7 actions, M8 collection presentation, M9 density, M10 chrome purity, M11 Search state, M12 RegionRole, M13 composition topology, and M14 authored labels.
