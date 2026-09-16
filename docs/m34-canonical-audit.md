# M34 — Canonical Audit

Accepted ViewWright and ViewWitness sources were audited for the identity-bridge slice.

## ViewWitness capture

Current `sguzman/viewwitness/src/egui_capture.rs` constructs each canonical observed node with:

```text
NodeIdentity {
  provenance: "accesskit_node_id",
  stability: "structure_sensitive",
  author_id: node.author_id().map(...),
}
```

Therefore AccessKit `author_id` is already the canonical observation-side slot for explicit application-authored identity evidence. No ViewWitness model or format change is required for M34.

## ViewWright egui

Current `crates/viewwright-egui/src/lib.rs` renders:

- one root screen projection;
- root-reachable regions inside planned rectangles;
- semantic elements using ordinary egui widgets/scopes;
- M31 overlay children in authored layer order;
- M32 ScrollY content inside an egui ScrollArea.

The backend currently sets no AccessKit authored IDs.

## Existing identity authority

ViewWright already requires nonblank exact structural identifiers and preserves exact semantic IDs through resolution. M29/M30 ensure dominant and fixture targets belong to the root-reachable screen. M33 exports those exact IDs as normative `author_id` fields.

M34 does not need new source identity syntax.

## Canonical pressure

Project Browser provides sibling region + element pressure.

M31 overlay provides overlapping but structurally distinct `palette_surface` identity pressure.

M32 Reader overflow provides ScrollY hierarchy pressure.

No accepted source requires migration.

## API feasibility

egui exposes its AccessKit node builder during an enabled accessibility pass, and AccessKit nodes support `set_author_id`. Semantic container anchors are therefore an available backend projection mechanism rather than a new source/model concept.

## Migration result

No accepted TOML source, resolved model, LayoutPlan, expectation format, or ViewWitness repository change is authorized by M34.

If implementation requires any of those, stop and report the contradiction.