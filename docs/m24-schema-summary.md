# M24 — Schema Summary

No source syntax changes.

Existing arbitrary-name token maps remain:

- `[tokens.spacing]`
- `[tokens.corners]`
- `[tokens.color]`

M24 adds only a validation invariant on their authored keys: each key must contain at least one non-whitespace character. `tokens.type` is unchanged because its field names are fixed schema fields rather than arbitrary authored token names.