# M39 — Acceptance Matrix

| Surface | Required evidence |
|---|---|
| README status | M0–M38 accepted; M39 implementation pending/audit state accurate |
| Bridge wording | current docs describe implemented expectation/Witness comparison |
| Architecture | no obsolete "future bridge" framing in current architecture |
| Charter | normative/descriptive boundary preserved; implemented bridge recognized |
| Historical model | M0-era `model.md` explicitly historical/non-authoritative |
| Changelog | root `CHANGELOG.md` with 0.1.0 entry |
| Release note | `docs/releases/v0.1.0.md` with scope + deferrals + verification |
| License | root MIT LICENSE consistent with manifest |
| Format | `cargo fmt --all --check` |
| Tests | `cargo test --workspace` |
| Compile | `cargo check --workspace --all-targets` |
| Lints | `cargo clippy --workspace --all-targets -- -D warnings` |
| Rustdoc | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` |
| M37 regression | ViewWitness compatibility test passes |
| M38 regression | real exact-verification test passes |
| Metadata | workspace packages remain 0.1.0 + MIT as applicable |
| Dependency boundary | ViewWitness egui test-only; compare model-only |
| TODO audit | no unresolved release-blocking `todo!()` / `unimplemented!()` / TODO/FIXME |
| Responsive roadmap | Issue #33 remains open and unpromoted |
| Release tag | absent until Director accepts M39 |
| Scope | no responsive/new feature/M40 work |
