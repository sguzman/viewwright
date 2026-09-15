# M27 — Canonical Audit

Accepted Reader document fixtures were reviewed for authored document titles.

Loaded/selected/settings Reader states use the nonblank title `The Quiet Machine`. The authored empty Reader state uses the explicit nonblank title `No document loaded` with `paragraphs = []`.

No accepted canonical document title is empty or whitespace-only. Empty paragraph lists are already canonical and must remain legal.

No canonical source migration, renderer change, layout change, projection change, or visible-output change is expected for M27. Valid authored titles remain byte-for-byte unchanged.

If implementation discovers a contradictory accepted source, stop and report it rather than silently editing canonical fixtures.
