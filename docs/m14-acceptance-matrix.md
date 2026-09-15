# M14 — Acceptance Matrix

| Pressure | Required result |
|---|---|
| canonical authored label | preserved exactly in resolved element |
| omitted label | validation error |
| empty label | validation error |
| whitespace-only label | validation error |
| internal element id | never used as visible-label fallback |
| command | authored label remains control-owned |
| search / collection / tree / property sheet / document / status | authored label remains current kind-appropriate visible text |
| text / preview | authored label remains their current visible text |
| Project Browser | no intended visual delta |
| Reader | no intended visual delta |
| Dependency Workbench | no intended visual delta |
| density pressure pair | no intended visual delta |
| semantic/debug / ASCII / concept | consume authored resolved label, no synthetic-id copy |
| M0–M13 | regressions remain passing |