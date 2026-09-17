# M36 — Acceptance Matrix

| Case | Expected result |
|---|---|
| distinct screen / region / element IDs | valid |
| `screen.id == reachable region.id` | validation error |
| `screen.id == unused region.id` | validation error |
| `screen.id == reachable element.id` | validation error |
| `screen.id == element.id` in unused region | validation error |
| `screen.id == composition.id` | remains valid if all other rules pass |
| `screen.id == fixture.id` | remains valid if all other rules pass |
| local collection item ID equals screen ID | remains legal |
| local tree node ID equals screen ID | remains legal |
| valid Unicode/case-sensitive distinct IDs | preserved exactly |
| M33 expectation export on canonical sources | unchanged |
| M34 AccessKit exact author IDs | unchanged |
| M35 exact comparator | unchanged |
| canonical Project Browser / Reader / Dependency Workbench / M31 / M32 sources | valid without migration |

## Diagnostics

Collision diagnostics must be deterministic and identify:

- the conflicting `screen.id`;
- whether the conflicting observable semantic declaration is a region or element.

Exact wording is implementation detail; tests should avoid overfitting punctuation while still proving useful author-facing diagnostics.
