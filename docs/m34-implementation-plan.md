# M34 — Implementation Plan

1. **Accessibility identity helpers**
   - add the smallest egui-backend helper(s) needed to set AccessKit `author_id` on a stable egui node ID;
   - keep authored values exact.

2. **Screen anchor**
   - reuse or create a non-visual semantic container for the rendered ViewWright screen;
   - attach exact `screen.id` author identity.

3. **Region anchors**
   - ensure each rendered region has exactly one semantic accessibility anchor;
   - attach exact region ID;
   - associate the node with the already-planned region rectangle;
   - keep ordinary region content beneath it.

4. **Element anchors**
   - wrap/reuse one semantic accessibility container per rendered ViewWright element;
   - attach exact element ID;
   - render all existing native widgets beneath the same semantic anchor;
   - do not annotate fixture-local subcontent as canonical identity.

5. **Regression coverage**
   - enable AccessKit in focused egui tests and inspect `FullOutput.platform_output.accesskit_update`;
   - assert exact screen/region/element author IDs;
   - assert uniqueness/one anchor per semantic object;
   - assert unused declarations omitted;
   - assert region anchor bounds match LayoutPlan;
   - assert native child nodes remain present;
   - cover Project Browser, M31 overlay, M32 ScrollY, redraw, and fixture changes.

6. **Compatibility**
   - run existing renderer/model/layout tests;
   - prove no visible geometry/style/interaction behavior changed;
   - no ViewWitness dependency or repo mutation.

7. **Publication**
   - README: M33 accepted; M34 implementation complete / Director audit pending;
   - run full checks;
   - no screenshot QA unless implementation unexpectedly changes visible UI;
   - comment implementation issue and leave open;
   - do not begin M35.