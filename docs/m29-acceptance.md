# M29 — Acceptance

M29 is accepted when all of the following hold:

- omitted `design.dominant` remains legal;
- a dominant region reachable from `screen.root` resolves successfully;
- a dominant element whose owning region is reachable resolves successfully;
- a dominant region that exists but is unreachable from `screen.root` is rejected;
- a dominant element that exists but whose owning region is unreachable is rejected;
- missing dominant references continue to fail under M15 semantics;
- exact authored target identity is preserved;
- unrelated unused regions/compositions/elements remain legal;
- canonical accepted specimens require no migration;
- renderer, layout, concept semantics, and visible output remain unchanged for valid sources;
- M0–M28 regressions remain green.

No global reachability rule is introduced.