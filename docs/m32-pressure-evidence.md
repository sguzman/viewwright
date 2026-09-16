# M32 — Pressure Evidence

The pressure source is `specimens/reader-overflow-pressure.toml`.

It reuses the accepted Reader geometry and visual language but deliberately supplies a long plain-text document that exceeds the available Reader region height at the standard 1440 × 900 preview window.

The `reader` region authors:

```toml
overflow = "scroll_y"
```

This pressure is intentionally narrow:

- app commands remain fixed above the reading body;
- library remains a fixed left region;
- inspector remains a fixed right region;
- transport remains fixed below the reading body;
- Reader geometry is still produced by the existing composition/LayoutPlan system;
- only the Reader region's inner content needs vertical movement.

Without M32, long document content depends on whatever the backend happens to do after it exceeds the region's planned rectangle. That is not an authored UI contract.

The pressure does not require pagination, horizontal scrolling, virtualization, rich text, EPUB loading, TTS synchronization, responsive topology, or persisted application scroll position.

M5 explicitly named scrolling architecture as a deferred non-goal. The long-document specimen supplies the concrete pressure needed to reopen only the smallest useful part of that deferred area.
