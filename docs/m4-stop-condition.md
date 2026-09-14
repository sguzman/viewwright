# M4 — Stop Condition

Stop M4 when all of the following are true:

1. nested compositions can explicitly participate in parent growth via `composition.grow`;
2. growth values are validated consistently;
3. a pure layout projection converts a resolved blueprint + concrete viewport into deterministic composition/region rectangles;
4. padding, gaps, fixed main-axis sizes, and grow weights are reflected in those rectangles;
5. egui consumes those slots rather than independently approximating major geometry;
6. region surfaces fill their assigned slots;
7. the Reader Workspace uses its full body area between top and bottom command regions;
8. the Dependency Workbench uses its full body area between command and status regions;
9. Project Browser remains correct;
10. accepted M0–M3 semantics remain intact.

Do not continue M4 merely to add richer sizing vocabulary or polish element-level layout.

The following require new pressure and a later milestone:

- responsive breakpoints
- percentages
- min/max sizing
- drag resizing
- docking
- scroll behavior
- generalized intrinsic measurement
- interaction semantics
- ViewWitness comparison/export