# M8 — Stop Condition

M8 stops when existing authored collection presentation is typed, inspectable, and faithfully projected by egui.

Stop when:

1. `list` and `adaptive_cards` are typed resolved collection presentations;
2. unknown presentation values are rejected;
3. presentation on non-collection elements is rejected;
4. omitted collection presentation resolves to `list`;
5. semantic/debug output exposes presentation;
6. concept output preserves the presentation distinction;
7. `project_browser / many_projects` renders navigation as a list and projects as adaptive cards;
8. adaptive cards wrap across available collection width and preserve representative selection;
9. `project_browser / selected_project` remains a card presentation;
10. `project_browser / empty` fabricates no cards;
11. `dependency_workbench / healthy` and `dependency_workbench / advisory` remain list presentations;
12. M0–M7 behavior remains intact.

Do not continue into:

- rich card-content schemas;
- per-card actions;
- collection interaction events;
- drag/reorder;
- card-specific authored styling;
- reusable components;
- generalized grids/constraints;
- breakpoints or responsive-layout language;
- virtualization;
- ViewWitness integration.

M8 is complete once the already-authored distinction between a list and adaptive cards stops disappearing at the backend boundary.
