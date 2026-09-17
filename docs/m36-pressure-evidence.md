# M36 — Pressure Evidence

M36 is earned by the accepted bridge sequence rather than by hypothetical naming policy.

## M17 pressure boundary

M17 made structural IDs nonblank but explicitly left `globally unique screen IDs against child namespaces` out of scope. The implementation validates `screen.id` separately while region, element, composition, and fixture IDs use the existing shared structural-ID set.

That was sufficient before observation identity became first-class.

## M34 pressure

M34 now projects exact existing ViewWright IDs into one AccessKit `author_id` channel for:

- the screen;
- root-reachable regions;
- rendered semantic elements.

It deliberately does not prefix or rewrite those IDs.

Therefore a source whose screen ID equals a rendered region/element ID produces two distinct AccessKit nodes with the same authored identity evidence.

## M35 pressure

M35 indexes ViewWitness nodes globally by exact `NodeIdentity.author_id` and preserves all matches. More than one match for an expected author ID is a typed ambiguity mismatch and dependent checks stop rather than guessing.

This is correct comparator behavior, but it reveals that current source validity can create unavoidable bridge ambiguity.

The bug is upstream of the comparator.

## Why source validation is the narrow fix

Changing M35 to consider object kind would require object-kind evidence not currently present in `NodeIdentity.author_id`.

Prefixing M34 IDs would violate its accepted exact-authored-ID contract and make observed author identity differ from intended M33 author identity.

Rejecting the collision at authoring time preserves the established exact identity bridge with the smallest semantic change.

## Reachability pressure

The check applies to unused region/element declarations too. An unused declaration can later become reachable without changing its ID; validity should not depend on whether a latent identity collision happens to be rendered today.

## Non-pressure

No equivalent bridge ambiguity is currently created by screen/composition or screen/fixture equality because compositions and fixtures are not M34 semantic AccessKit anchors.
