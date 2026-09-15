# M13 — Acceptance Matrix

| Concern | Required proof |
| --- | --- |
| Split | resolves to typed Split and typed axis |
| Row | resolves to typed Row + Horizontal |
| Column | resolves to typed Column + Vertical |
| Unknown kind | useful validation failure |
| Stack / overlay | rejected as unimplemented topology |
| Contradictory row axis | validation failure |
| Contradictory column axis | validation failure |
| Split omitted axis | preserves current compatibility/default behavior |
| Semantic/debug | deterministic composition kind + axis inspection |
| Concept | deterministic kind + axis inspection |
| Layout | no raw string topology comparisons |
| Reader | vertical root and horizontal nested split unchanged |
| Project Browser | horizontal split unchanged |
| Density pair | same M4 geometry remains identical |
| Regressions | M0–M12 remain passing |
| Scope | no new topology system or M14 work |