# M17 — Structural Identifier Fidelity

## Problem

ViewWright treats IDs as stable semantic identity throughout validation, resolution, fixtures, layout, projections, and backend state. The model document explicitly requires projections to preserve stable semantic IDs wherever feasible.

The current resolver validates duplicate IDs and reference existence, but it does not require general structural IDs to contain any non-whitespace character.

That is not merely cosmetic. Root resolution currently uses an empty `String` as an internal failure sentinel. A composition may also currently author `id = ""`, and `screen.root = ""` can therefore collide with that sentinel. Root-dependent validation then uses `if !root.is_empty()`, so an authored empty identity can bypass validation paths intended only for failed root resolution.

Local collection-item and tree-node IDs have the same identity role inside fixture payloads: selection and tree-parent relationships address them by ID, yet blank local IDs are currently accepted.

## Principle

Identity must actually identify.

Every authored structural ID must contain at least one non-whitespace character. Valid authored IDs are preserved exactly; ViewWright must not trim, normalize, slug, lowercase, title-case, or otherwise rewrite them.

## Scope

M17 applies the nonblank identity invariant to:

- `screen.id`;
- `region.id`;
- `element.id`;
- `composition.id`;
- `fixture.id`;
- collection item `id` values;
- tree node `id` values.

An ID is invalid when `id.trim().is_empty()`.

M17 does not introduce a broader identifier grammar.

## Root-resolution boundary

M17 must remove the semantic ambiguity between an authored ID and the resolver's internal invalid-root sentinel.

The implementation may use `Option<String>`, a small local enum, or an equivalent explicit internal state while validating `screen.root`.

A successful `ResolvedBlueprint.root` may remain a `String`; after successful validation it must refer to a valid nonblank composition ID.

## References

M17 does not add a second normalization rule for references. Existing exact-string reference semantics remain authoritative.

Because targets can no longer have blank identity, blank or whitespace-only references cannot resolve successfully to structural targets and existing missing-reference diagnostics remain valid.

## Non-goals

M17 does not add:

- general ID newtypes;
- regex/slug requirements;
- ASCII-only IDs;
- case normalization;
- underscore/hyphen policy;
- globally unique screen IDs against child namespaces;
- global uniqueness for local collection/tree IDs;
- automatic ID generation;
- display-name derivation from IDs;
- localization;
- reusable component instance identity;
- M18 work.

## Intended visible effect

None. Accepted canonical sources already use nonblank IDs. M17 is a validation and resolution-fidelity milestone.