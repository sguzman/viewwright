# M7 — Schema Summary

M7 adds only the smallest authoring needed for command affordance identity and representative enabled state.

## Element extension

Command element:

```toml
[[element]]
id = "play_pause"
region = "transport"
kind = "command"
importance = "primary"
label = "Play / pause"
action = "reader.play_pause"
```

Rules:

- `action` is required for `kind = "command"` once canonical migration is complete;
- `action` is invalid on non-command kinds;
- actions are stable namespaced identifiers, not Rust callbacks or toolkit commands.

## Fixture command state

```toml
[[fixture.content]]
element = "play_pause"
command = { enabled = false, reason = "Open a document first" }
```

Rules:

- `command` fixture payload is valid only for command elements;
- absent payload means enabled;
- `enabled` is required inside the payload;
- `reason` is optional representative text;
- the record must not mix command state with collection/properties/text/tree/document payload families.

## Resolved shape

Conceptually:

```text
ResolvedElement
  action: Option<ActionId>

ResolvedFixtureContent::Command {
  element,
  enabled,
  reason,
}
```

A backend activation result is conceptually:

```text
InteractionEvent {
  element_id,
  action,
}
```

No source syntax for callbacks, event handlers, conditions, or business logic is introduced.