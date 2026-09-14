# M1 Schema Pressure — Reader Workspace

M0 proved that ViewWright can carry one authored screen from TOML through validation/resolution into ASCII and egui projections without reducing the source language to serialized widget calls.

M1 should not begin by adding a broad feature list. It begins with a second real screen whose structure differs materially from the M0 project browser.

The pressure specimen is `specimens/reader-workspace.toml`.

It represents a reading workspace with:

- a top command region
- a horizontal reading body
- library/navigation at left
- a dominant document surface in the center
- a tertiary inspector at right
- a bottom transport/status region

The important difference from M0 is that the screen cannot be represented cleanly as one flat split. It requires a vertical composition containing a nested horizontal composition.

## What M0 cannot express cleanly

### 1. No explicit composition root

M0 stores a list of compositions and the egui renderer uses the first composition as the root.

Ordering in a TOML array should not define semantic root authority.

The screen should explicitly name its root composition.

Proposed source shape:

```toml
[screen]
id = "reader_workspace"
root = "workspace"
```

Resolution should turn this into a typed root reference.

### 2. Composition cannot recursively contain composition

M0 composition children resolve only against regions/elements.

The reader workspace needs:

```text
workspace
├── app_commands
├── reading_body
│   ├── library
│   ├── reader
│   └── inspector
└── transport
```

`reading_body` is itself a composition.

M1 therefore needs composition nodes to reference child composition nodes.

This should remain a composition tree, not become an arbitrary scene graph.

Validation should reject at least:

- missing composition references
- cycles
- an invalid root

Multiple-parent semantics should remain disallowed or explicitly deferred unless a real screen requires them.

### 3. Vertical fixed size intent

M0 regions support fixed width and grow weight, but not fixed height.

Top command strips and bottom transport/status surfaces need simple vertical size intent.

M1 should add the smallest symmetric extension:

```toml
height = "52px"
```

This does not justify a complete constraint language.

Minimum/maximum/intrinsic sizing remain deferred unless implementation demonstrates they are required for this specimen.

### 4. A document surface is semantically distinct

The existing M0 element vocabulary does not have a good semantic term for the central authored reading surface.

Using `text` would throw away meaningful intent, while `preview` would be incorrect: the document is the primary object, not a preview of another object.

M1 should add one semantic element kind:

```toml
kind = "document"
```

This means a primary document/content reading surface. It does not prescribe a text widget, webview, rich-text engine, pagination system, or EPUB implementation.

### 5. Fixture semantics remain intentionally weak

The reader specimen includes states such as:

- document loaded
- outline selection
- settings visible
- no document

M1 should not turn fixtures into a general state/data-binding language.

The specimen is allowed to expose fixture limitations, but recursive composition is the primary M1 concern.

## Minimal M1 schema response

M1 should implement only the following schema changes unless coding exposes a genuine blocker:

1. `screen.root` as an explicit composition reference.
2. Composition children may reference regions or other compositions.
3. Recursive composition resolution with cycle detection.
4. Fixed region `height` symmetric with M0 fixed `width`.
5. `document` as a semantic element kind.
6. ASCII and egui projections traverse the resolved composition tree rather than assuming one flat root split.

## Non-goals

The reader specimen does **not** justify adding:

- reusable authored components
- arbitrary nested widget trees
- general data binding
- responsive breakpoints
- animation
- scrolling ontology
- docking systems
- arbitrary overlays/popovers
- EPUB-specific concepts
- TTS engine concepts
- application commands/actions
- ViewWitness integration
- theme inheritance
- full min/max/intrinsic constraint solving

Those may become real later, but they have not yet been earned by this pressure test.

## Acceptance signal for M1

M1 is successful when both the original Project Browser specimen and the Reader Workspace specimen can resolve into the same backend-independent composition model and project recognizably into ASCII and egui.

The project browser must not regress or require special-case renderer logic.

The reader workspace should demonstrate that nested composition is real semantic structure rather than hard-coded layout knowledge in the backend.

## Architectural principle reinforced

ViewWright should grow by encountering interfaces that its current ontology cannot faithfully describe.

A new abstraction is justified when a real screen requires it, not merely because the abstraction is imaginable.
