# ViewWright — QA Execution Policy

## Hard invariant

Automated ViewWright development and verification MUST NOT commandeer the operator's interactive desktop.

This is an execution constraint, not a preference.

Routine QA must not:

- launch a visible native preview window;
- steal keyboard focus;
- move or synthesize the operator's real pointer;
- capture the operator's real keyboard;
- resize, move, minimize, maximize, or otherwise manipulate operator windows;
- inject input into the operator's compositor/session;
- require the human to click, scroll, drag, resize, or screenshot routine regression cases.

If a verification requirement cannot be completed without doing one of those things, the automated worker must STOP and report the limitation.

It must not silently fall back to desktop automation.

## Default QA substrate

Prefer, in order:

1. headless `egui::Context` execution with synthetic `RawInput`;
2. real `egui::FullOutput` inspection;
3. AccessKit / ViewWitness semantic evidence;
4. backend-independent LayoutPlan assertions;
5. real paint-shape / clip-rectangle assertions;
6. deterministic structured QA reports;
7. offscreen rasterization or an isolated virtual display only when pixels materially add evidence and no visible operator desktop is touched.

The existing ViewWright exact-verification suite already demonstrates the core pattern:

```text
ResolvedBlueprint
→ synthetic viewport RawInput
→ real viewwright_egui::show
→ real egui::FullOutput
→ ViewWitness
→ validation
→ M35 comparison
```

Future UI exploration should extend this pattern instead of opening native windows.

## Synthetic interaction

Automated QA may drive renderer-local UI using synthetic egui input inside the headless context.

Examples:

- pointer hover/click;
- scroll-wheel input;
- keyboard input;
- choice/select opening and option selection;
- boolean toggling;
- scalar adjustment;
- repeated viewport resize frames;
- focus changes inside the headless egui context.

These are test-context interactions only.

They must never control the operator's real input devices.

## Offscreen visual evidence

When visual evidence is useful, generate it offscreen.

Acceptable outputs include:

- deterministic paint/shape manifests;
- clip/bounds reports;
- SVG/PNG captures produced without a visible native window;
- semantic overlays derived from real render output.

Do not open a native preview merely to obtain screenshots.

A raster screenshot is not automatically required for every milestone. If semantic, layout, and paint-level evidence completely establish the acceptance condition, structured evidence is sufficient.

## Human role

Human review is reserved for questions that are genuinely subjective or product-directional, such as:

- whether a visual language feels good;
- whether a concept matches the intended product;
- whether an interaction model is desirable;
- whether a north-star design should change.

Human review is NOT the routine regression mechanism for:

- clipping;
- scrolling;
- breakpoint switching;
- stale/duplicate nodes;
- popup theming mechanics;
- control event emission;
- region reachability;
- exact geometry;
- author identity;
- renderer state persistence.

Those should become automated evidence.

## Interactive preview

Interactive/native preview remains a useful product-inspection tool, but it is opt-in.

A worker may launch or request it only when the human explicitly authorizes interactive QA for that specific session.

Authorization is not permanent and does not generalize to later milestones.

After the authorized inspection, any worker-launched preview must be closed unless the human explicitly requests otherwise.

## Failure handling

If a worker violates this policy:

- treat it as a workflow defect;
- document the violation;
- stop repeating the same mechanism;
- add or strengthen automated coverage so the human is not asked to perform the same routine check again.

## Director workflow

Default milestone verification flow:

```text
Director authority
→ Codex implementation
→ headless exploration/regression suite
→ deterministic QA bundle
→ Codex self-correction until green
→ Director code + evidence audit
→ milestone acceptance
```

Human product review is optional at meaningful product boundaries and should not gate routine engineering milestones unless the authority explicitly identifies a genuinely subjective acceptance question.
