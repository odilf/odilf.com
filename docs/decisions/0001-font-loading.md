# 0001 — Font loading

## Status

Accepted

## Context

Iosevka was loaded from jsDelivr through nine unversioned `@font-face` rules with
`font-display: swap` and a fallback whose metrics did not match the webfont. Each
fontsource "latin" file is ~1 MB, so a page fetched roughly 5 MB from a
third-party origin, and the swap reflowed text once the webfont arrived. The
result was non-blocking, but it shifted layout.

## Decision

- Self-host the five weights the site uses (300/400/600/700/900) from the pinned
  `@fontsource/iosevka` package instead of jsDelivr.
- Subset each weight at build time to the codepoints present in content and
  source, via `pnpm subset:fonts`, preserving programming ligatures.
- Keep `font-display: swap`, paired with metric-matched fallback faces:
  `size-adjust`, `ascent-override`, `descent-override`, and `line-gap-override`
  are set so the fallback occupies the same advance width and line box as
  Iosevka. The swap then does not move anything.
- Preload the 400 weight.

## Consequences

- Text paints immediately in a fallback that is metrically identical to Iosevka,
  then swaps with no layout shift.
- Ligatures dominate the subset: keeping them is ~363 KiB across the five
  weights, versus ~61 KiB without. `KEEP_LIGATURES` in `scripts/subset-fonts.ts`
  flips the trade.
- The overrides are derived from Iosevka v22.1.2 metrics (`unitsPerEm` 1000,
  advance 500, ascent 965, descent 215, lineGap 70). Updating the font means
  regenerating them.
- A codepoint the scanner misses falls back per character without shifting
  layout, because the fallback metrics already match.
