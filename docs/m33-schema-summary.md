# M33 — Schema Summary

M33 introduces a separate machine-readable normative artifact conceptually shaped as:

```yaml
viewwright_expectation_version: "0.1"
epistemic: intended
screen:
  author_id: project_browser
viewport:
  width: 1440
  height: 900
dominant:
  kind: region
  author_id: projects
regions:
  - author_id: navigation
    role: navigation
    importance: secondary
    bounds: { x: 16, y: 16, width: 240, height: 868 }
    overflow: clip
elements:
  - author_id: project_search
    region_author_id: navigation
    kind: search
    importance: secondary
    label: Search projects
```

Exact Rust type names are implementation detail.

## Required semantic fields

Top level:
- version;
- epistemic = intended;
- screen author ID;
- logical viewport width/height;
- optional dominant target.

Region expectation:
- author_id;
- role;
- importance;
- bounds;
- overflow.

Element expectation:
- author_id;
- region_author_id;
- kind;
- importance;
- label;
- optional command action ID.

## Explicit absences

No element bounds, fixture payloads, mutable state, observed IDs, screenshots, tolerances, or ViewWitness witness envelope.

The exporter consumes resolved semantics and LayoutPlan; it does not add fields to authored TOML.
