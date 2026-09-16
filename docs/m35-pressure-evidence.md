# M35 — Pressure Evidence

M35 exists because M33 and M34 now provide the two halves required for a truthful first comparison loop.

## Intended side

M33 produces a typed `ViewWrightExpectation` containing:

- logical viewport width/height;
- authored screen identity;
- root-reachable authored region identities;
- root-reachable semantic element identities;
- exact region bounds copied from `LayoutPlan`;
- each semantic element's owning `region_author_id`.

M33 deliberately does not invent element geometry.

## Observed side

At audited ViewWitness revision:

`f1930ab2a70175c46d12dd1e61501c3b4ae09408`

model-only `viewwitness::Witness` exposes:

- `capture.viewport.width` / `height` in logical coordinates;
- `Node::parent` as witness-local node identity;
- optional `NodeIdentity::author_id` as application-authored identity evidence;
- optional observed `Node::bounds`;
- `Witness::validation_issues()` for structural validation.

M34 makes ViewWright's egui projection publish exact ViewWright screen/region/element IDs through AccessKit `author_id`, and ViewWitness preserves that evidence.

## Earned comparison surface

The overlap is therefore concrete and narrow:

- exact viewport size;
- expected author-ID presence and uniqueness;
- exact region bounds when the witness supplies them;
- exact element-to-region ownership through the observed parent node's author identity.

This overlap is enough to justify a typed comparator without inventing new ontology.

## Why other facts remain deferred

Current observation evidence does not yet justify directly comparing ViewWright's:

- region role;
- importance;
- overflow policy;
- element kind;
- labels;
- command action IDs;
- dominant target.

Those concepts may later gain explicit observation-side mappings, but M35 must not silently treat similarly named fields as equivalent semantics.

## Evidence absence matters

Observed bounds and parent-author evidence are optional. Their absence is not the same epistemic event as contradictory evidence.

M35 therefore needs a first-class distinction between:

- mismatch — evidence contradicts/fails an exact expected fact;
- evidence gap — the witness does not contain enough evidence to evaluate that fact.

That distinction is part of the milestone pressure, not incidental error handling.

## No visible pressure

M35 is model analysis only. It has no intended visible UI delta and requires no screenshot QA.