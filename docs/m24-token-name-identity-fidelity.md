# M24 — Token Name Identity Fidelity

## Problem

ViewWright treats spacing, corner, and color entries as named authoring tokens, but their map keys currently receive no identity validation. Empty or whitespace-only quoted TOML keys can therefore exist and may even be referenced exactly.

## Principle

A named token must actually have a name.

Every authored key under `tokens.spacing`, `tokens.corners`, and `tokens.color` must contain at least one non-whitespace character. Validation may use `name.trim().is_empty()` only to detect invalidity. Valid token names are preserved exactly and references remain exact-string references.

## Boundary

M24 adds no token-name grammar, normalization, ASCII restriction, namespace, case policy, unused-token warning, token-use requirement, token registry redesign, type-token redesign, renderer/layout change, or M25 work.