# M30 — Acceptance

M30 is accepted when all of the following hold:

- fixture content targeting an element in a root-reachable region resolves successfully;
- fixture content targeting an element in a nested root-reachable region resolves successfully;
- fixture content targeting an existing element whose owning region is outside the root tree is rejected;
- the rejection is independent of payload family and does not weaken family/kind validation;
- missing element references retain their existing diagnostic;
- missing/invalid roots do not produce dependent fixture-content reachability errors;
- fixtures may omit content for reachable elements;
- unrelated unused regions/compositions/elements remain legal;
- M18 global cycle validation remains global;
- M29 dominant reachability behavior remains unchanged;
- accepted canonical specimens require no migration;
- renderer, layout, concept, ASCII, and visible output remain unchanged for valid sources;
- M0–M29 regressions remain green.

No global reachability requirement or mandatory fixture coverage is introduced.