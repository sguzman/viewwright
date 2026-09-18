# Lantern Leaf as the ViewWright v0.2 north star

## Decision

Lantern Leaf's archived EPUB + TTS reader concept is the explicit design north star for ViewWright v0.2.

ViewWright 0.1.0 is a completed foundational release. v0.2 begins a new pressure-driven phase whose question is:

> Can ViewWright author the durable design intent of a sophisticated application concept like Lantern Leaf without collapsing into CSS, MUI props, egui calls, or application runtime logic?

The answer today is: **partially**.

That partial fit is exactly why the concept is useful.

## Reference artifacts

The north-star evidence lives in the Lantern Leaf repository:

- `docs/concept-art/epub-tts-reader/README.md`
- `docs/concept-art/epub-tts-reader/furnishing.toml`
- `docs/concept-art/epub-tts-reader/leafline-epub-tts-reader-concept.webp`

The furnishing is explicitly a semantic TOML-like design experiment, not Lantern Leaf's production schema. ViewWright must not copy it mechanically.

## Durable north-star intent

The v0.2 target should preserve at least these product-level ideas:

- reading and listening are one coherent activity;
- the book/document remains the visually dominant surface;
- library/navigation and customization remain supporting surfaces;
- TTS transport is persistent and immediately available;
- the currently spoken sentence is visibly projected into the document;
- customization is powerful, inspectable, and grouped rather than hidden;
- navigation remains book-shaped rather than becoming a generic text-buffer shell;
- operational complexity lives around a calmer reading surface;
- visual language is dark, restrained, warm, and selectively accented;
- resize behavior is deliberate authored intent, not accidental compression.

## Spatial north star

The archived desktop concept establishes a strong wide layout:

- left library/navigation rail: roughly 260 logical px;
- center reader: growing dominant surface;
- right inspector: roughly 330 logical px;
- persistent bottom TTS surface: roughly 96 logical px.

Current ViewWright can already express this outer shell with nested row/column compositions, fixed dimensions, grow allocation, surface roles, and vertical overflow.

The gaps appear inside those major surfaces.

## v0.2 fidelity target

ViewWright v0.2 should be able to author one canonical north-star design that captures:

1. **Major shell topology**
   - library;
   - reader;
   - inspector;
   - persistent TTS transport.

2. **Nested semantic furnishing**
   - toolbars;
   - clusters;
   - settings groups;
   - transport groups;
   - local row/column/grid-like relationships where earned.

3. **Representative control semantics**
   - commands;
   - search;
   - choices;
   - toggles;
   - scalar/range adjustments;
   - tabs/segmented choices as presentations of semantic choice where appropriate;
   - progress/seek surfaces.

4. **Reader/document semantics**
   - chapter/heading/paragraph/quote structure;
   - presentation intent distinct from application chrome;
   - authored current-spoken-range/highlight semantics as representative preview state, without implementing TTS execution.

5. **Visual authorship**
   - sufficient palette roles for the concept;
   - UI-vs-reading typography distinction;
   - local presentation roles where semantic pressure justifies them;
   - assets/icons only at an authoring level, not backend implementation detail.

6. **Responsive authoring**
   - explicit `wide`, `compact`, and `narrow` design states;
   - deterministic width/height conditions;
   - deliberate region sizing, reordering, movement, collapse/hide semantics where explicitly authored;
   - exact breakpoint behavior testable independently of egui;
   - no CSS cascade or device sniffing.

7. **Concept-art projection**
   - a deterministic concept dossier rich enough for an external image-generation step;
   - generated imagery remains exploratory, never canonical;
   - accepted visual discoveries return to canonical ViewWright source.

8. **Verification**
   - semantic identity remains stable where the same object survives across responsive states;
   - expectations can be built for selected responsive states/viewports;
   - real renderer output can still be compared through ViewWitness.

## What v0.2 does not need to become

The north star does not authorize ViewWright to become:

- Lantern Leaf's application runtime;
- a TTS engine;
- a data-binding language;
- React, MUI, Tailwind, CSS, or egui serialization;
- an EPUB parser;
- a general animation system;
- a Figma clone;
- a full image-generation service;
- a browser DOM model.

The goal is design authorship and faithful projection.

## Relationship to Lantern Leaf

Lantern Leaf is a consumer/product pressure source, not a dependency.

ViewWright should eventually be able to produce a design artifact from which Lantern Leaf can implement or audit its UI, but v0.2 does not require a React/Tauri backend.

A future backend may be earned separately.

## North-star rule

When a proposed ViewWright feature has no clear relationship to the Lantern Leaf north star or another concrete pressure specimen, it should not be added merely because it is imaginable.
