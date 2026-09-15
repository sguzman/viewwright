# M25 — Pressure Evidence

ViewWright's core principles require design intent and explicit avoidances to remain machine-readable. `design.character` and `design.avoid` are the authored vocabulary for that intent.

The concept projector emits every resolved entry directly under `Design character` and `Avoid`. The resolver currently copies both vectors without validating individual strings. As a result, `character = [""]` or `avoid = ["   "]` resolves and projects as an empty instruction line.

This is not arbitrary copy sanitation: these are explicit design-intent entries whose only semantic payload is the authored text itself.