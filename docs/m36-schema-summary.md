# M36 — Schema Summary

M36 adds no TOML fields and no new resolved-model fields.

It strengthens validation of the existing authored identity fields.

## Observable semantic identity namespace

The namespace projected by M34 into AccessKit `author_id` is:

- `screen.id`;
- every `region.id`;
- every `element.id`.

M36 requires exact-string uniqueness of the screen ID against the region and element ID sets.

Existing region/element uniqueness remains as already implemented.

## Validation behavior

A source is invalid when:

```text
screen.id == any region.id
```

or:

```text
screen.id == any element.id
```

This validation applies regardless of reachability.

A compact implementation may maintain dedicated region/element ID sets and perform explicit screen-collision checks.

Do not broaden this by putting the screen ID into an existing structural-ID set if doing so would also reject screen/composition or screen/fixture equality.

## Unchanged serialized/projection contracts

No changes to:

- M33 `ViewWrightExpectation` schema/version;
- M34 AccessKit `author_id` values;
- M35 `ComparisonReport` / matching semantics;
- ViewWitness `Witness`.
