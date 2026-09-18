# M41 — Stop Condition

Stop M41 when ViewWright can express and render region-local furnishing trees using existing element semantics.

Required stop state:

- optional region furnishing root exists;
- row/column furnishing structures resolve and validate;
- branch furnishing slots use deterministic fixed-plus-grow allocation;
- padding/gap and vertical overflow work;
- leaf furnishings render existing elements in authored order;
- no furnished region double-renders legacy flat content;
- all existing unfurnished specimens preserve behavior;
- semantic/ASCII/concept projections expose furnishing structure;
- furnishings are structural, not visible panels and not M34 author identities;
- M33 expectation remains 0.1 unchanged;
- M35 exact comparison semantics remain unchanged;
- prior exact-verification tests remain green;
- new Lantern Leaf furnished specimen passes exact verification;
- human QA confirms the M40 toolbar/TTS allocation artifacts are materially improved;
- M40 baseline remains untouched;
- no new control kinds;
- no responsive syntax;
- no M42.

After acceptance, the next v0.2 pressure should be assessed from the furnished north-star specimen rather than assumed in advance.
