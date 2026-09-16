# M29 — Acceptance Matrix

| Source case | Expected result |
|---|---|
| `design.dominant` omitted | accept |
| dominant region exists and is root-reachable | accept, preserve exact typed region identity |
| dominant element exists and owning region is root-reachable | accept, preserve exact typed element identity |
| dominant region exists but is unreachable | reject with dominant/reachability context |
| dominant element exists but owning region is unreachable | reject with dominant/reachability context |
| dominant id does not exist | reject under existing M15 missing-reference behavior |
| unrelated unused region/composition/element not named dominant | accept |
| nested reachable dominant region | accept |
| element in nested reachable region | accept |
| canonical accepted specimens | accept unchanged |

M29 does not add warnings for unused declarations generally.