# M17 Acceptance Matrix — Structural Identifier Fidelity

| Case | Expected result |
|---|---|
| nonblank screen ID | resolves unchanged |
| empty screen ID | rejected |
| whitespace-only screen ID | rejected |
| nonblank region/element/composition/fixture IDs | resolve unchanged |
| empty global structural ID | rejected |
| whitespace-only global structural ID | rejected |
| nonblank collection item ID | resolves unchanged |
| empty/whitespace collection item ID | rejected |
| nonblank tree node ID | resolves unchanged |
| empty/whitespace tree node ID | rejected |
| missing root | rejected |
| root names missing composition | rejected |
| root names valid nonblank composition | resolves exactly |
| valid IDs containing punctuation/spacing | preserved exactly if nonblank |
| canonical accepted sources | resolve unchanged |
| duplicate/global/local relationship checks | remain enforced |
| renderer/layout output | unchanged for valid sources |