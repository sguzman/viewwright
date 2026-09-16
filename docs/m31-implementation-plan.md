# M31 — Implementation Plan

1. **Model topology**
   - add `CompositionKind::Overlay`;
   - change resolved composition axis representation so overlay is genuinely axisless while split/row/column retain typed axes;
   - keep source syntax unchanged.

2. **Validation**
   - preserve existing split/row/column validation;
   - keep `stack` rejected;
   - for overlay require exactly two children;
   - require child 0 composition and child 1 region;
   - reject authored axis/gap;
   - require floating-region fixed width and height;
   - reject positive floating-region grow under overlay;
   - preserve ownership, cycle, root, M29, and M30 validation.

3. **Layout**
   - branch on typed composition kind;
   - preserve existing linear allocator exactly for split/row/column;
   - for overlay compute common inner rect from padding;
   - assign base composition the full inner rect;
   - center floating region at authored fixed dimensions in the same inner rect;
   - keep both rectangles in existing LayoutPlan maps.

4. **Projections**
   - update semantic/debug output for axisless overlay;
   - update ASCII traversal so overlap/layer order is explicit;
   - update concept specification without inventing an axis.

5. **egui**
   - consume planned overlapping rectangles;
   - preserve base-first/floating-second authored order;
   - ensure overlapping interaction precedence belongs to floating UI;
   - do not implement modality or focus trapping.

6. **Pressure specimen / preview**
   - add `overlay-command-palette-pressure` to the preview selector;
   - fixture remains static and always open;
   - validate Search state and collection rendering through existing contracts.

7. **Regression coverage**
   - model constraint cases from the acceptance matrix;
   - layout geometry at deterministic viewport dimensions;
   - projection representation;
   - backend overlap interaction if practical in automated egui tests;
   - existing canonical sources and layout remain unchanged.

8. **QA and publication**
   - run formatting/tests/check/diff checks;
   - perform human visual QA because M31 intentionally adds a new visible topology;
   - close any process Codex launches before reporting completion unless the human is explicitly performing the live QA at that moment;
   - publish implementation but leave the issue open for director audit and human visual acceptance.
