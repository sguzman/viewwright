# M30 — Acceptance Matrix

| Case | Expected |
| --- | --- |
| fixture content targets element in root child region | resolves |
| fixture content targets element in nested reachable region | resolves |
| fixture content targets existing element in unreachable region | validation error |
| missing fixture-content element id | existing missing-element error |
| invalid/missing screen root + existing fixture-content target | root error only; no dependent reachability error |
| fixture omits content for reachable element | resolves |
| unused region/element with no fixture content | remains legal |
| unused composition cycle | remains invalid under M18 |
| payload family incompatible with element kind | existing compatibility error |
| canonical fixture content | resolves unchanged |
| renderer/layout/projection output for valid source | unchanged |

M30 must not introduce mandatory fixture coverage or global declaration reachability.