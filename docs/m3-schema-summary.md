# M3 Schema Summary

The M3 pressure test proposes only one new authored area: nested fixture content.

```text
fixture
  id
  state
  content[]
    element
    one of:
      collection items + optional selected item
      ordered properties
      text
```

Representative source:

```toml
[[fixture]]
id = "healthy"
state = "ready"

[[fixture.content]]
element = "packages"
items = [
  { id = "serde", label = "serde 1.0.210" },
  { id = "toml", label = "toml 0.8.19" },
]
selected = "serde"

[[fixture.content]]
element = "package_details"
properties = [
  { name = "Version", value = "1.0.210" },
  { name = "Scope", value = "direct" },
]

[[fixture.content]]
element = "workspace_status"
text = "24 packages · 0 advisories"
```

The element reference determines semantic compatibility. The content record does not contain an egui widget type.

M3 deliberately does not add runtime bindings, arbitrary maps, scripts, actions, editable-form definitions, table schemas, or application-model references.