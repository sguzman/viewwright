# M30 — Fixture Content Reachability Fidelity

M3 established fixture-backed representative content for semantic elements, and M5/M7 extended that same model to tree, document, and command content. M16 deliberately permits otherwise-valid unused declarations. M29 then established a root-reachability concept for hierarchy intent.

Those accepted rules currently leave one contradiction: a fixture content record may reference an element that exists and has a compatible payload family even when that element belongs to a region outside the composition reachable from `screen.root`. The content resolves successfully but can never appear in the isolated preview.

## Principle

Fixture content is representative state for the authored screen, not storage for unreachable declarations.

For M30, every explicitly authored `fixture.content.element` must refer to an element whose owning region is structurally reachable from the validated `screen.root`.

This applies uniformly to all existing fixture payload families:

- collection items
- properties
- status text
- tree nodes
- document content
- command state

## Existing contracts preserved

- Missing element references retain their existing diagnostic.
- Payload-family / element-kind compatibility remains unchanged.
- Duplicate content records remain invalid.
- Exactly-one-payload-family validation remains unchanged.
- Fixtures may still omit content for any reachable element.
- Unused regions, compositions, and elements remain legal when no fixture content targets them.
- M18 global cycle validation remains global, including unused compositions.
- M29 dominant reachability remains independent.

## Root reachability

Reachability begins only from a validated `screen.root` and follows nested composition children to regions. An element is fixture-addressable only when its owning region is in that root-reachable region set.

If `screen.root` is missing or invalid, preserve the existing root error and do not manufacture a dependent fixture-content reachability diagnostic.

## Boundary

M30 adds no new TOML syntax, no mandatory fixture coverage, no global dead-declaration rule, no renderer/layout behavior, no application data binding, and no M31 work.

## Expected effect

Accepted canonical sources already target visible/root-reachable elements, so M30 should require no migration and have no intended visible delta. It only rejects fixture data that could never be projected into the authored screen.