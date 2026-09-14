# M10 — Stop Condition

Stop M10 when:

1. region ids and roles no longer appear automatically in authored egui output;
2. command labels are not duplicated;
3. non-command authored labels still identify their content where appropriate;
4. semantic/debug output retains internal region identity and role;
5. canonical Reader, Project Browser, Dependency Workbench, density pressure, audit, layout, fixture, action, and collection-presentation regressions pass;
6. no new region-title syntax or generic chrome framework has been introduced.

Do not continue into explicit region headings, menus, title bars, dormant element kinds, or M11 work.