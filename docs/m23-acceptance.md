# M23 — Acceptance

M23 is accepted when:

- empty/whitespace-only collection item labels fail resolution;
- empty/whitespace-only tree node labels fail resolution;
- valid labels are preserved exactly, including intentional surrounding whitespace;
- missing labels remain TOML deserialization errors;
- duplicate label text remains legal;
- item/node IDs remain the only selection/parent identity;
- canonical fixtures require no migration;
- renderer/layout behavior for valid sources is unchanged;
- M0–M22 regressions pass.

No general copy-validation policy is introduced.