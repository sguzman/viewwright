# M34 — Acceptance Matrix

| Case | Expected result |
| --- | --- |
| Project Browser screen | screen author identity emitted exactly |
| reachable region | exactly one region author-id anchor |
| reachable element | exactly one element author-id anchor |
| same frame redraw | same authored IDs remain observable |
| fixture switch | semantic authored IDs remain stable |
| unused region/element | no anchor emitted |
| fixture collection item/tree node | not promoted to canonical ViewWright author ID |
| region anchor geometry | bounds correspond to planned region rect |
| composite element | one semantic element anchor may contain multiple ordinary widget nodes |
| native child widgets | existing accessibility nodes remain present |
| M31 floating region | `palette_surface` identity remains distinct and observable |
| M32 ScrollY region | `reader` identity survives scroll container structure |
| exact IDs | no prefix/trim/hash/normalization |
| visible UI | unchanged |
| interactions | unchanged |
| ViewWitness dependency | not added |
| expectation comparison | not implemented |
| tolerance/fuzzy matching | not implemented |
| responsive rules | not implemented |