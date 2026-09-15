# M24 — Acceptance

M24 is accepted when:

- empty and whitespace-only names under `tokens.spacing`, `tokens.corners`, and `tokens.color` are rejected;
- valid token names are preserved exactly;
- valid unused tokens remain legal;
- existing exact token-reference semantics remain unchanged;
- color literal validation from M19 remains intact;
- accepted canonical sources require no migration and project identically;
- no new naming grammar or token framework is introduced.