# M40 — Lantern Leaf v0.1-language baseline findings

Status: implementation evidence for `lantern_leaf_reader_baseline`; human/Director visual review is pending. This is an intentionally under-furnished approximation, not a claim of Lantern Leaf parity.

Source pressure inspected: Lantern Leaf's archived EPUB + TTS reader README, semantic furnishing TOML, and 400×300 archived concept preview. The furnishing remains external and was not copied into ViewWright.

## Executed baseline

Canonical ViewWright 0.1 source: [`specimens/lantern-leaf-reader-baseline.toml`](../../specimens/lantern-leaf-reader-baseline.toml), fixture `reading` (`chapter_three_preview`). It uses existing text, status, collection/list, property-sheet, tree, document, command, fixture, visual-profile, and composition semantics.

The wide shell is represented by a vertical `workspace` composition containing a growing horizontal `reading_body` and a fixed-height `tts_player` region. `reading_body` splits into a 260 px `library`, a growing `reader_column`, and a 330 px `inspector`. `reader_column` stacks a 52 px command/search toolbar above a growing document region. The player region is 96 px high. These are ordinary 0.1 region/composition dimensions; the center's dominance follows its grow allocation.

The authored palette maps the archived near-black green application background, dark reading surface, raised surface, warm text, muted metadata, jade accent, and quiet border onto the existing seven visual roles. The type scale approximates the hierarchy numerically; it does not express the concept's Inter/Literata family split.

## Surface-by-surface classification

| Lantern Leaf surface or intent | Baseline treatment | Classification | What remains absent or distorted |
|---|---|---|---|
| Wide library / reader / inspector shell | Nested vertical and horizontal compositions; 260 px library, growing reader, 330 px inspector | Direct mapping for topology; approximate dimensions | Only the wide arrangement is authored. No compact/narrow state exists. |
| Persistent bottom player | 96 px `commands` region at the bottom of the root column | Direct mapping for persistence and height | Media summary, transport, and auxiliary controls share one flat region. Their local groups and between/center alignment are flattened. |
| Library brand | `Text` label plus a fixture-backed tagline status | Approximation | No leaf icon, cover art, or authored text-style role. |
| Primary navigation | Selected `collection` with list presentation for Library, Search, Highlights, Bookmarks, and Notes | Direct mapping for list and selected item | Icons and active-nav control semantics are absent; selection is only representative collection state. |
| Current-book summary | Property sheet for title, author, chapter count, and progress | Approximation / flattened furnishing | No media-summary semantic object, image/cover reference, or typed chapter/progress state. |
| Table of contents | Tree with eight chapter nodes and chapter 3 selected | Direct mapping for hierarchy and selection | Chapter identity is just fixture tree content; it is not bound to the document or playback. |
| Reader toolbar | Search, previous/next/bookmark commands, and chapter status in a 52 px commands region | Fake-region inflation / flattened furnishing | `reader_toolbar` is promoted to a first-class region solely to position this local toolbar above the document. Toolbar clusters and left/right grouping cannot be represented. Search has only renderer-local edit state; no authored query state. |
| Chapter/document | Document title and four ordered paragraphs in the growing reading region | Approximation | Eyebrow, heading level, divider, and quote block are flattened into the title and paragraph strings. Paragraphs have no stable authored block IDs. |
| Spoken-sentence highlight and follow | Not represented | Missing representative state and missing document/visual semantics | There is no current sentence/range, soft-fill role, playback-to-document relation, or auto-follow state. The fixture does not claim a playing state or create playback behavior. |
| Inspector settings | Three labeled property sheets for layout/typography, theme/annotations, and speech settings | Approximation / flattened furnishing | Group headings are neighboring elements, not groups. Values are display-only text, not controls or editable state. |
| Tabs, selects, sliders, toggles, segmented choices, color picker, preset grid | Their current example values are summarized as property-sheet rows | Missing control semantics | No choice, boolean, scalar, color-picker, tab, or preset interaction/state exists in 0.1. |
| Playback, voice, speed, volume, waveform/progress | Labeled commands and status text | Approximation / missing control semantics | Commands emit action events only; they do not perform application work. Voice/rate are labels, volume is text, and progress is not seekable or typed. No waveform or media-summary semantics exist. |
| TTS settings groups and control clusters | Flattened into three property-sheet elements and a single horizontal-wrapped command region | Flattened furnishing; no additional fake-region inflation | No nested semantic groups, local row/column layout, cluster gap, or alignment/justification. |
| UI vs reading typography | Numeric global display/heading/body/caption scale and warm text palette | Missing visual semantics / approximation | No UI sans vs book serif family, line-height, paragraph spacing, reading color role, column width, or page-margin presentation semantics. The values appear as static properties only. |
| Icons and book imagery | Omitted | Missing visual semantics | No asset/icon reference or image-summary semantic exists. Text labels are not substitutes for those semantics. |
| Dark restrained jade visual direction | Existing canvas/surface/raised/text/muted/accent/border roles and minimal border policy | Approximation through existing visual semantics | No separate hover/active dark role, soft jade highlight role, faint/divider role, or per-reading-surface typography/color role. |
| Wide / compact / narrow behavior | One fixed wide shell | Missing responsive behavior | No breakpoints, alternate topology, reordering, collapse/hide behavior, or exact responsive-state tests. Narrow compression is accidental, not authored. |

## Projection evidence and concept-specification loss

The same specimen is used for parse/validate/resolve, semantic/debug output, ASCII, concept specification, LayoutPlan, preview selection, expectation export, and AccessKit/Witness comparison. The focused baseline projection/layout integration test passes. At 1440×900, LayoutPlan reports the authored 260 px library, 330 px inspector, 96 px player, and a growing reading region. The M33 expectation and real AccessKit-enabled egui `FullOutput` convert through the pinned ViewWitness adapter to a valid witness (zero validation issues); M35 reports an exact match with zero mismatches and zero evidence gaps. The M38 comparison logic and its contracts are unchanged.

The current concept specification retains screen identity/purpose, design character and avoidances, dominant identity, nested composition axes, region IDs/roles/surfaces/importance/overflow, semantic element kinds/labels/importance, the seven resolved palette roles, numeric type scale, corner radius, and border policy.

It does **not** carry fixture values: selected Library/chapter, book/author, TOC labels, document text, settings values, chapter/playback status, or command fixture state disappear. It also has no local grouping, settings/control semantics, document block taxonomy or spoken range, asset references, separate UI/book font intent, or responsive variants to project. The baseline does not expand the concept projection to conceal these losses.

The preview executable was launched with the baseline in its specimen selector and then closed. The available computer-use surface exposed no native application windows, so this run did not provide a direct visual inspection of the eframe specimen. Human/Director visual QA remains pending; no visual-parity claim is made.

## Findings

The outer wide shell and major region sizing are expressible in accepted 0.1 language. The reader toolbar demonstrates fake-region inflation; settings and player furnishing otherwise flatten into region element order. Many pictured settings and playback affordances can only be explanatory strings or action labels. The current concept output further drops all fixture state. The specimen therefore exposes a recognizable architecture with substantial, explicit semantic and visual gaps; it is not a simplified substitute for the north star.

No conclusion about which v0.2 schema should follow is pre-decided here. Human/Director visual QA remains pending.
