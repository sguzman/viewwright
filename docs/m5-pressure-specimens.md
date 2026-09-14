# M5 — Pressure Specimens

M5 uses separate aspirational specimens so accepted `main` behavior remains parseable before the new fixture schema is implemented.

- `specimens/reader-workspace-m5.toml`
- `specimens/reader-workspace-visual-m5.toml`

These files intentionally use the proposed `nodes` and `document` fixture payloads before the current parser/resolver supports them.

They are architectural pressure inputs, not yet accepted runtime specimens.

Once M5 implementation is complete and accepted, Codex should migrate the canonical Reader specimens to the supported fixture-content form and either retire or clearly reclassify the aspirational M5 copies.

The accepted M4 Reader files must remain valid until that migration occurs.