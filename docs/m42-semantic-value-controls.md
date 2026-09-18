# M42 — Semantic Value Controls and Representative State

## Pressure

M41 solved region-local structure. The furnished Lantern Leaf screen now has truthful places for controls, but many controls are still semantically fake:

- font family is display-only property text;
- font size, line height, paragraph spacing, margins, and column width are property rows rather than adjustable values;
- text alignment and reading flow are not authored as finite choices;
- annotation visibility cannot be authored as booleans;
- bottom Voice and Speed still masquerade as commands even though they represent selected values;
- volume is not a typed bounded value.

The external Lantern Leaf furnishing shows repeated instances of three semantic value families:

1. finite discrete choice;
2. true/false choice;
3. bounded numeric scalar.

M42 adds those families without copying toolkit widget vocabulary.

## Canonical semantic families

### choice

A finite authored option set with one representative selected value.

Choice may carry one of two presentation hints in M42:

- `select`;
- `segmented`.

These are presentation intent, not backend widget serialization.

M42 does not add tabs because tab selection also implies conditional content/view switching that ViewWright does not yet model truthfully.

### boolean

A true/false authored affordance.

The semantic kind is `boolean`, not `toggle` or `checkbox`.

The egui backend may realize it with a checkbox/toggle-like control.

### scalar

A finite bounded numeric adjustment:

- minimum;
- maximum;
- positive step;
- optional display unit.

The semantic kind is `scalar`, not `slider`.

The first egui backend realizes it with a slider.

## Host/runtime boundary

These elements describe interaction affordances and representative values, not application state ownership.

The renderer may maintain ephemeral preview-local values initialized from the active fixture.

When a value changes, the renderer reports:

- element ID;
- stable authored action ID;
- typed changed value.

The host may decide what the action means.

ViewWright does not:

- persist values;
- execute settings logic;
- bind to user preferences;
- alter document layout because font-size changed;
- synchronize duplicate product settings;
- implement TTS behavior.

## Interaction event evolution

Existing command events remain valid.

M42 extends interaction reporting with an optional typed value:

```text
none                command activation
choice("aria")      finite choice changed
boolean(true)       boolean changed
scalar(18.0)        numeric value changed
```

Exact Rust naming is implementation detail, but the type must not degrade values to display strings.

Search remains renderer-local and unchanged.

## Representative fixture state

Control value is fixture state.

Each fixture in a blueprint containing M42 controls must explicitly supply exactly one typed state record for each control element.

No hidden first-option/false/minimum fallback is allowed for canonical rendering.

This makes isolated preview state inspectable and deterministic.

Renderer-local interaction may temporarily diverge from the fixture seed until RenderState is cleared or the specimen/fixture changes.

## Action binding

`choice`, `boolean`, and `scalar` elements require stable namespaced `action` IDs, using the accepted command action-ID rules.

This is an affordance contract only.

## Semantic scope

M42 deliberately excludes:

- color choice;
- progress/seek;
- waveform;
- tabs/content switching;
- media summary;
- icon-only semantics;
- rich document blocks;
- responsive rules.

Those are not aliases for the three M42 value families merely because an egui widget could be improvised.
