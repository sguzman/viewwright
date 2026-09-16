# M28 — Canonical Audit

Accepted canonical status fixture payloads were reviewed for authored `text` content.

Dependency Workbench uses nonblank status strings such as `24 packages · 8 direct · 0 advisories` and `24 packages · 1 advisory requires review`.

Reader Workspace states use nonblank status strings such as `Ready to read · 12 min remaining`, `Chapter selected · 12 min remaining`, `Settings open · 12 min remaining`, and `Open a document to begin reading`.

No accepted canonical status `text` payload is empty or whitespace-only. Fixtures may omit status content records where no representative status content is authored.

No canonical source migration, renderer change, layout change, projection change, or visible-output change is expected for M28. Valid authored status text remains byte-for-byte unchanged.

If implementation discovers a contradictory accepted source, stop and report it rather than silently editing canonical fixtures.