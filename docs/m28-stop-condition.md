# M28 — Stop Condition

Stop M28 once present status `text` fixture payloads reject empty/whitespace-only strings, valid strings remain exact, omission remains legal, accepted canonical fixtures resolve unchanged, and the renderer/layout/projection stack is untouched.

Do not continue into generalized text policy, status semantics, command-reason rules, property-value rules, paragraph rules, localization, or M29.