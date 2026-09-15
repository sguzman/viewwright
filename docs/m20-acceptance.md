# M20 — Acceptance

M20 is accepted when:

- missing `screen.purpose` continues to fail at source parsing as today;
- empty `screen.purpose` fails validation;
- whitespace-only `screen.purpose` fails validation;
- nonblank purpose text resolves exactly as authored;
- `ResolvedScreen.purpose` remains `String`;
- semantic/debug and concept projections remain unchanged for valid sources;
- no purpose fallback or normalization is introduced;
- canonical accepted sources require no migration;
- M0–M19 behavior remains intact.

No human screenshot QA is required because M20 has no intended visible delta.
