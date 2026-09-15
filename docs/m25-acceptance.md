# M25 — Acceptance

M25 is accepted when:

1. empty and whitespace-only `design.character` entries fail resolution;
2. empty and whitespace-only `design.avoid` entries fail resolution;
3. valid entries are preserved exactly, including intentional surrounding whitespace;
4. omitted and explicitly empty character/avoid lists remain legal;
5. duplicate nonblank entries remain legal;
6. `design.dominant` behavior remains unchanged;
7. valid concept output remains unchanged;
8. canonical sources require no migration;
9. no general copy-validation framework, taxonomy, normalization, renderer/layout change, or M26 scope is introduced.