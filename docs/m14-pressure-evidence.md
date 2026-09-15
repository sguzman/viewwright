# M14 — Pressure Evidence

M14 is driven by an accepted semantic rule rather than speculative feature breadth.

## Existing contradiction

M10 established:

> Internal identifiers and ontology terms must not become visible application text merely because the backend knows them.

It also established that `element.label` is authored visible text whose presentation is owned by each element kind.

Current resolution still does the equivalent of:

```text
label = authored_label.unwrap_or(element_id.replace('_', ' '))
```

Therefore an omitted authored label causes internal element identity to become visible product copy.

Example:

```text
project_search -> "project search"
reader_settings -> "reader settings"
workspace_status -> "workspace status"
```

Those are plausible-looking strings, which makes the semantic leak particularly easy to miss.

## Canonical pressure

Accepted canonical screens already author labels explicitly for their real elements:

- Project Browser;
- Reader Workspace / visual Reader;
- Dependency Workbench;
- Comfortable/Dense density pressure pair.

So removing the fallback does not require a canonical UI redesign.

## Why this is separate from M10

M10 corrected renderer-level invented chrome. M14 closes the remaining upstream source-to-resolved loophole where resolution itself invents visible text.

The resulting invariant is simple:

```text
visible element label
    <- authored element.label

never

visible element label
    <- internal element.id
```