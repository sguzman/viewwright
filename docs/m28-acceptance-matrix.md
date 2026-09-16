# M28 — Acceptance Matrix

| Case | Expected |
|---|---|
| status `text = ""` | reject |
| status `text = "   "` | reject |
| status `text = "\t"` | reject |
| status `text = "  Ready  "` | accept and preserve exactly |
| fixture omits status content record | accept |
| property value is empty | unchanged / legal |
| document paragraph is blank | unchanged / legal |
| command reason is blank or absent | unchanged by M28 |
| text payload attached to non-status element | existing compatibility error |
| record mixes payload families | existing exclusivity error |
| accepted canonical fixtures | resolve unchanged |