# M41 — Pressure Evidence

M41 is directly earned by M40 implementation and human QA.

## Source-model evidence

ViewWright 0.1 has:

```text
composition -> region -> flat element list
```

There is no authored structural level below a region.

The renderer confirms that a region gathers all `element.region == region.id` elements and either:

- emits them sequentially; or
- for a commands region, emits the entire set through one `horizontal_wrapped` flow.

There is no local authored grouping or slot allocation.

## M40 authoring evidence

The Lantern Leaf baseline had to:

- invent `reader_toolbar` as a major region;
- put all TTS status and command elements directly in one commands region;
- flatten inspector control groups into property sheets.

The findings document classifies this explicitly as fake-region inflation and flattened furnishing.

## Human visual evidence

At 1440×900:

- the shell was recognizable;
- the reader remained dominant;
- chapter-position/status text was squeezed/clipped at the toolbar's right edge;
- the TTS `Voice` label collapsed into a narrow vertical character stack;
- otherwise the baseline behaved coherently.

These are allocation failures caused by absent local structure, not by missing application behavior.

## Verification constraint

M35 currently requires each expected element's immediate observed author-identity parent to be its expected region author ID.

M41 must not break this merely to introduce structural layout scopes.

Furnishings therefore follow composition precedent: authored structural objects without M34 author identity in this milestone.

## Why not controls first

Adding sliders/toggles/selects before local furnishing would create more semantic elements with nowhere truthful to live.

Structure is the dependency.

## Why not responsive first

Responsive variants need stable structures to vary.

Authoring alternate versions of today's flattened/fake-region arrangement would encode the wrong ontology multiple times.

Nested furnishing therefore precedes responsive authoring on the v0.2 path.
