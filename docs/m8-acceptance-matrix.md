# M8 — Acceptance Matrix

| Preview selector | Authored presentation | Expected M8 projection |
| --- | --- | --- |
| `project_browser / many_projects` — navigation | `list` | compact vertical list |
| `project_browser / many_projects` — projects | `adaptive_cards` | wrapping discrete cards |
| `project_browser / selected_project` — projects | `adaptive_cards` | one selected card |
| `project_browser / empty` — projects | `adaptive_cards` | no fabricated items |
| `dependency_workbench / healthy` — Scope | `list` | compact vertical list |
| `dependency_workbench / healthy` — Dependencies | `list` | compact vertical list |
| `dependency_workbench / advisory` — Scope | `list` | compact vertical list |
| `dependency_workbench / advisory` — Dependencies | `list` | compact vertical list |

The important acceptance property is semantic differentiation: the backend must not route both presentation values through one visually equivalent collection renderer.
