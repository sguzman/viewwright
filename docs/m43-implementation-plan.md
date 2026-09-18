# M43 — Implementation Plan

## 1. Model evolution

Refactor DocumentSource into a backward-compatible shape supporting:

- legacy title + paragraphs;
- rich blocks + optional spoken range.

Resolve to an explicit enum.

Add typed block kind and spoken-range types.

Validate document form exclusivity, IDs, block fields, and range offsets.

Use Unicode scalar-value indexing for ranges.

Add exhaustive negative tests.

## 2. Preserve legacy renderer

Keep legacy document rendering byte-for-byte/behaviorally equivalent where practical.

Do not alter existing spacing, label behavior, or paragraph flow for old documents.

Add explicit regressions around a current legacy specimen.

## 3. Rich renderer

For rich documents:

- do not render the generic element label before the block sequence;
- eyebrow uses restrained existing caption/muted treatment;
- heading uses existing heading scale;
- paragraph uses body text;
- quote is visibly distinct using existing typography/layout primitives;
- divider uses existing border color/policy where visual config exists.

Do not add source visual fields.

## 4. Spoken range

Split one textual block into:

- prefix;
- highlighted range;
- suffix.

Index by Unicode scalar values.

Render through egui text layout preserving exact textual content.

Derive a soft dark accent-tinted fill from existing palette.

Add tests for:

- ASCII text;
- Unicode text;
- range at beginning/middle/end;
- exact prefix/range/suffix preservation;
- readable highlight contrast;
- no range on divider.

## 5. Projections

Concept fixture-aware output should include block list and spoken range.

Add fixture-aware ASCII output (while preserving existing structural `render`) or an equivalent deterministic debug projection with:

- block ID;
- kind;
- heading level;
- representative spoken range.

## 6. Lantern Leaf specimen

Add:

`specimens/lantern-leaf-reader-document.toml`

Start from M42 controls specimen without modifying it.

Replace only the chapter document fixture content with rich blocks.

Required structure:

- CHAPTER 3 eyebrow;
- The Mountain Path H1;
- divider;
- p1/p2/p3;
- closing quote;
- spoken range in p2.

Keep controls and furnishing the same unless a tiny fixture-only copy adjustment is required.

## 7. Identity / verification

Assert rich block IDs are absent from AccessKit author IDs.

Document element remains owned immediately by reader region.

Build expectation 0.2 and pass the real ViewWitness/M35 chain.

Keep all prior exact cases green.

## 8. Human QA

Compare M42 controls specimen with M43 document specimen.

Check richer chapter semantics, soft spoken range, reader dominance, existing inspector/TTS behavior, and scroll/clipping.

## 9. Checks

Run:

```text
cargo fmt --all
cargo fmt --all --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo test -p viewwright-egui --test viewwitness_compat
cargo test -p viewwright-egui --test exact_verification -- --nocapture
git diff --check
```

## 10. Publish

README -> M43 implementation complete; human/Director review pending.

Comment issue with implementation evidence.

Leave issue open.

Commit/push/sync/clean.

Do not start M44.
