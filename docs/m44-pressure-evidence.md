# M44 — Pressure Evidence

## User/product ambition

ViewWright's intended language has long included the requirement to describe what happens when windows are enlarged, shrunk, compact, or narrow rather than leaving resize behavior to backend physics.

The responsive roadmap preserved that ambition before implementation.

## Promotion condition

The roadmap required a concrete screen with intentionally different layouts that cannot be represented honestly by the current single-topology model.

The accepted M43 Lantern Leaf screen now satisfies that condition.

## Current failure

At every width, ViewWright currently uses the same:

- library 260px;
- reader grow;
- inspector 330px;
- bottom TTS topology;
- reader toolbar furnishing;
- TTS furnishing.

Below the intended wide desktop size, fixed surfaces simply consume the available width until the reader and local groups compress.

That behavior is accidental layout physics, not authored design.

## Why M44 before broader visual polish

Human QA has repeatedly identified crushed-darkness and richer typography as real visual pressure.

Those issues remain recorded.

However, palette/type refinement does not answer which semantic surfaces exist or where they go at different viewport sizes.

Responsive topology is therefore the stronger remaining architectural gap and can be established without choosing the final visual system.

## Why width only

The north-star pressure is presently horizontal: library + dominant reader + inspector competing for width.

Height-specific layout changes have not yet produced an independently authored requirement.

M44 does not speculate beyond the evidence.

## Why alternate roots plus sparse region overrides

Existing composition/furnishing languages already express the structures needed at each state.

The missing capability is selection among those authored structures.

Reusing them avoids a second responsive-only box language.

## Why dormant fixture content

Narrow layouts may intentionally omit library or inspector surfaces.

Deleting their fixture state would conflate viewport presentation with application data.

Keeping it dormant preserves semantic state across resize while exporting/rendering only what is active.

## Why dominant must survive

The reader is the durable center of the Lantern Leaf product.

Allowing design.dominant to disappear would make a responsive state contradict the screen's own hierarchy intent.
