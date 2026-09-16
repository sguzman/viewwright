# M33 — Implementation Plan

1. **Expectation model**
   - add a dedicated ViewWright-owned expectation model;
   - include version, epistemic=`intended`, screen author ID, viewport, optional dominant target, regions, and elements;
   - keep types serializable and deterministic.

2. **Projection**
   - consume `ResolvedBlueprint` plus explicit logical viewport dimensions;
   - call the existing backend-independent layout planner;
   - traverse only the root-reachable screen;
   - emit reachable regions in deterministic structural order;
   - emit reachable elements deterministically, preserving authored semantic identity.

3. **Geometry**
   - copy region bounds exactly from LayoutPlan;
   - do not recalculate geometry independently;
   - do not emit element bounds.

4. **Serialization**
   - provide deterministic YAML;
   - include a stable expectation format version;
   - preserve exact labels/action IDs without normalization.

5. **Cross-project boundary**
   - no ViewWitness crate dependency required for the core exporter;
   - document that output is intended evidence and not a `Witness`;
   - no capture parsing/comparison in M33.

6. **Regression coverage**
   - Project Browser at 1440×900;
   - M31 overlay pressure at 1440×900;
   - M32 Reader overflow pressure at 1440×900;
   - deterministic repeated serialization;
   - unused declaration omission;
   - dominant region + element target preservation;
   - command action preservation;
   - exact LayoutPlan-bound equality;
   - no element geometry or fixture/runtime-state leakage.

7. **Publication**
   - update README with M33 implementation status;
   - run formatting/tests/checks/diff check;
   - no screenshot QA unless implementation unexpectedly changes visible preview behavior;
   - publish, comment implementation issue, leave open for Director audit;
   - do not start M34.
