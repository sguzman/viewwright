# M17 Implementation Boundary — Structural Identifier Fidelity

## Required implementation

M17 should make the smallest model-layer correction necessary to enforce nonblank structural identity and remove the empty-root sentinel ambiguity.

Expected work:

- add a small reusable nonblank-ID validation helper or equally narrow local logic;
- apply it to screen, region, element, composition, fixture, collection-item, and tree-node IDs;
- preserve existing duplicate/reference validation;
- represent unresolved/invalid `screen.root` explicitly during validation instead of with an empty string sentinel compatible with authored identity;
- retain `ResolvedBlueprint.root: String` if desired after successful validation;
- add focused tests.

## Allowed incidental edits

Synthetic tests that currently use blank IDs only because the resolver permitted them may be repaired if blank identity is unrelated to the test's actual subject.

Canonical sources must not be migrated unless a contradiction is discovered and reported.

## Explicitly forbidden scope

Do not:

- add `RegionId`, `ElementId`, `CompositionId`, etc. across the project;
- impose an ASCII/slug/regex grammar;
- normalize or trim valid IDs;
- change reference matching from exact equality;
- change namespace/global-uniqueness policy;
- add generated IDs;
- derive visible labels from IDs;
- alter layout or egui behavior;
- begin M18.

Validation belongs in parse/resolve, not the render loop.