# M35 — Canonical Audit

M35 is intentionally constrained by the already-accepted ViewWright and audited ViewWitness models.

## ViewWright authority carried forward

### M33 expectation export

M33 already provides the intended facts M35 is allowed to consume:

- explicit logical viewport;
- authored screen ID;
- root-reachable region IDs;
- root-reachable semantic element IDs;
- exact region bounds from `LayoutPlan`;
- element `region_author_id`.

M35 must not modify the M33 expectation version/schema merely to make comparison easier.

M33 deliberately exports no element bounds; M35 therefore cannot compare element geometry.

### M34 observation identity bridge

M34 already establishes exact authored ViewWright IDs in AccessKit `author_id` for:

- screen;
- root-reachable regions;
- semantic elements.

It also preserves semantic hierarchy screen → region → element and ordinary native accessibility children.

M35 must consume that explicit identity evidence rather than add heuristic fallback matching.

## Audited ViewWitness revision

M35 is pinned to:

`f1930ab2a70175c46d12dd1e61501c3b4ae09408`

At that revision, model-only ViewWitness publicly exposes:

- `Witness`;
- `Capture`;
- `Viewport`;
- `Node`;
- `NodeIdentity`;
- `Rect`;
- `Witness::validation_issues()`.

The relevant observation semantics are:

- `Capture.viewport.width` and `.height` are f32 logical dimensions;
- `Viewport.scale_factor` exists independently;
- `Node::id` is witness-local identity;
- `Node::parent` references another witness-local node ID;
- `NodeIdentity::author_id` is optional application-authored identity evidence and does not replace `Node::id`;
- `Node::bounds` is optional observed geometry;
- `Witness::validation_issues()` rejects duplicate witness-local node IDs, missing parents, invalid geometry, and other structural defects.

The audited example corpus uses:

`viewwitness_version: "0.1"`

M35 should therefore support that exact version and reject unsupported versions explicitly.

## Dependency audit

The ViewWitness public model is available without enabling its optional egui integration.

M35 must use:

- Git dependency pinned to the audited revision;
- `default-features = false`.

It must not enable ViewWitness `egui`, `observer`, or showcase-related integration merely to compare typed models.

If Cargo resolution shows that model-only use still introduces conflicting renderer dependencies, implementation must stop and report that conflict rather than copying the ViewWitness schema locally.

## Exact comparison semantics

The canonical first overlap between intended and observed state is:

- viewport width/height;
- exact author identity presence/uniqueness;
- exact region bounds when observed;
- element ownership through parent node author identity.

Other superficially similar fields are not yet canonically equivalent and remain out of scope.

## Epistemic audit

M35 must preserve the distinction between:

- contradiction/failure of an exact expected fact (`mismatch`);
- absence of sufficient observed evidence (`evidence gap`).

Examples:

- expected region author ID absent → mismatch;
- matched region has no observed bounds → evidence gap;
- observed region bounds differ → mismatch;
- matched element parent has no author ID → evidence gap;
- matched element parent has explicit wrong author ID → mismatch.

No score/confidence/tolerance is canonically justified yet.

## Canonical non-changes

M35 requires no changes to:

- authored TOML;
- resolved blueprint vocabulary;
- LayoutPlan;
- egui rendering;
- AccessKit identity projection;
- M33 expectation YAML;
- ViewWitness repository;
- responsive-layout roadmap.

If implementation pressure appears to require one of those changes, stop and reassess rather than silently expanding M35.