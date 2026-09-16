# M33 — Implementation Boundary

Keep M33 as a one-way normative exporter.

Allowed implementation work:

- add a dedicated expectation-projection crate/module;
- define typed expectation structs owned by ViewWright;
- derive root-reachable region/element expectations from `ResolvedBlueprint`;
- derive region geometry from existing `LayoutPlan` at an explicit logical viewport;
- preserve typed dominant target identity;
- serialize deterministic YAML;
- add golden/regression coverage over canonical Project Browser plus M31 overlay and M32 overflow pressure;
- add a tiny operator-facing example/CLI only if needed to materialize the YAML without broadening architecture;
- update README milestone bookkeeping.

Do not:

- change authored TOML syntax;
- change model/layout/renderer semantics merely for export;
- fabricate element geometry;
- export unused declarations as visible expectations;
- serialize as ViewWitness `Witness`;
- add a dependency on the ViewWitness crate merely to reuse observed-witness structs;
- parse ViewWitness captures;
- compare expected vs observed;
- add tolerances or pass/fail verification;
- instrument AccessKit or egui IDs;
- export fixture data or runtime mutable state;
- modify the ViewWitness repository;
- implement responsive variants;
- start M34.

The bridge boundary is deliberate: M33 creates a stable intended artifact. A later milestone can define the comparator between that artifact and observed ViewWitness evidence.
