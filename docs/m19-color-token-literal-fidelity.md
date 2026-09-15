# M19 — Color Token Literal Fidelity

M2 established named color tokens as authored visual semantics and required malformed color literals to be diagnosable. The current resolver validates a color literal only when a visual role references that token through `color_ref`.

That leaves a hole: an authored but currently unreferenced color token may contain a malformed value and still survive resolution. If a blueprint has no `[visual]` block, malformed `[tokens.color]` literals are not examined at all.

## Principle

Authored token vocabulary must be valid even when a particular token is not currently consumed by the active visual profile.

Every value authored under `[tokens.color]` must be a valid ViewWright color literal.

For the current language, the accepted literal syntax remains exactly the existing six-digit hexadecimal form:

```text
#RRGGBB
```

Existing case behavior is preserved; M19 does not invent a new color syntax.

## Required behavior

- validate every authored color-token value, referenced or unreferenced;
- reject malformed unused color tokens even when `[visual]` is absent;
- preserve existing missing-reference diagnostics for visual roles;
- preserve existing referenced malformed-color rejection;
- keep valid unused color tokens legal;
- keep unused tokens from becoming a reachability/use requirement;
- keep resolved palette and renderer behavior unchanged for valid blueprints.

## Boundary

M19 does not add token-name grammar, require color tokens to be referenced, expose unused-token warnings, add alpha/short hex/named colors/CSS syntax, preserve an unused-token map in `ResolvedBlueprint`, redesign visual profiles, or begin M20 work.

Validation must remain deterministic. If implementation iterates the color-token map, use stable key order rather than relying on `HashMap` iteration.
