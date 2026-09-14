# M6 — Reader Palette Pressure Metrics

This document records the human-QA pressure that motivated M6 using deterministic sRGB relative-luminance/contrast calculations.

## Canonical M5 visual Reader palette

```text
canvas          #101216
panel           #171A20
raised          #1E222A
text            #E8EAF0
text_muted      #9299A6
accent          #8DB7C7
border          #2B313B
```

Approximate relative luminance:

```text
canvas          0.0060
panel           0.0103
raised          0.0159
text            0.8229
text_muted      0.3165
accent          0.4365
border          0.0303
```

Representative foreground contrast remains strong:

```text
text / canvas           15.59 : 1
text / panel            14.49 : 1
text / raised           13.25 : 1
text_muted / panel       6.08 : 1
accent / raised          7.39 : 1
```

But structural surface separation is compressed:

```text
canvas / panel           1.08 : 1
panel / raised           1.09 : 1
canvas / raised          1.18 : 1
```

Human QA described the result as still "dark as fuck" even after M5 made the Reader content representative and M4 made the layout sane.

The useful conclusion is that text contrast and overall palette legibility are not the same question.

## M6 pressure palette

The separate M6 pressure specimen uses:

```text
canvas          #262B33
panel           #313844
raised          #3E4856
text            #F2F4F7
text_muted      #B9C0CA
accent          #9BCBE0
border          #566172
```

Approximate structural relative luminance:

```text
canvas          0.0238
panel           0.0390
raised          0.0633
```

Approximate structural separation:

```text
canvas / panel           1.21 : 1
panel / raised           1.27 : 1
canvas / raised          1.54 : 1
```

Representative foreground contrast remains strong, including muted/accent text on the raised surface above 5:1.

This palette is intentionally only a pressure direction. M6 human QA decides whether it actually improves the screen without making it washed-out or losing the intended quiet/reading-first character.
