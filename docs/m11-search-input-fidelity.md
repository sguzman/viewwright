# M11 — Search Input Fidelity

Accepted ViewWright specimens already author Search elements in the Project Browser, Reader Workspace, and Dependency Workbench.

The current egui projection renders those elements with a fresh local `String::new()` on every frame. The field therefore looks editable but cannot honestly retain typed text across frames.

M11 makes Search a truthful isolated-preview affordance without turning ViewWright into an application runtime.

## Required semantic boundary

Search text is **backend-local ephemeral projection state**.

It is not:

- canonical TOML;
- fixture content;
- resolved blueprint semantics;
- business state;
- a filter expression;
- an application event;
- persisted application data.

The canonical blueprint continues to say only that a Search affordance exists and what its authored label is.

## Required projection behavior

For a selected specimen + fixture, text entered into a Search element must survive normal frame-to-frame redraws.

The egui backend should consume explicit mutable renderer state owned by the host (or an equivalently explicit backend-local state object). Do not hide search values in process globals.

State should be keyed by stable authored element identity. Multiple Search elements must not share one value accidentally.

The preview host should clear or rebind ephemeral search state when changing specimen or fixture so one design state does not contaminate another.

## No filtering

Typing into Search does not filter collections, trees, cards, documents, or properties in M11.

No search-change interaction event is required.

M11 is about input continuity only: an editable control should remain editable across frames.

## Existing pressure

Canonical examples include:

- `project_browser` → `project_search` / `Search projects`;
- `reader_workspace_visual` → `library_search` / `Search library`;
- `dependency_workbench` → `package_search` / `Search dependencies`;
- M9 density pressure specimens → `search`.

## Architectural rule

Keep mutable preview input state outside `ResolvedBlueprint`.

ViewWright source and resolved semantics remain deterministic and serializable. Backend-local interactive state belongs to the renderer/host boundary.
