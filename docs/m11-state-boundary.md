# M11 — Renderer State Boundary

M11 introduces or formalizes only one category of mutable state: ephemeral egui projection state needed to keep Search controls editable across frames.

## Canonical versus ephemeral

Canonical ViewWright data remains:

```text
TOML
  -> parsed source
  -> validated/resolved blueprint
  -> semantic / ASCII / concept / layout / egui projections
```

Search query text is not inserted into that pipeline.

Instead:

```text
resolved blueprint + backend-local render state
  -> egui projection
```

## Host ownership

The preview host owns the lifetime of backend-local state.

The renderer may provide a small state type and helpers, but it should not rely on process-global mutable storage.

The preview host is responsible for clearing or replacing ephemeral state when the selected specimen or fixture changes.

## Why explicit state

Explicit host-owned state makes the boundary visible:

- authored intent remains deterministic;
- renderer-local interaction continuity is possible;
- application/business state is not smuggled into the blueprint;
- future backends are not forced to copy egui implementation details.

M11 does not generalize this into a full application-state framework.
