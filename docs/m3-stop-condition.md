# M3 Stop Condition

Stop M3 once one real non-reader specimen can derive representative collection items, selected item, property-sheet values, and status text from canonical resolved fixture content.

At that point:

- the dependency workbench should switch between at least two authored fixtures without application-specific renderer code;
- the supported element families should not depend on fixture-name substring heuristics;
- existing accepted structural and visual behavior should remain intact;
- no generalized runtime data-binding language should have been introduced.

Do not continue from fixture preview data into application state management, actions, forms, tables, tree schemas, persistence, or host-app integration merely because those directions become imaginable.