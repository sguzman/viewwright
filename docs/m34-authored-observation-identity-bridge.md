# M34 — Authored Observation Identity Bridge

M33 created a ViewWright-owned `intended` expectation artifact with exact authored screen, region, and element identity. A future expectation-vs-observation comparator must not recover those identities through labels, geometry, widget order, or other heuristics.

ViewWitness already preserves AccessKit `Node.author_id()` as explicit author identity evidence. M34 therefore makes the ViewWright egui projection publish its existing semantic IDs into AccessKit without changing visible UI behavior.

## Goal

For the egui projection, expose stable authored identity anchors for the root-reachable semantic screen, regions, and elements.

Conceptually:

```text
ViewWright authored identity
        ↓
egui / AccessKit author_id
        ↓
ViewWitness NodeIdentity.author_id
```

This is observation identity plumbing only. It is not expectation comparison.

## Identity values

Use exact existing ViewWright IDs as AccessKit `author_id` values.

Do not prefix, trim, normalize, hash, or derive them from labels.

Expected examples include:

- screen `project_browser`;
- region `navigation`;
- region `projects`;
- element `project_search`;
- floating region `palette_surface`.

## Semantic anchors

A semantic ViewWright object may render as several native egui widgets. M34 may introduce/access existing non-visual AccessKit container anchors so each ViewWright screen, region, and element has exactly one author-identity-bearing node.

The identity anchor must not replace or suppress the ordinary child widget accessibility nodes.

Preferred semantic hierarchy:

```text
screen author anchor
  region author anchor
    element author anchor
      ordinary egui widget/accessibility nodes
```

Do not require every leaf widget created from fixture data to receive a ViewWright author ID.

## Region geometry

A region identity anchor should use the region's already-planned egui rectangle corresponding to LayoutPlan, so observation can later associate the region author ID with honest observed bounds.

Do not recalculate region geometry.

## Element geometry

M34 does not establish a normative element-geometry contract. An element anchor may naturally carry its observed egui subtree bounds, but M33 still exports no intended element bounds.

Do not add element geometry to LayoutPlan or M33.

## Reachability

Only objects actually projected from the root-reachable screen should produce identity anchors.

Unused declarations remain legal and must not appear merely because they exist in `ResolvedBlueprint`.

## Epistemic boundary

AccessKit `author_id` is authored identity evidence attached to an observed node. It does not make the observation itself intended.

ViewWright still specifies. ViewWitness still observes.

## Non-goals

M34 does not add:

- expectation-vs-observation comparison;
- tolerance rules;
- ViewWitness parsing;
- a ViewWitness crate dependency;
- ViewWitness repository changes;
- heuristic identity matching;
- fixture item/node IDs as author identity anchors;
- accessibility role redesign;
- responsive layout;
- M35 work.
