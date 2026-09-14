# M2 — Visual Authoring Pressure Test

M0 proved that ViewWright can author a semantic screen and project it into ASCII and egui. M1 proved that the structure can be genuinely nested without becoming application-specific.

M2 addresses the original reason the project exists: structurally correct interfaces can still look terrible.

The goal is not to recreate CSS. The goal is to preserve a small amount of **visual authorship** above the toolkit level so that the same intent can inform a real renderer and a concept-art brief.

## Two different kinds of visual information

ViewWright should deliberately distinguish:

### Authorial character

Examples:

- quiet
- technical
- crafted
- reading-first
- dense but breathable
- restrained

And avoidances such as:

- pill soup
- border everything
- nested card soup
- uniform visual weight
- toolbar dominance

These are meaningful design constraints for humans, agents, concept generation, and linting, but they are not exact renderer instructions.

### Renderable visual semantics

These are concrete enough for a backend to resolve and apply:

- semantic color roles
- a small typographic scale
- region surface roles
- border policy
- corner policy
- existing spacing rhythm
- semantic importance

The two layers should cooperate without being conflated.

## Small visual profile

M2 should support one screen-local visual profile. Theme inheritance and shared application themes remain deferred.

Desired source shape:

```toml
[tokens.color]
canvas = "#101216"
surface = "#171A20"
surface_raised = "#1E222A"
text = "#E8EAF0"
text_muted = "#9299A6"
accent = "#8DB7C7"
border = "#2B313B"

[tokens.type]
display = 26
heading = 18
body = 14
caption = 12

[visual]
canvas = "canvas"
surface = "surface"
surface_raised = "surface_raised"
text = "text"
text_muted = "text_muted"
accent = "accent"
border = "border"
corner = "medium"
border_policy = "minimal"
```

The visual profile references named tokens rather than embedding toolkit colors throughout the screen.

## Semantic surface roles

Regions need one small visual distinction that cannot be recovered reliably from role or importance alone: **surface treatment**.

M2 candidate values:

- `canvas` — dominant continuous work/document surface
- `panel` — ordinary supporting surface
- `raised` — visually separated command/inspection surface
- `transparent` — no independent surface treatment

Example:

```toml
[[region]]
id = "reader"
role = "primary_content"
importance = "primary"
surface = "canvas"
grow = 1

[[region]]
id = "transport"
role = "commands"
importance = "secondary"
surface = "raised"
height = "68px"
```

This is deliberately not arbitrary per-region styling. A region chooses a semantic treatment; the visual profile determines the concrete values.

## Typography

M2 should not add per-widget font declarations.

The backend should derive text hierarchy from existing semantics such as:

- screen purpose / display context
- element kind
- importance
- region role

The visual profile supplies the small type scale that the renderer maps onto those semantics.

A later milestone may earn explicit semantic text roles if real specimens demonstrate that inference is insufficient.

## Borders and corners

M2 needs policy, not a box-model language.

The visual profile may select:

- `border_policy = "none" | "minimal" | "defined"`
- a named corner token such as `small` or `medium`

The egui backend should use these policies consistently rather than wrapping every region and element in default groups.

The project should explicitly resist the failure mode where every semantic object becomes a bordered rounded rectangle.

## Concept specification projection

M2 should add a **concept specification** projection.

This is textual, deterministic, and non-authoritative. It converts the resolved blueprint into a concise design brief suitable for:

- human discussion
- image-generation prompting
- visual review
- comparing alternate visual directions

It should include at least:

- screen purpose
- structural hierarchy
- dominant and supporting regions
- design character
- avoidances
- semantic palette
- type scale
- region surface hierarchy
- important element kinds

It should not directly call an image-generation service. ViewWright owns the specification; an external agent/tool may use that specification to create concept art.

This preserves the boundary:

```text
ViewWright blueprint
    ↓
resolved semantic + visual model
    ├── ASCII
    ├── egui
    └── concept specification
             ↓
       optional image generation
```

Generated imagery remains exploratory evidence, never source of truth.

## Why concept specification comes before image integration

Direct image generation from arbitrary prose would recreate the original problem: visual decisions would exist outside the canonical model and be difficult to reconcile with implementation.

The concept specification gives both the renderer and the visual exploration process a shared authored basis.

## Token validation

Visual token references must resolve before projection.

M2 should diagnose:

- malformed color literals
- missing visual color tokens
- missing type tokens required by the profile
- invalid corner token reference
- unknown surface roles
- invalid border policy

The resolved visual model should contain concrete values rather than repeatedly interpreting source strings inside renderers.

## Unknown source fields

A canonical authoring language must not silently discard intended design information.

M2 should make unknown authored fields diagnosable rather than allowing a typo or unsupported visual field to disappear silently during TOML deserialization.

This matters especially once the format carries visual intent: `surafce = "raised"` must not quietly become "use the default".

## egui pressure test

The M2 egui goal is not production beauty. It is to prove that visual semantics materially affect presentation.

For the visual reader specimen, the backend should visibly establish:

- the document area as the dominant canvas
- supporting library and inspector panels as quieter surfaces
- transport/command surfaces as separated without excessive borders
- meaningful typographic hierarchy
- the authored palette
- restrained corner/border policy
- spacing rhythm already supplied by the blueprint

The result should look intentionally composed rather than like default egui groups nested inside default egui groups.

## Explicit non-goals

M2 does not include:

- CSS-like selectors
- arbitrary per-widget styling
- reusable authored components
- theme inheritance
- font-file loading or font-family management
- gradients
- shadows/elevation engines
- animation
- responsive breakpoints
- arbitrary layout constraints
- image generation APIs
- image-to-blueprint reconstruction
- Figma import/export
- ViewWitness comparison
- generalized accessibility theming

## Acceptance principle

M2 succeeds when the visual reader specimen can be understood from TOML, resolved into concrete visual semantics, projected into a useful concept specification, and rendered by egui with visibly intentional hierarchy without application-specific styling code.

The original Project Browser and non-visual structural semantics must remain valid. If existing specimens require a small canonical visual profile migration, make it explicit rather than adding hidden backend defaults that erase authorship.
