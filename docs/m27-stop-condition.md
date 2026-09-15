# M27 — Stop Condition

Stop M27 once:

- empty and whitespace-only document titles are rejected;
- valid titles are preserved exactly;
- missing titles remain parse failures;
- paragraph content and empty paragraph lists remain legal and unchanged;
- canonical Reader fixtures resolve unchanged;
- no renderer/layout/projection changes are required;
- all accepted regressions remain green.

Do not continue into paragraph policy, rich-text/document-engine work, or M28.
