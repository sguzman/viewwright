# M7 — Command Affordance Acceptance

M7 is accepted when ViewWright can preserve stable command action intent through parsing, resolution, fixture preview state, and backend interaction reporting without taking ownership of application behavior.

## Source / model

Required:

- `command` elements author a stable `action` identifier;
- action identifiers resolve into the backend-independent element model;
- non-command elements reject `action`;
- command elements missing `action` are rejected after canonical migration;
- malformed action identifiers are rejected with useful diagnostics.

## Static command fixture state

Support fixture content of the form:

```toml
[[fixture.content]]
element = "play_pause"
command = { enabled = false, reason = "Open a document first" }
```

Required semantics:

- absent command-state payload means enabled;
- disabled state is typed and resolved before rendering;
- optional reason survives resolution;
- command payload on non-command elements is rejected;
- command payload participates in existing one-payload-family-per-record validation;
- unknown command-state fields are rejected.

## Backend interaction result

The egui backend must expose activation rather than discarding it.

When an enabled command is clicked, the renderer reports at least:

- semantic element id;
- stable authored action id.

It must not execute host behavior.

Disabled commands must not report activation.

## Preview-host proof

The preview host should make the event boundary visible outside the authored specimen, for example by showing the latest emitted action.

Human QA should be able to verify:

- `reading` fixture: `Play / pause` is enabled and clicking it reports `reader.play_pause`;
- `empty` fixture: `Play / pause` is visibly disabled and clicking it emits nothing;
- `Open` remains enabled in the empty Reader and reports `document.open`;
- the authored screen itself contains no diagnostic event log.

## Canonical migration

After parser/model support exists, migrate canonical commands to explicit action identifiers.

At minimum verify:

- visual Reader commands;
- structural Reader commands if present;
- Dependency Workbench `Refresh`;
- any other accepted canonical command elements found in the repository.

Do not leave action meaning dependent only on labels.

## Tests

Add meaningful tests covering at least:

- valid namespaced action id;
- malformed action id rejection;
- command missing action rejection;
- action on non-command rejection;
- duplicate action ids across separate command elements remain legal;
- disabled command fixture state resolves;
- enabled command fixture state / default resolves;
- command state on non-command rejection;
- mixed command payload + another fixture payload rejection;
- unknown command-state field rejection;
- optional disabled reason survives resolution;
- enabled command produces an interaction event in renderer/backend-level test where practical;
- disabled command produces no interaction event where practical;
- accepted M0–M6 model/layout/audit tests remain passing.

## Non-goals

M7 does not require:

- execution of actions;
- application callbacks;
- action parameters;
- condition-expression language;
- runtime state synchronization;
- generalized interaction state machines;
- navigation routing;
- keyboard shortcut ontology;
- menus;
- ViewWitness integration.

## Worker hygiene

Any preview/process launched for QA must be closed before completion is reported unless explicitly requested otherwise.