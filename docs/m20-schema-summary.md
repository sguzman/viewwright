# M20 — Schema Summary

No new TOML syntax is introduced.

Existing source shape remains:

```toml
[screen]
id = "reader_workspace"
purpose = "Read a document while controlling navigation and narration"
```

`ScreenSource.purpose` remains `String` and `ResolvedScreen.purpose` remains `String`.

The only semantic change is validation: `screen.purpose` must contain at least one non-whitespace character.

Validation must not trim, rewrite, title-case, normalize Unicode, infer, or generate accepted purpose text.
