# M23 — Local Fixture Label Authorship Fidelity

## Problem

M3 collection items and M5 tree nodes both pair stable local identity with authored visible labels. M17 already makes their IDs nonblank. M14 separately established that stable identity and visible copy are different semantics.

Current fixture resolution still accepts empty or whitespace-only collection-item and tree-node labels. Because egui renders those labels directly, a successfully resolved local object can exist structurally while carrying no authored visible copy.

## Principle

Required local labels must contain at least one non-whitespace character. Validation may inspect `label.trim()` for blankness only. Valid authored labels are preserved exactly.

## Scope

M23 applies only to `CollectionItemSource.label` and `TreeNodeSource.label`.

Missing labels remain TOML deserialization failures because both fields stay required `String`s. IDs remain authoritative for collection selection, tree selection, and tree parent references. Labels are not identity, so duplicate label text remains legal.

## Compatibility

Accepted canonical fixtures already use nonblank collection-item and tree-node labels. No source migration or valid-source visible delta is expected.

## Boundary

M23 adds no label uniqueness, ID-derived fallback, normalization, localization, validation of property/document/status text, command-reason policy, renderer/layout change, or broader copy-validation system.