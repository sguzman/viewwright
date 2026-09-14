# M3 Implementation Boundary

M3 changes only what is necessary to support fixture-backed representative content for the dependency-workbench pressure specimen.

Expected implementation areas:

- source fixture-content types
- strict fixture-content parsing
- validation of element references and payload compatibility
- resolved typed fixture content
- preview fixture lookup by selected fixture id
- egui rendering of collection items, selection, property pairs, and status/text from resolved fixture content
- migration of existing fixture heuristics where the new model directly replaces them
- tests
- preview registration for the dependency-workbench specimen

Unexpected scope that should trigger review before implementation continues:

- arbitrary application data binding
- runtime event/action graphs
- editable forms
- generalized table/grid schemas
- generalized hierarchical tree data
- networking, databases, file loading, or host-app integration
- dependency-specific renderer branches
- new visual token families
- responsive layout systems
- reusable component authoring
- ViewWitness integration

## Architectural rule

Fixture content is static preview/test evidence owned by canonical ViewWright source. It is not the application's model layer.

The source and resolved models should retain semantic element references; egui-specific widget state must stay below the resolved blueprint.

## Renderer rule

For element families supported by M3, renderers should not invent representative content when fixture content exists.

Avoid semantic behavior based on fixture-id substring matching. Explicit fixture content is the authority.

## Runtime rule

Parse and resolve fixture content before the normal frame loop. The UI thread may switch among already-resolved fixtures and render their data.

## Worker cleanup rule

Any preview window/process launched for implementation QA must be closed before the worker reports completion unless explicitly told otherwise.