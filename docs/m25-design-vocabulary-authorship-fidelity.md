# M25 — Design Vocabulary Authorship Fidelity

## Problem

`design.character` and `design.avoid` are explicit authorial design vocabulary. They survive into `ResolvedDesign` and are emitted directly by the concept-specification projection, but individual authored entries currently receive no content validation. Empty or whitespace-only entries therefore resolve as blank design instructions.

## Principle

If an author explicitly supplies a design-vocabulary entry, it must contain at least one non-whitespace character.

M25 applies to each string in:

- `design.character`
- `design.avoid`

Validation may inspect `entry.trim().is_empty()` only for blankness. Valid entries are preserved exactly.

## Boundary

The lists themselves remain optional/default-empty. M25 adds no minimum list length, taxonomy, enum, uniqueness rule, normalization, case policy, identifier grammar, localization, concept redesign, renderer/layout change, or M26 work. `design.dominant` remains governed by M15.