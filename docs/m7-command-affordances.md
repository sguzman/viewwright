# M7 — Command Affordance Contracts

M6 completed the first visual-authoring/audit loop. The next earned gap is interaction intent.

ViewWright already authors command elements such as `Open`, `Play / pause`, and `Refresh`, and the egui backend already detects button clicks. But the blueprint cannot currently say what stable application action a command represents. The renderer therefore discards a meaningful interaction boundary.

M7 adds the smallest interaction-affordance contract needed to preserve that intent without turning ViewWright into an application runtime.

## Goal

A command should be able to declare a stable application-facing action identifier:

```toml
[[element]]
id = "play_pause"
region = "transport"
kind = "command"
importance = "primary"
label = "Play / pause"
action = "reader.play_pause"
```

The action identifier describes **what application action this affordance invokes**. It does not implement that action.

The host application remains responsible for behavior.

## Command action identifiers

For M7:

- every `command` element must declare `action`;
- non-command elements must not declare `action`;
- the action identifier is resolved into the backend-independent element model;
- multiple command elements may bind to the same action identifier;
- action identifiers are stable application contracts, not widget IDs.

Use a conservative namespaced textual form such as:

```text
document.open
reader.play_pause
reader.choose_voice
reader.choose_speed
dependency.refresh
```

Validate identifiers as non-empty dot-separated segments with no whitespace. Segments should be limited to simple ASCII alphanumeric / `_` / `-` characters. Do not build a URI or RPC schema.

## Static fixture-backed command state

Representative previews also need to express whether a command is available in a fixture.

M7 adds one narrow fixture payload:

```toml
[[fixture.content]]
element = "play_pause"
command = { enabled = false, reason = "Open a document first" }
```

Semantics:

- if no command-state payload is authored for a command in a fixture, it is enabled by default;
- `enabled = false` means the affordance is visibly disabled in that fixture;
- `reason` is optional representative explanatory text;
- command state is static fixture preview data, not a runtime condition language.

The resolved fixture model should carry typed command state.

## Renderer event boundary

The egui backend should no longer discard command clicks.

A click on an enabled command should produce a small backend-independent interaction event conceptually equivalent to:

```text
InteractionEvent {
    element_id,
    action,
}
```

The renderer must not execute application behavior.

It should only report that the authored affordance was activated.

Disabled commands must not emit activation events.

## Preview host

The isolated preview host may display the most recently emitted action in host chrome, for example:

```text
Last action: reader.play_pause (play_pause)
```

This is preview-tool state only. It is not canonical screen state and must not appear inside the authored specimen.

The preview host must not fake business logic in response to the action.

## Why this is not an event framework

M7 models only the command-to-action contract already justified by accepted screens.

It does not add:

- event bubbling
- event capture
- command parameters
- callbacks in TOML
- condition expressions
- reducers
- application state graphs
- async task execution
- keyboard shortcut systems
- menu systems
- navigation routing
- runtime action registries

Those remain outside ViewWright until separately earned.

## Canonical migration

The M7 pressure specimen is separate so accepted `main` remains valid before implementation exists.

After implementation supports `action` and command fixture state, migrate every canonical command element to an explicit stable action identifier.

At minimum this includes the existing Reader and Dependency Workbench commands.

Do not preserve a second legacy class of command elements whose action meaning exists only in labels.

## Projection boundary

Stable action identity is authored semantic intent, so it should be inspectable in semantic/debug output.

Concept output may include action identities when useful, but M7 does not require a major concept-spec redesign.

ASCII may remain structural.

## Business-logic boundary

The project charter remains authoritative:

> ViewWright may know that the command exists, where it belongs, how important it is, and what state enables it. The host application owns what the action actually does.

M7 stops at that line.