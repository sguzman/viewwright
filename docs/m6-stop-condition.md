# M6 — Stop Condition

M6 stops when ViewWright can diagnose the current Reader's visual-legibility pressure from the resolved visual model and expose that diagnosis without mutating authored intent.

Specifically, stop when:

1. palette relative luminance is deterministic and inspectable;
2. semantic foreground/surface contrast is deterministic and inspectable;
3. semantic structural surface separation is deterministic and inspectable;
4. the current visual Reader produces advisory findings for its compressed near-black surface range while still resolving successfully;
5. a less-compressed dark Reader pressure palette produces materially improved audit results;
6. the preview host can expose audit warnings outside the authored specimen;
7. human QA can compare canonical and pressure palettes;
8. M0–M5 behavior remains intact.

Do not continue into:

- complete accessibility modeling
- automatic palette correction
- theme generation/inheritance
- screenshot-based visual analysis
- color-vision simulation
- new interaction semantics
- ViewWitness integration

A human may still dislike a palette that passes the M6 audit. That is acceptable. The audit is evidence, not aesthetic authority.