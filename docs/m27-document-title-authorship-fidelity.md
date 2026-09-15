# M27 — Document Title Authorship Fidelity

## Problem

M5 defines document fixture content as a required `title` plus ordered plain-text `paragraphs`. The resolved model and egui renderer preserve and display the authored title directly. Today a present-but-empty or whitespace-only title can survive resolution, producing a document fixture whose required visible heading carries no authored content.

## Principle

**A required authored document title must contain at least one non-whitespace character.**

Use trim-based blankness detection only for validity. Preserve every valid title exactly as authored.

## Scope

M27 applies only to `DocumentSource.title`.

- empty title: invalid
- whitespace-only title: invalid
- valid title with surrounding whitespace: valid and preserved exactly
- missing title remains a TOML deserialization failure
- paragraph strings remain unconstrained
- empty paragraph strings remain legal
- empty paragraph lists remain legal

## Boundary

M27 does not add title normalization, title grammar, localization, rich text, document IDs, paragraph validation, paragraph normalization, document schemas, renderer/layout changes, or M28 work.
