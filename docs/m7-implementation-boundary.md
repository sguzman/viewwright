# M7 — Implementation Boundary

M7 is an interaction-affordance milestone, not an application runtime milestone.

## In scope

- typed `action` identity on command elements;
- validation of action identifiers;
- command-only action compatibility rules;
- typed static fixture-backed command enabled/disabled state;
- optional representative disabled reason;
- egui reporting of enabled command activation;
- preview-host display of the most recent emitted action;
- canonical migration of accepted command elements;
- semantic/debug visibility of action identity and command fixture state;
- preservation of M0–M6 behavior.

## Renderer boundary

The egui backend may detect a lightweight command click and emit a small semantic activation result.

It must not:

- execute the action;
- own callbacks;
- invoke application services;
- mutate canonical fixture state;
- pretend to implement Reader or Dependency Workbench business behavior.

A renderer API change from `show(...) -> ()` to a small result/output type is appropriate if needed.

Keep the output backend-neutral in meaning even if the concrete API lives in the egui crate.

## Fixture boundary

Fixture command state exists only to preview representative availability.

It does not model why a real application is enabled or disabled at runtime.

Do not add:

- boolean expressions;
- dependency graphs;
- permissions systems;
- loading/task state machines;
- state transitions;
- computed predicates.

The optional `reason` is authored representative UI explanation only.

## Action identity boundary

An action id is a stable semantic application contract.

It is not:

- a URL;
- a function path;
- a Rust symbol;
- a serialized closure;
- a plugin invocation;
- a message-bus schema.

Prefer simple dotted namespacing.

Multiple UI affordances may intentionally refer to the same action.

## Preview host

The host may remember the last emitted activation solely for inspection.

That state lives in preview tooling, not in the blueprint.

Do not modify fixture content in response to emitted actions.

## Threading rule

Click detection and appending a tiny activation result are normal render-thread work.

No heavy work, I/O, action execution, or application dispatch may be introduced on the UI thread.

## Stop rather than widen

If implementation pressure appears to require:

- action parameters;
- generalized events;
- search-submit semantics;
- collection-selection events;
- navigation routing;
- keyboard shortcuts;
- reactive state;
- async command execution;

stop and report the pressure instead of implementing it as part of M7.