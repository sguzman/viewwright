# M14 — Stop Condition

Stop M14 when all of the following are true:

1. missing current-element labels are rejected;
2. blank/whitespace-only labels are rejected;
3. successful resolution never derives an element label from its id;
4. `ResolvedElement.label` remains authored non-empty text;
5. canonical accepted specimens resolve unchanged;
6. semantic/debug, ASCII, concept, and egui projections preserve their accepted label behavior;
7. M0–M13 regressions pass;
8. no localization, accessibility-label, placeholder, icon-only, visibility, or M15 scope is introduced.

Do not use M14 as an excuse to redesign text semantics.