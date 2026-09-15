# M21 — Canonical Audit

Director audit of accepted canonical sources found no region that currently combines a positive `grow` with a fixed size on that region's parent composition main axis.

## Reader Workspace / visual Reader / M7 Reader

- `app_commands` and `transport` use fixed vertical heights and no grow.
- `library` and `inspector` use fixed horizontal widths and no grow.
- `reader` uses horizontal grow and no fixed width.
- nested `reading_body` uses composition grow; compositions have no fixed-size field.

## Dependency Workbench

- command/status strips use fixed vertical height and no grow.
- filters/inspector use fixed horizontal width and no grow.
- packages uses horizontal grow and no fixed width.
- nested `body` uses composition grow.

## Project Browser

- navigation/inspector use fixed horizontal widths and no grow.
- projects uses horizontal grow and no fixed width.

## Density pressure pair

- controls uses a fixed horizontal width and no grow.
- content uses horizontal grow and no fixed width.

## Expected migration

None.

M21 should therefore change geometry only for previously valid sources that author same-axis fixed size plus positive grow. Accepted canonical layout rectangles should remain unchanged.