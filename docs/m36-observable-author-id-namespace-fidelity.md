# M36 — Observable Author-ID Namespace Fidelity

## Problem

M17 intentionally required structural IDs to be nonblank without requiring `screen.id` to be globally unique against child namespaces.

That boundary became unsafe after the accepted ViewWitness bridge milestones:

- M33 exports exact authored screen, region, and element IDs in the intended expectation;
- M34 projects those same exact IDs into one AccessKit `author_id` channel;
- ViewWitness preserves that channel as `NodeIdentity.author_id`;
- M35 indexes observed nodes globally by exact `author_id` and treats multiple matches as ambiguity.

A source may therefore currently be valid while authoring the same string for `screen.id` and a region or element ID. Once rendered, two distinct semantic objects publish the same author identity. The comparator cannot truthfully choose one.

## Principle

Any ViewWright semantic object projected into the shared observation `author_id` channel must have unambiguous authored identity in that channel.

M36 therefore makes the observable semantic identity namespace unique across:

- the screen;
- every authored region;
- every authored semantic element.

## Exact rule

`screen.id` must not equal any authored `region.id` or any authored `element.id`.

The comparison is exact-string equality. Valid IDs remain byte-for-byte authored values; ViewWright must not prefix, suffix, hash, normalize, lowercase, trim, or otherwise rewrite them.

The rule applies to unused region and element declarations as well as root-reachable ones. Reachability changes must not turn a previously valid semantic identity into an observation collision.

## Existing uniqueness carried forward

Existing validation already keeps region, element, composition, and fixture structural IDs in their accepted shared namespace. M36 does not redesign that behavior.

The new pressure is specifically that `screen.id`, previously outside that global duplicate check, may collide with objects that M34 also publishes as `author_id`.

## Deliberately preserved separate namespaces

M36 does **not** require `screen.id` to differ from:

- composition IDs;
- fixture IDs;
- collection-item IDs;
- tree-node IDs;
- command action IDs;
- token names;
- labels or other copy.

Composition and fixture IDs are not M34 semantic author-ID anchors. Local fixture IDs are separate identity domains.

Do not implement M36 by blindly inserting `screen.id` into the existing all-structural-ID set if that would newly reject screen/composition or screen/fixture equality.

## Bridge preservation

M36 fixes the collision at source validation rather than weakening the bridge.

Preserve:

- M33 exact raw author IDs;
- M34 exact raw AccessKit `author_id` values;
- M35 exact author-ID matching and ambiguity behavior.

No bridge prefixes or generated surrogate IDs are introduced.

## Intended visible effect

None. Accepted canonical sources already use distinct screen, region, and element IDs.

## Non-goals

M36 does not add:

- a general identifier grammar;
- new ID newtypes;
- prefixes such as `screen:` / `region:` / `element:`;
- automatic ID generation;
- ViewWitness changes;
- comparator changes;
- AccessKit hierarchy changes;
- responsive layout;
- M37 work.
