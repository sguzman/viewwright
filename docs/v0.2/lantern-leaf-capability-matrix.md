# Lantern Leaf capability matrix

This matrix compares the archived Lantern Leaf EPUB + TTS reader concept with ViewWright 0.1.0.

Legend:

- **Native** — current ViewWright can represent the intent directly enough to treat it as authored truth.
- **Approximate** — current primitives can mimic the idea, but important semantics or structure are lost.
- **Missing** — the intent cannot be represented honestly without extending ViewWright.
- **Runtime boundary** — the behavior itself belongs to the host application, though representative design state may still need fixture support.

## Shell and major layout

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Screen identity and purpose | Native | `screen.id`, `screen.purpose` | none |
| Comfortable/dense overall rhythm | Native | `screen.density` | only two global policies |
| Library + reader + inspector horizontal shell | Native | row/split composition | none for wide shell |
| Fixed left rail around 260 px | Native | region width | none |
| Growing center reader | Native | region/composition grow | none |
| Fixed inspector around 330 px | Native | region width | none |
| Persistent bottom TTS surface around 96 px | Native | outer column + fixed-height region/composition | none |
| Nested major topology | Native | recursive compositions | elements cannot participate directly |
| Vertical scrolling | Native | `overflow = "scroll_y"` | vertical only |
| Overlay/floating surface | Native | M31 overlay | constrained to accepted overlay topology |
| Asymmetric local padding such as 20×28 | Missing | uniform composition padding only | local box/layout intent needed |
| Local align/justify | Missing | backend decides | furnishing layout semantics needed |
| Local fixed width/grow on controls/groups | Missing | only major regions/compositions size this way | local layout semantics needed |

## Semantic content and controls

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Navigation list | Native | collection/list fixture | icons absent |
| Table of contents hierarchy | Native | tree fixture | richer book metadata optional |
| Search field | Native | Search element | search execution intentionally host-owned |
| Text/status labels | Native | Text / Status | none for simple copy |
| Push commands | Native | Command + action ID + enabled fixture state | icon-only presentation absent |
| Current-book summary | Approximate | Text + PropertySheet + Collection | no media/image-summary semantic object |
| Book cover/image | Missing | generic Preview is not an asset binding | asset/media semantics |
| Toolbar | Approximate | Commands region with flat horizontal flow | no nested clusters or justify |
| Command cluster | Missing | flatten into region element order | local semantic grouping |
| Settings group | Approximate | PropertySheet or extra regions | loses control semantics; region inflation |
| Tabs | Approximate | collection/commands | no semantic selected choice presentation |
| Select/dropdown | Missing | property text only | choice semantic family |
| Slider/range input | Missing | property text only | scalar adjustment semantic family |
| Toggle/switch | Missing | command/property approximation | boolean control semantics |
| Segmented choice | Missing | commands/collection approximation | semantic choice + presentation |
| Color picker | Missing | property approximation | color-valued control semantics |
| Preset grid | Approximate | adaptive-card collection | item metadata/preview colors limited |
| Progress/scrubber | Missing | status text only | progress/seek semantics |
| Waveform scrubber | Missing | none | specialized presentation may project from progress/seek semantics |
| Volume control | Missing | property/command approximation | scalar adjustment |
| Icon button / icon identity | Missing | label-only command | asset/icon authoring |
| Media summary | Missing | Text/PropertySheet approximation | media identity + asset relationship |

## Element structure

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Elements belong to major regions | Native | `element.region` | none |
| Toolbar → cluster → buttons hierarchy | Missing | elements are flat inside a region | nested furnishing tree |
| Settings group → controls hierarchy | Missing | flatten or create fake regions | nested furnishing tree |
| TTS transport → controls + progress | Missing | flatten or fake regions | nested furnishing tree |
| Local row/column/grid inside region | Missing | only region/composition topology | element/group layout |
| Reusable semantic grouping without new visual panel | Missing | regions imply major semantic/layout surface | group/container primitive above widgets |

This is the most important structural gap. Today, achieving Lantern Leaf-like local composition requires inflating local UI groupings into first-class regions. That distorts ontology and motivates the first post-baseline language pressure.

## Document and reading surface

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Document title | Native | fixture Document title | none |
| Ordered paragraphs | Native | fixture Document paragraphs | none |
| Chapter eyebrow | Approximate | title/text | no document block taxonomy |
| Heading levels | Missing | title only | rich document blocks |
| Quote block | Missing | paragraph approximation | rich document blocks |
| Divider | Missing | none | presentation/decorative semantic decision |
| Stable paragraph/block IDs | Missing | document paragraphs are anonymous strings | document semantic identity |
| Current spoken sentence/range | Missing | none | representative document-range state |
| Soft sentence highlight | Missing | none | document projection state + visual role |
| Scroll-follow intent | Runtime boundary | host executes scrolling | representative follow policy may be authored as interaction intent later |
| EPUB parsing | Runtime boundary | outside ViewWright | not a ViewWright feature |

## Visual system

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Canvas/surface/raised/text/muted/accent/border | Native | M2 visual palette | good foundation |
| Dark restrained jade direction | Native | color tokens + visual bindings | basic palette expressible |
| Design character / avoidances | Native | `design.character`, `design.avoid` | concept projection already carries them |
| One global corner policy | Native | corner token + border policy | insufficient for varied component roles |
| Hover/active surface color | Missing | egui derives state | semantic interaction-state palette roles |
| Accent-soft/highlight color | Missing as bound role | token may exist but renderer cannot consume it semantically | richer visual roles |
| Reading text distinct from UI text | Missing | one text role | reading/document typography/palette |
| Text-faint/divider roles | Missing | no bound semantic role | richer palette vocabulary |
| UI sans vs book serif | Missing | only four numeric type sizes | font-family/semantic text role authorship |
| Line height / paragraph spacing | Missing | none | reading presentation |
| Column width / page margin | Missing | major geometry only | document presentation |
| Text alignment | Missing | none | document presentation |
| Shadow/elevation | Missing | none | only add if north-star pressure proves necessary |
| Theme presets | Missing | separate sources could mimic | named visual variants/presets not modeled |

## Fixture and representative state

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Collection/tree selection | Native | fixture selected ID | none |
| Command enabled/disabled | Native | fixture command state | none |
| Property values | Native | fixture properties | display-only |
| Search edit state | Approximate/runtime-local | egui RenderState | not canonical fixture state |
| Active tab | Missing | none | representative choice state |
| Slider/toggle/select values | Missing | property text approximation | typed control fixture state |
| TTS playing/paused visual state | Missing | command enabled != playback state | representative media state |
| Playback progress | Missing | none | typed progress state |
| Voice/rate/volume selection | Missing | property text approximation | typed control state |

Representative state should be authorable for isolated design previews without turning ViewWright into the application state machine.

## Responsive behavior

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Fixed wide layout | Native | one topology at one viewport | current strength |
| Explicit compact layout | Missing | backend compression only | responsive variants |
| Explicit narrow layout | Missing | backend compression only | responsive variants |
| Width/height breakpoints | Missing | none | deterministic conditions |
| Reorder/move regions by viewport | Missing | one topology | variant topology |
| Resize region dimensions by variant | Missing | one authored value | variant overrides |
| Hide/collapse optional controls | Missing | none | responsive presence semantics |
| Exact breakpoint tests | Missing | LayoutPlan takes viewport but no authored rules | responsive selector before LayoutPlan |
| Stable identity across variants | Missing contract | one topology only | v0.2 identity semantics |

This is a required v0.2 capability, not an indefinite someday item.

## Concept-art workflow

| Lantern Leaf intent | 0.1 status | Current mapping | Gap |
|---|---|---|---|
| Deterministic concept brief | Native | `viewwright-concept` | exists |
| Brief contains purpose/character/hierarchy/palette/type/elements | Native | current concept projection | exists |
| Brief contains rich local furnishing | Missing | model lacks it | follows furnishing work |
| Brief contains representative fixture state | Missing/limited | concept projection mostly ignores fixture state | richer concept dossier |
| Brief contains responsive variants | Missing | no responsive model | follows responsive work |
| Brief contains assets/media references | Missing | no asset model | richer dossier |
| Direct image-generation API | Intentionally absent | external agent/tool | not required for v0.2 core |
| Generated concept archived beside consumer docs | External workflow | consumer repo can archive | ViewWright should make reproducible dossier/export easy |
| Accepted visual ideas return to TOML | Policy exists | M2 concept boundary | must remain governing rule |

## Bottom line

ViewWright 0.1 can author the **architectural shell** of the Lantern Leaf concept.

It cannot yet author the **furnishing, responsive behavior, or rich reading-state semantics** with enough fidelity to claim the concept is represented.

The largest enabling gap is nested semantic furnishing and local layout inside major regions. The most important user-facing v0.2 deliverable is authored responsive behavior. The north-star convergence target requires both.
