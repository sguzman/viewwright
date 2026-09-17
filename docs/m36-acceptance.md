# M36 — Acceptance

M36 is accepted when all of the following hold:

- `screen.id` equal to any authored region ID is rejected;
- `screen.id` equal to any authored element ID is rejected;
- collisions are rejected even when the colliding region/element is unused or unreachable;
- diagnostics identify the screen ID and the conflicting observable semantic object clearly;
- valid authored IDs are preserved exactly;
- canonical accepted sources require no migration;
- existing region/element/composition/fixture uniqueness behavior remains unchanged;
- `screen.id` equal to a composition ID remains legal when all other source rules are satisfied;
- `screen.id` equal to a fixture ID remains legal when all other source rules are satisfied;
- local collection/tree IDs remain separate and are not pulled into the observable namespace;
- M33 expectation format and exact author IDs are unchanged;
- M34 AccessKit author IDs remain exact authored IDs with no prefixes or surrogate mapping;
- M35 comparator logic remains unchanged and still treats duplicate observed author IDs as ambiguity;
- no ViewWitness repository changes are made;
- M0–M35 regressions remain green;
- README records M35 accepted and M36 implementation complete / Director audit pending.

M36 has no intended visible UI delta, so no human screenshot QA is required if renderer files remain unchanged.
