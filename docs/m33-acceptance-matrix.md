# M33 — Acceptance Matrix

| Case | Expected result |
| --- | --- |
| ordinary linear canonical screen | deterministic expectation document |
| identical blueprint + viewport twice | byte-identical YAML |
| different viewport size | region bounds change according to LayoutPlan |
| unused region/element declarations | remain valid source, omitted from expectation artifact |
| dominant region target | preserved as typed intended target |
| dominant element target | preserved as typed intended target |
| reachable region | exported with author ID, role, importance, bounds, overflow |
| reachable element | exported with author ID, owner region, kind, importance, exact label |
| command element | optional action ID preserved |
| non-command element | no invented action |
| element geometry | not exported/invented |
| overlay pressure | base/floating region bounds match LayoutPlan; no fake axis semantics |
| scroll_y region | exports scroll_y intent, no scroll offset |
| default clip region | exports typed clip consistently |
| ViewWitness witness envelope | not emitted |
| observed/derived evidence claims | not emitted |
| fixture payload values / search query / scroll offset | not emitted |
| existing accepted sources | no migration |
| expectation-vs-observation comparison | not implemented |
