# M12 — Implementation Boundary

M12 is a typing/validation milestone for existing region-role semantics.

## In scope

- typed `RegionRole` in resolved model;
- validation from authored strings;
- useful diagnostics for unsupported roles;
- downstream use of typed role in semantic/debug, concept, and egui;
- removal of raw string branching such as `r.role == "commands"` after resolution;
- regression coverage for all canonical roles.

## Out of scope

- new role values;
- role aliases;
- role inheritance;
- role-driven styling system;
- role-driven responsive layout;
- new visible region chrome;
- region titles;
- toolbar/menu semantics;
- navigation behavior;
- collection selection events;
- ViewWitness integration;
- M13 work.

## Preservation requirements

M4 geometry, M6 visual semantics, M7 commands, M8 collection presentation, M9 density, M10 chrome fidelity, and M11 Search state remain authoritative.
