# M11 — Pressure Evidence

M11 is driven by accepted canonical Search elements, not speculative schema design.

## Canonical Search elements

### Project Browser

- element id: `project_search`
- label: `Search projects`

### Visual Reader

- element id: `library_search`
- label: `Search library`

### Dependency Workbench

- element id: `package_search`
- label: `Search dependencies`

### M9 density pressure pair

- element id: `search`
- label: `Search`

## Current renderer defect

The egui Search branch currently creates:

```text
let mut query = String::new();
text_edit_singleline(&mut query)
```

inside the frame render path.

That means the user-facing editable value has frame lifetime. Any typed text is discarded on the next redraw.

## Why this is an earned gap

The existing Search element already projects to an interactive egui text editor. The backend therefore claims editability.

A text editor that cannot retain its text through redraws is not merely missing application behavior; it is a broken projection of the authored affordance.

M11 fixes that projection truthfulness only.
