# M19 — Acceptance

M19 is accepted when color-token literal validity is an authored-source invariant rather than an incidental consequence of visual-role lookup.

Required acceptance:

- malformed referenced color token still fails;
- malformed unreferenced color token fails when a visual profile exists;
- malformed color token fails even when no `[visual]` profile exists;
- valid unreferenced color token remains legal;
- valid canonical palettes resolve unchanged;
- existing six-digit hex syntax/case behavior is preserved;
- missing visual token references still fail as before;
- diagnostics introduced by global token validation are deterministic;
- no token-use/reachability requirement is introduced;
- no token-name grammar or new color syntax is introduced;
- resolved visual model, concept projection, layout, and egui output remain unchanged for valid blueprints;
- M0–M18 regressions remain green.

No human screenshot QA is required unless implementation unexpectedly changes rendering or canonical visual output.
