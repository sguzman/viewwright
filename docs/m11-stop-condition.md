# M11 — Stop Condition

M11 stops when Search controls are truthful frame-to-frame editable previews.

Stop when all of the following are true:

- typed Search text survives redraws for the currently selected specimen/fixture;
- Search state is backend-local and explicit rather than canonical blueprint data;
- different Search element ids do not alias one value;
- switching specimen/fixture does not leak stale query text;
- no filtering, query execution, search event contract, or persistence layer has been added;
- M0–M10 regressions remain passing;
- human QA can type a multi-character value into a canonical Search field and see it remain visible.

Do not continue into:

- real search/filter semantics;
- search result ranking;
- collection/tree filtering;
- selection coupling;
- SearchChanged application events;
- text-input ontology beyond the existing Search affordance;
- generic form state;
- application state management;
- persistence;
- M12 work.
