# M22 — Stop Condition

Stop M22 when:

- empty fixture state is rejected;
- whitespace-only fixture state is rejected;
- valid state text is preserved exactly;
- missing `state` remains a parse/deserialization error;
- accepted canonical fixtures remain unchanged;
- valid semantic/debug projection remains unchanged;
- no broader state/copy system is introduced.