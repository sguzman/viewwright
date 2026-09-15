# M19 — Stop Condition

Stop M19 when:

- every authored color-token literal is validated whether referenced or not;
- malformed unused tokens fail even without a visual profile;
- valid unused tokens remain legal;
- existing referenced-token and missing-reference behavior remains intact;
- deterministic diagnostics are covered;
- canonical accepted palettes require no migration;
- resolved visual output and renderer/layout behavior are unchanged for valid blueprints;
- all established checks pass.

Do not continue into token usage analysis, naming rules, additional color syntaxes, shared themes, or M20.