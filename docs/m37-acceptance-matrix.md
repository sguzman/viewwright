# M37 — Acceptance Matrix

| Case | Expected result |
|---|---|
| ViewWright egui 0.31 remains | reject: bridge type incompatibility remains |
| ViewWright aligned to egui/eframe 0.36.2 | required |
| real ViewWright `FullOutput` passed directly to pinned ViewWitness adapter | compiles and converts |
| manual ViewWright AccessKit-to-Witness translator | reject |
| ViewWitness `egui` enabled only for test/dev compatibility proof | allowed |
| ViewWitness `observer` or `showcase` enabled | reject |
| produced Witness validation | no issues |
| screen author ID after conversion | exact authored value |
| region author IDs after conversion | exact authored values |
| semantic element author IDs after conversion | exact authored values |
| fixture-local IDs promoted to canonical author IDs | reject |
| M34 direct AccessKit regressions | unchanged / green |
| M35 comparator API/logic | unchanged |
| ViewWitness repository | unchanged |
| canonical UI semantics | unchanged |
| representative visible preview | no unintended migration regression |
| end-to-end M33/M35 comparison | deferred to next pressure |

## Dependency audit

The implementation must show that the relevant test graph resolves the same egui package version across ViewWright and the pinned ViewWitness egui adapter. The normal M35 model-only dependency must remain model-only.
