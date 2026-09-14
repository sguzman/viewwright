# M6 — Schema Summary

M6 adds **no new canonical authoring syntax**.

This is deliberate.

The pressure is not that ViewWright lacks another visual token or style declaration. The pressure is that authored M2 visual semantics can resolve successfully while still producing a palette whose surface hierarchy is visually compressed.

M6 therefore operates entirely on the existing resolved visual model.

## Existing canonical inputs

The audit consumes the already-supported semantic palette roles:

- `canvas`
- `surface`
- `surface_raised`
- `text`
- `text_muted`
- `accent`
- `border`

It also inspects which semantic region surface roles are actually used:

- `canvas`
- `panel`
- `raised`
- `transparent`

## No authored thresholds

M6 does not add source fields such as:

- `min_contrast`
- `surface_separation`
- `brightness`
- `darkness`
- `accessibility_level`
- `audit_policy`

Initial thresholds are implementation-level lint heuristics documented by the milestone.

If later pressure demonstrates that projects need authored audit policy, that should be a separate earned schema change.

## Retained regression specimen

A non-canonical Reader regression specimen retains the pre-M6 color-token values to exercise the audit.

That is not a schema extension.

## Resolved output

M6 adds derived audit output, not source authority.

Conceptually:

```text
ResolvedBlueprint
    ↓
VisualAudit
    - palette luminance metrics
    - foreground contrast metrics
    - surface separation metrics
    - advisory findings
```

The audit is a projection and must not be serialized back into canonical TOML as if it were authored intent.
