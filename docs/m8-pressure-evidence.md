# M8 — Pressure Evidence

M8 is justified by already-accepted canonical source, not by a hypothetical future screen.

## Project Browser

`examples/project-browser.toml` authors two collection presentations in the same screen:

```toml
[[element]]
id = "navigation_items"
kind = "collection"
presentation = "list"
```

```toml
[[element]]
id = "project_collection"
kind = "collection"
presentation = "adaptive_cards"
```

The accepted `many_projects` fixture supplies three navigation entries and four project entries.

The current egui backend ignores `ResolvedElement.presentation` and routes every collection through the same vertical-label renderer. Therefore the two authored presentations become visually identical in kind.

That is the defect M8 exists to remove.

## Dependency Workbench regression pressure

`specimens/dependency-workbench.toml` explicitly authors both collection elements as `presentation = "list"`.

M8 must not solve Project Browser by globally converting collections into cards. Dependency Workbench should remain compact and linear.

## Why the existing specimens are sufficient

The repository already contains all required acceptance states:

- `project_browser / many_projects` — list and adaptive cards visible together;
- `project_browser / selected_project` — selected single-card case;
- `project_browser / empty` — empty card collection;
- `dependency_workbench / healthy` — list regression;
- `dependency_workbench / advisory` — list regression under alternate fixture data.

No new pressure specimen or source syntax is required for M8.
