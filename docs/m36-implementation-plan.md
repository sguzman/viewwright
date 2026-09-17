# M36 — Implementation Plan

## 1. Preserve the existing ID architecture

Do not refactor the whole resolver or introduce new ID types.

Use the existing resolved/source ID sets and add the narrow observable-namespace validation needed for `screen.id`.

## 2. Validate screen vs regions

After region IDs are known, compare every authored region ID against `screen.id` using exact string equality.

Any collision is an error, regardless of reachability.

## 3. Validate screen vs elements

After element IDs are known, compare every authored element ID against `screen.id` using exact string equality.

Any collision is an error, including elements assigned to unused regions.

## 4. Do not broaden to composition/fixture

Preserve the currently legal cases where `screen.id` equals:

- a composition ID;
- a fixture ID.

This means implementation should not simply insert the screen ID into the resolver's existing global structural `ids` set if that set also covers compositions/fixtures.

## 5. Diagnostics

Emit deterministic author-facing diagnostics that make the collision source obvious.

Conceptually:

```text
screen.id 'workspace' conflicts with region id 'workspace' in observable author-id namespace
```

and equivalent element wording.

Exact punctuation is not authority.

## 6. Tests

Add focused model tests for:

- reachable region collision;
- unused region collision;
- reachable element collision;
- element in unused region collision;
- screen/composition equality preserved;
- screen/fixture equality preserved;
- local collection/tree IDs remain separate if practical;
- exact case-sensitive identity behavior;
- canonical source compatibility.

## 7. Bridge regression

Do not alter M33/M34/M35 code unless compilation mechanically demands an import/test adjustment. Existing tests should remain green.

A compact integration regression may demonstrate that a formerly collision-valid source is now rejected before M34/M35 can see ambiguous identity, but do not duplicate the whole bridge stack merely for ceremony.

## 8. Bookkeeping

Update README:

- M35 — accepted;
- M36 — implementation complete; director audit pending.

Do not mark M36 accepted yourself and do not begin M37.
