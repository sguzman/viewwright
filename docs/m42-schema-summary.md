# M42 — Schema Summary

## New element kinds

M42 adds:

```toml
kind = "choice"
kind = "boolean"
kind = "scalar"
```

All retain existing required fields:

- `id`;
- `region`;
- `importance`;
- `label`.

All three additionally require a valid namespaced `action`.

## Choice configuration

```toml
[[element]]
id = "font_family"
region = "inspector"
kind = "choice"
importance = "secondary"
label = "Font Family"
action = "reader.font_family.set"
choice = { presentation = "select", options = [
  { id = "literata", label = "Literata" },
  { id = "source_serif", label = "Source Serif" },
  { id = "georgia", label = "Georgia" },
] }
```

Choice configuration is required for choice elements and invalid elsewhere.

Fields:

- `presentation = "select" | "segmented"`;
- `options` with at least two entries;
- option `id` must be nonblank and unique within the element;
- option `label` must be nonblank.

Option IDs are local to that choice and are not M34 author IDs.

## Boolean configuration

Boolean needs no extra element configuration beyond kind/label/action.

## Scalar configuration

```toml
[[element]]
id = "font_size"
region = "inspector"
kind = "scalar"
importance = "secondary"
label = "Font Size"
action = "reader.font_size.set"
scalar = { min = 12.0, max = 32.0, step = 1.0, unit = "px" }
```

Scalar configuration is required for scalar elements and invalid elsewhere.

Required:

- finite `min`;
- finite `max`;
- `min < max`;
- finite positive `step`.

Optional:

- nonblank `unit`.

M42 does not require a representative fixture value to fall exactly on a floating-point step lattice; it must be finite and within the inclusive range.

## Fixture state

```toml
[[fixture.content]]
element = "font_family"
choice = { selected = "literata" }

[[fixture.content]]
element = "font_size"
scalar = { value = 18.0 }

[[fixture.content]]
element = "show_highlights"
boolean = { value = true }
```

Rules:

- choice state only on choice elements;
- selected choice ID must exist in authored options;
- scalar state only on scalar elements;
- scalar value finite and within inclusive range;
- boolean state only on boolean elements;
- every fixture must provide exactly one corresponding state record for every M42 control element;
- duplicate state records for one control in one fixture are invalid.

## Resolved model

Add typed resolved control metadata:

- choice presentation;
- resolved choice options;
- scalar range/step/unit.

Add typed resolved fixture state for:

- selected choice;
- boolean value;
- scalar value.

Do not encode representative values as strings.

## Renderer state

RenderState gains fixture-scoped ephemeral values for M42 controls.

Switching specimen/fixture and `RenderState::clear()` resets to fixture seeds.

## Interaction event

Existing interaction event gains an optional typed value.

Command activation emits no value.

M42 controls emit their changed typed value.

## Expectation format

M42 advances canonical `viewwright_expectation_version` from `0.1` to `0.2`.

Reason: the normative exported `ElementKind` vocabulary now includes:

- `choice`;
- `boolean`;
- `scalar`.

The typed expectation model may retain the historical V0_1 variant for provenance/tests, but canonical `build_expectation` output after M42 is V0_2.

No new expectation fields are required.

M35 comparison semantics remain unchanged because M35 still compares identity, viewport, region bounds, and element ownership rather than element kind/value.

ViewWitness remains 0.1 and unchanged.
