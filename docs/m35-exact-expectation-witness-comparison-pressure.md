# M35 — Exact Expectation/Witness Comparison Pressure

M33 created a typed `ViewWrightExpectation` describing intended screen, region, element, viewport, and region geometry. M34 made the egui projection publish exact ViewWright semantic IDs through AccessKit `author_id`, and ViewWitness preserves that identity evidence as `NodeIdentity.author_id`.

M35 earns the first exact intended-vs-observed comparator.

## Goal

Compare one `ViewWrightExpectation` against one ViewWitness `Witness` using only facts that both sides currently represent truthfully.

This first comparator is deliberately strict and small. It compares:

- logical viewport width and height;
- expected screen author identity presence;
- expected region author identity presence;
- expected semantic element author identity presence;
- exact region bounds where observed bounds exist;
- expected semantic element ownership by region.

It does not infer or compare facts that the current ViewWitness model does not represent equivalently.

## Identity matching

Matching is only by exact `NodeIdentity.author_id`.

No label, role, geometry, ordering, or backend node-ID fallback is allowed.

An expected author ID with no observed node is a mismatch.

An expected author ID with more than one observed node is an identity ambiguity mismatch. The comparator must not pick one arbitrarily.

Observed nodes with no author ID are ignored for identity matching.

Observed author IDs not present in the expectation are also ignored in M35. A witness may include host/tooling or future authored identities outside this expectation slice; M35 is expectation-driven rather than enforcing global observed-author-ID closure.

## Region geometry

M33 region bounds are intended logical geometry copied from `LayoutPlan`.

ViewWitness node bounds are observed logical geometry.

For a uniquely matched expected region:

- if observed bounds exist, compare `x`, `y`, `width`, and `height` exactly;
- if observed bounds differ, report a geometry mismatch;
- if observed bounds are absent, report an evidence gap rather than inventing a rectangle or labeling absence as a contradictory geometry observation.

M35 adds no tolerance.

## Element ownership

M33 records each expected element's `region_author_id`.

For a uniquely matched expected element, inspect its observed parent node:

- if the parent resolves to the expected region author ID, ownership matches;
- if a different explicit author ID is observed on the parent, report an ownership mismatch;
- if parent/parent-author identity evidence is absent, report an evidence gap.

Do not walk labels or geometry to guess ownership.

## Viewport

Compare expectation logical viewport width/height exactly to `Witness.capture.viewport.width/height`.

Ignore ViewWitness `scale_factor` in M35 because M33 intentionally has no scale-factor field and both compared geometries are logical coordinates.

## Epistemic result boundary

Comparison output must distinguish:

- **mismatch** — observed evidence contradicts or fails an exact expected fact;
- **evidence gap** — the witness lacks enough evidence to evaluate a comparable fact.

Do not collapse evidence absence into geometry mismatch.

A convenience `is_exact_match()` may return true only when there are no mismatches and no evidence gaps.

Do not add scores, percentages, grades, confidence numbers, or fuzzy status.

## Facts explicitly NOT compared in M35

M35 does not compare:

- region role;
- region importance;
- region overflow policy;
- element kind;
- element importance;
- element label;
- command action ID;
- dominant target;
- fixture content/state;
- runtime search text;
- scroll offset;
- focus/hover/selection;
- element geometry;
- paint output;
- screenshots.

Those facts either lack equivalent current observation evidence or need separate pressure.

## ViewWitness dependency boundary

Use the ViewWitness model crate only, with default features disabled, pinned to the audited revision:

`f1930ab2a70175c46d12dd1e61501c3b4ae09408`

Do not enable ViewWitness egui/observer/showcase features. ViewWright and ViewWitness currently use different egui versions; M35 compares typed/wire models, not live renderer types.

Do not modify the ViewWitness repository.

## Non-goals

M35 does not add:

- tolerance;
- fuzzy matching;
- heuristic fallback identity;
- element-geometry expectations;
- semantic role mapping;
- screenshot/image comparison;
- live capture transport integration;
- automatic preview capture;
- CI gating;
- responsive layout;
- M36 work.
