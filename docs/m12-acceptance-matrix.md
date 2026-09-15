# M12 — Acceptance Matrix

| Area | Required proof |
| --- | --- |
| Source vocabulary | All six accepted spellings map successfully |
| Unknown role | At least one unsupported spelling is rejected with a useful diagnostic |
| Resolved model | Region role is typed, not arbitrary `String` |
| Semantic/debug | Region id + role remain visible deterministically |
| Concept | Region role remains visible with existing surface/importance semantics |
| egui | Command-region behavior branches on typed `RegionRole::Commands` or equivalent |
| Canonical Project Browser | `navigation`, `primary_content`, `inspector` still resolve |
| Canonical Reader | `commands`, `navigation`, `primary_content`, `inspector` still resolve |
| Dependency Workbench | `commands`, `navigation`, `primary_content`, `inspector`, `status` still resolve |
| Density pressure | `controls` and `primary_content` still resolve |
| Regressions | M0–M11 test suite remains passing |
| Scope | No new role, visible chrome, navigation, selection events, or M13 work |
