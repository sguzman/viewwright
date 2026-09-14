# M5 — Reader Fixture Honesty

M3 established canonical fixture-backed representative content for collections, property sheets, and status text. M4 then made major geometry honest. Human runtime QA of the Reader Workspace exposed the next preview-fidelity gap: the dominant reader surface and outline are still largely backend placeholders rather than authored representative state.

This matters because ViewWright's visual-authoring goal cannot be judged well against an almost-empty reader. Before treating the current dark Reader palette as inherently wrong, the preview should contain enough canonical document/outline/settings/status content to make legibility and hierarchy visible.

M5 extends the fixture-content layer only far enough to make the Reader specimen honest.

## Goal

Replace Reader-specific placeholder rendering and fixture-name heuristics with canonical fixture content for:

- tree/outline structure
- a plain representative document
- existing property-sheet settings
- existing status text

The intended pipeline remains:

```text
TOML fixture content
    ↓
validation / resolution
    ↓
typed resolved fixture content
    ↓
semantic preview
```

M5 is not a document engine, tree widget framework, or application state system.

## Tree fixture content

A tree payload is authored as a flat ordered list of stable nodes:

```toml
[[fixture.content]]
element = "document_outline"
nodes = [
  { id = "introduction", label = "Introduction" },
  { id = "part_one", label = "Part I" },
  { id = "chapter_one", label = "Chapter 1 · The Quiet Machine", parent = "part_one" },
  { id = "chapter_two", label = "Chapter 2 · The City at Night", parent = "part_one" },
]
selected = "chapter_two"
```

Each node has:

- stable `id`
- display `label`
- optional `parent` node id

`selected` is optional and references a node id in the same tree payload.

The flat source shape is deliberate. It keeps validation and serialization simple while still expressing hierarchy.

M5 does not add expanded/collapsed runtime state, drag/drop, lazy loading, checkboxes, icons, columns, or arbitrary node metadata.

## Document fixture content

A document payload is intentionally plain:

```toml
[[fixture.content]]
element = "document_surface"
document = {
  title = "The Quiet Machine",
  paragraphs = [
    "Rain gathered on the windows while the city settled into its evening rhythm.",
    "The machine remained awake, listening to the building around it.",
  ]
}
```

A document contains only:

- `title`
- ordered plain-text `paragraphs`

No markup, spans, images, pagination, chapters, annotations, links, layout objects, or rich-text semantics are introduced.

An empty Reader fixture should author its empty presentation explicitly rather than depend on the fixture id. For example, it may provide a document with title `No document loaded` and no paragraphs.

## Existing M3 families remain authoritative

Reader settings should use existing property content.

Reading status should use existing text content.

M5 must not create competing payload families for information M3 already models.

## Validation

Tree content must diagnose at least:

- missing referenced element
- incompatible element kind
- duplicate content record for one element in a fixture
- duplicate node ids
- missing parent node
- self-parenting
- parent cycles
- `selected` node that does not exist
- incompatible payload families in one record
- unknown fields

Document content must diagnose at least:

- incompatible element kind
- missing/invalid document structure
- incompatible payload families in one record
- unknown fields

Existing M0–M4 validation remains intact.

## Resolved model

Renderers should receive typed backend-independent variants conceptually equivalent to:

```text
Tree {
    element,
    nodes: [{ id, label, parent }],
    selected
}

Document {
    element,
    title,
    paragraphs
}
```

The renderer must not parse arbitrary source maps or infer document/tree state from fixture names.

## Reader pressure specimen

For a loaded reading fixture, the preview should contain authored:

- outline nodes
- selected chapter
- document title
- several representative paragraphs
- reading settings
- reading status

For an empty fixture, the preview should visibly become an authored empty state without using fixture-id substring checks.

The visual Reader should then be useful for a fresh human judgment of palette legibility and hierarchy.

## Runtime boundary

Fixture parsing/validation/resolution remains outside the render loop.

Rendering already-resolved static tree/document content is lightweight immediate-mode work.

No file loading, EPUB parsing, TTS work, database access, or network work enters the frame path.

## Explicit non-goals

M5 is not:

- a rich-text model
- Markdown/HTML authoring
- EPUB parsing
- pagination
- scrolling architecture
- document persistence
- annotations
- hyperlink semantics
- editable documents
- generalized tree runtime state
- expanded/collapsed authored state machines
- drag/drop
- tree virtualization
- runtime data binding
- event/action wiring
- TTS semantics
- interaction state machines
- responsive layout
- new visual vocabulary
- ViewWitness integration

## Stop condition

Stop M5 once the accepted Reader specimens can render representative outline, document, settings, and status state entirely from typed resolved fixture content, the empty state is authored rather than inferred from the fixture id, and accepted M0–M4 behavior remains intact.