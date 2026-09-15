# M16 — Acceptance Matrix

| Case | Expected result |
| --- | --- |
| Canonical Project Browser tree | resolves unchanged |
| Canonical Reader nested tree | resolves unchanged |
| Same region listed twice by one composition | validation error |
| Same region listed by two compositions | validation error |
| Same nested composition listed twice by one parent | validation error |
| Same nested composition referenced by two parents | validation error |
| `screen.root` referenced as any composition child | validation error |
| Existing composition cycle | validation error remains |
| Unused declared region/composition | unchanged by M16 |
| Valid M4 layout tree | geometry unchanged |

No visible delta is intended for accepted canonical screens.