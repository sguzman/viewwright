# M15 — Stop Condition

Stop M15 once the existing `design.dominant` reference survives resolution as typed semantic identity.

M15 is complete when:

- `ResolvedBlueprint.design` uses a resolved design type;
- dominant region targets and element targets are distinguishable after resolution;
- invalid dominant references still fail clearly;
- omission remains valid;
- character/avoid metadata is preserved;
- concept output remains deterministic and source-like;
- canonical behavior remains unchanged;
- M0–M14 regressions pass.

Do not continue into renderer interpretation of dominance, general reference typing, accessibility hierarchy, or any other M16 candidate.
