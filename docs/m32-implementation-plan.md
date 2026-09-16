# M32 — Implementation Plan

1. **Model**
   - add optional source `region.overflow`;
   - add typed resolved overflow policy;
   - default omission to `clip`;
   - reject unknown values.

2. **Projection**
   - preserve existing output for default `clip` regions where practical;
   - annotate non-default `scroll_y` in semantic/debug, ASCII, and concept inspection;
   - do not invent geometry or scrollbar styling.

3. **Layout**
   - do not change LayoutPlan allocation;
   - region/composition rectangles remain exactly determined by existing M4/M21/M31 rules.

4. **egui**
   - keep region surface/frame painting fixed;
   - apply the existing density region inset to produce the content viewport;
   - for `clip`, explicitly confine paint/interaction to that content viewport;
   - for `scroll_y`, place ordinary region element rendering inside a vertical scroll area bounded by that content viewport;
   - use stable backend-local identity so scroll interaction survives redraws without adding canonical runtime state.

5. **Pressure specimen / preview**
   - register `reader_overflow_pressure`;
   - exact fixture `long_document`;
   - do not modify accepted Reader specimens.

6. **Regression coverage**
   - model default/valid/invalid overflow cases;
   - resolved typed policy;
   - LayoutPlan equality between otherwise-identical clip/scroll sources;
   - projection annotation for `scroll_y` and no fake responsive semantics;
   - backend smoke/regression coverage for clipping/scroll area construction where practical;
   - canonical-source compatibility.

7. **QA / publication**
   - run formatting/tests/check/diff checks;
   - human QA the exact pressure selector;
   - verify mouse-wheel scrolling moves only Reader inner content while surrounding regions stay fixed;
   - close any process Codex launches;
   - publish implementation, comment the issue, leave it open for Director + human acceptance;
   - do not begin M33.
