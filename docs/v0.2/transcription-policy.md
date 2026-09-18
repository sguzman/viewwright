# Lantern Leaf transcription policy

The Lantern Leaf furnishing is pressure evidence, not ViewWright syntax.

## Do not copy the grammar

Lantern Leaf's `furnishing.toml` contains useful experimental concepts such as:

- `region.*.children`;
- `type = "slider"`;
- `type = "tabs"`;
- `type = "cluster"`;
- `layout = "row"`;
- `justify = "between"`;
- live TTS state;
- reader-style and theme tables.

These names are not automatically good ViewWright ontology.

Each proposed ViewWright extension must ask:

1. What authorial intent is this field preserving?
2. Is that intent general across applications?
3. Is it semantic, presentation, fixture state, or host runtime behavior?
4. Can it be expressed at a higher level than a toolkit widget?
5. Does it preserve backend independence?
6. How will it project into LayoutPlan, concept output, egui, expectation export, and ViewWitness identity?

## Example: control vocabulary

Do not mechanically add MUI-style widget kinds for every pictured control.

Prefer semantic families where possible.

For example, pressure may lead toward concepts such as:

- boolean choice;
- scalar adjustment;
- discrete choice with multiple presentations;
- progress/seek value;
- grouped command cluster.

Whether the canonical names become `toggle`, `range`, `choice`, or something else must be earned by the actual north-star specimen and other screens.

## Example: grouping

The concept has toolbars, clusters, settings groups, and transport groups.

The gap is not "ViewWright needs a div."

The gap is that semantic elements currently form a flat list inside a major region, while real interfaces contain meaningful local grouping and local layout that should not require inventing fake major regions.

The solution should preserve that distinction.

## Runtime boundary

The furnishing includes TTS engine configuration and live playback state.

ViewWright may need representative fixture state such as:

- playing vs paused;
- current choice;
- scalar value;
- progress;
- highlighted document range.

ViewWright should not implement:

- audio playback;
- pronunciation processing;
- EPUB parsing;
- persistence;
- application commands themselves.

## Round-trip rule

When a concept-art iteration produces a useful design decision, translate the decision back into ViewWright source.

Never let the image become a hidden second source of truth.
