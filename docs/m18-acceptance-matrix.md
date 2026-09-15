# M18 Acceptance Matrix

| Case | Expected result |
|---|---|
| valid canonical root tree | resolves unchanged |
| root-reachable cycle | rejected |
| unreachable two-node cycle | rejected |
| unreachable longer cycle | rejected |
| unreachable self-cycle | rejected |
| unused acyclic composition | allowed |
| unused region | allowed |
| M16 multiple-parent topology | still rejected |
| root used as child | still rejected |
| duplicate sibling | still rejected |
| valid M4 layout geometry | unchanged |

No human visual QA is required unless implementation unexpectedly touches layout or rendering.
