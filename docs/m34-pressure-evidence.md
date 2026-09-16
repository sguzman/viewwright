# M34 — Pressure Evidence

## M33 expectation export

M33 now exports exact authored screen, region, and element IDs as intended identity. A future comparator must not guess how those IDs correspond to observed GUI nodes.

## ViewWitness evidence contract

Current ViewWitness egui capture maps:

```text
AccessKit Node.author_id()
    → NodeIdentity.author_id
```

That field is explicitly preserved as application-authored identity evidence separate from the witness-local AccessKit node ID.

Therefore ViewWright can create a trustworthy bridge without changing ViewWitness if its egui projection publishes its semantic IDs through AccessKit `author_id`.

## Current ViewWright gap

The accepted ViewWright egui backend renders semantic regions/elements through ordinary egui scopes/widgets but does not currently set AccessKit `author_id`.

Thus an observation today may expose labels, roles, bounds, and structure but lacks exact ViewWright semantic identity.

## Pressure screens

### Project Browser

Provides multiple sibling regions and multiple semantic elements:

- `navigation`
- `projects`
- `inspector`
- `project_search`
- `navigation_items`
- `project_collection`
- `project_inspector`

This is the baseline identity hierarchy pressure.

### M31 overlay

`palette_surface` must remain an authored floating-region identity rather than being confused with overlapping base content.

### M32 overflow

`reader` and its semantic elements must retain authored identity even when their ordinary egui children live inside a ScrollArea and move with scrolling.

## Why comparison is deferred

Identity evidence must exist before comparison. M34 earns the identity channel only; M35 or later may pressure actual intended-vs-observed comparison.