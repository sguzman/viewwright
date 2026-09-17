# M36 — Canonical Audit

M36 is constrained by accepted identity semantics from M17, M33, M34, and M35.

## M17

M17 established:

- all structural IDs must be nonblank;
- valid authored IDs are preserved exactly;
- region, element, composition, and fixture IDs already participate in existing structural duplicate validation;
- globally unique screen IDs against child namespaces were explicitly deferred.

M36 reopens only the part now proven necessary by the observation bridge.

## M33

M33 expectation export carries exact authored identity separately by semantic kind:

- `screen.author_id`;
- `regions[].author_id`;
- `elements[].author_id`.

The expectation schema already distinguishes kinds, so no M33 format change is needed.

## M34

M34 intentionally projects exact ViewWright IDs into AccessKit `author_id` without prefixes or normalization.

Screen, region, and element anchors therefore share one observed author-ID channel.

That exactness must remain intact.

## M35

M35 correctly indexes ViewWitness nodes by exact `NodeIdentity.author_id` and treats more than one observed match as ambiguity. It does not and should not guess object kind from labels, roles, geometry, or hierarchy.

Therefore a screen/region or screen/element string collision is an upstream authoring ambiguity, not a comparator bug.

## Current resolver audit

`screen.id` is validated separately for nonblank content. Region/element/composition/fixture IDs use existing structural duplicate machinery.

M36 must not mechanically add `screen.id` to that complete structural set because doing so would also alter screen/composition and screen/fixture equality, which is not justified by current bridge pressure.

The narrow invariant is:

```text
screen.id ∉ region IDs
screen.id ∉ element IDs
```

## Reachability

The check is declaration-global rather than root-reachability-gated. Existing ViewWright validation already treats several authored semantic invariants globally, and latent observation identity should not become invalid only when topology later exposes it.

## Canonical compatibility

Accepted canonical ViewWright sources use distinct screen, region, and element IDs, so M36 should require no source migration and no visible UI changes.

## Canonical non-changes

M36 requires no changes to:

- TOML shape;
- resolved model shape;
- LayoutPlan;
- expectation version/YAML;
- egui layout/rendering;
- AccessKit hierarchy or author-ID values;
- comparator result types/algorithm;
- ViewWitness;
- responsive-layout roadmap.
