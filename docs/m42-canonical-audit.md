# M42 — Canonical Audit

## Frozen north-star evidence

Do not modify:

- `specimens/lantern-leaf-reader-baseline.toml`;
- `specimens/lantern-leaf-reader-furnished.toml`;
- `v0.1.0`.

M40 and M41 remain before-states.

## New M42 source

Add a separate Lantern Leaf controls specimen.

It is canonical as M42 pressure evidence, not final Lantern Leaf parity.

## Expectation evolution

Expectation 0.2 is a deliberate canonical projection change.

Do not rewrite historical M33 authority/goldens as though 0.2 existed then; update current exporter tests and new current-facing docs only.

## Existing sources

Existing ViewWright TOML without M42 controls requires no migration.

No existing fixture must acquire control state unless its blueprint actually declares M42 controls.
