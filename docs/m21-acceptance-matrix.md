# M21 — Acceptance Matrix

| Case | Expected |
|---|---|
| horizontal region width only | unchanged fixed width |
| horizontal region grow only | unchanged proportional growth |
| horizontal region width + grow | fixed width base + proportional remaining-width share |
| vertical region height only | unchanged fixed height |
| vertical region grow only | unchanged proportional growth |
| vertical region height + grow | fixed height base + proportional remaining-height share |
| cross-axis fixed size + grow | growth still applies on parent main axis |
| nested composition grow | unchanged |
| canonical Reader geometry | unchanged |
| canonical Dependency Workbench geometry | unchanged |
| canonical Project Browser geometry | unchanged |
| density pair major geometry | unchanged |
| fixed sizes oversubscribe viewport | existing behavior unchanged; M21 does not add overflow policy |