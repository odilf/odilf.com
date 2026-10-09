# Rewrite to Svelte

## Purpose

Plan for replacing the custom Rust static-site generator with an idiomatic
SvelteKit app, keeping the site's look essentially the same while dropping
strict parity where it buys clarity.

## Reference revision

The Rust implementation this plan works against is jj revision `yspvxruy`
(change id `yspvxruyulpz`, git commit `32bbb187575b3bf68f76346fc0b91ae3449bac09`,
"tried to fix something, but I'll just use svelte").

It is the last revision containing the working Rust generator. The commit
immediately after it (`owmztlownzlo`, "scaffold svelte project") removes the
Rust tree and adds the current SvelteKit scaffold.

To extract the reference tree:

```sh
jj log -r yspvxruy --no-graph -T 'commit_id ++ "\n"'   # 32bbb187575b3bf68f76346fc0b91ae3449bac09
git archive --format=tar 32bbb187575b3bf68f76346fc0b91ae3449bac09 | tar -x -C /tmp/odilf-rust
```

## Current state (what exists today)

- **Build model**: `src/main.rs` reads markdown from two env-pointed directories,
  fetches projects/Immich data, renders Maud templates, and writes a fully static
  HTML tree to `target/{debug,release}/site/`.
- **Deploy**: `wrangler pages deploy target/release/site/` → Cloudflare Pages,
  static files only.
- **Routes produced**: `/`, `/about`, `/blog/` + `/blog/<slug>/`, `/media-log/` +
  `/media-log/<slug>/`, `/projects/`, `/pics/` + `/pics/<id>/`, plus
  `/blog/rss.xml`, `/blog/atom.xml`, `/static/media-log.json`.
- **Data sources**: blog markdown (env `ODILF_BLOG_PATH`), media-log markdown
  (env `ODILF_MEDIA_LOG_PATH`), committed `projects.toml`, Immich album API (env
  `IMMICH_URL`/`IMMICH_ALBUM_ID`/`IMMICH_API_KEY`), theme in `localStorage`.
- **Existing scaffold**: SvelteKit `3.0.1` + Svelte `5.57.2`, Tailwind v4 via
  `@tailwindcss/vite`, `adapter-auto`, enhanced-img, vitest; `experimental.async`
  and `experimental.remoteFunctions` both enabled.

## Preflight findings

Verified 2026-10-09 against the live environment. These change or unblock parts of
the plan below.

**Reference build works.** `/tmp/odilf-rust` builds with `nix develop --command
cargo build --release` (58s) and runs to completion, emitting 61 files: 15 blog
posts, 23 media-log entries, 0 pics. `tailwindcss` and `magick` are not on `PATH`
outside the flake devShell, so reference builds go through `nix develop`.

**Content paths were wrong.** `.env` pointed at `/home/odilf/brain/...`; this
machine is macOS with `$HOME=/Users/odilf`, so only `.envrc`'s direnv exports
resolved. `.env` is corrected locally (it is gitignored). Anything reading `.env`
directly — including the Kit build — found no content before this.

**Immich's album endpoint no longer returns assets.** `GET /api/albums/{id}`
returns album metadata only (the album holds 99 assets, no `assets` key).
`AlbumResponse.assets` is `#[serde(default)]`, so the Rust generator logs "Saved 0
photo metadata to cache" and succeeds with an empty gallery: `/pics/` lists
nothing and no `/pics/<id>/` pages exist. The reference site therefore cannot
reproduce the pics routes, and the port must source the list from `POST
/api/search/metadata` (`{"albumIds": [...], "withExif": true}` — captions come
from `exifInfo.description`) rather than translating the album call verbatim.

**Media-log covers are rate-limited.** 22 of 36 entries carry no `image-url` and
fall back to the Wikipedia lookup, which fails intermittently ("expected value at
line 1 column 1" — a non-JSON response); repeated runs converge (20 → 23 entries).
The cover cache is also written to a hardcoded
`target/debug/site/media-log/covers-cache/` even in release, a latent bug in the
Rust tree. The port should retry and use one stable cache path.

**Content inventory.** 40 blog markdown files: 15 `draft: false`, 18 `draft:
true`, 7 with no `draft` key — release ships only the 15, so "exclude unless
explicitly `false`" is the rule to match. 36 media-log files. Five subdirectories
are skipped: `historias-de-quarentena/`, `the-actual-point-of-rust/`,
`the-point-of-math/`, `trenitalia-bingo/`, `why-is-jonathan-blow-an-asshole/`.
Blog images are copied from the content dir into `blog/<slug>/`, which is how
`/blog/trenitalia-bingo/*.jpg` exists with no post of that slug.

**Feeds cannot be byte-compared.** RSS `lastBuildDate` and Atom `updated` are
build-time `now()`. Validate structure, don't diff bytes.

**Visual-diff harness works.** Reference served with `nix develop --command
static-web-server --root target/release/site --port 4173`; screenshots via the
repo's `playwright` with `executablePath` pointed at the `chromium-1243` build in
`~/Library/Caches/ms-playwright` (playwright 1.64 wants
`chromium_headless_shell-1248`, which is not installed). Shots of `/`, `/about`,
`/blog/`, a post, `/media-log/`, `/projects/`, `/pics/` are in `/tmp/ref-shots`.

## Decisions

| Decision         | Choice                                                                                                                                       | Rationale                                                                                                                                                                                   |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Build model      | SSG — `adapter-static` + `prerender = true` in root `+layout.ts`                                                                             | Mirrors today exactly. No runtime content, so SSR buys nothing.                                                                                                                             |
| URLs             | `trailingSlash = 'always'` in root `+layout.ts`                                                                                              | Current URLs are `/blog/<slug>/`, `/pics/<id>/`; produces `.../index.html` like the Rust generator.                                                                                         |
| Content location | Keep external dirs via env, read at build time                                                                                               | Preserves the Obsidian `~/brain` writing workflow. Requires content present on the build machine (deploys are local today).                                                                 |
| Env API          | SvelteKit 3 `src/env.ts` + `defineEnvVars` + `$app/env/private`                                                                              | `$env/dynamic/private` is deprecated in Kit 3.                                                                                                                                              |
| Markdown         | `unified` (remark/rehype) + `gray-matter` + `remark-gfm` + `remark-math` + `rehype-katex`, custom plugins for heading-bump and image rewrite | Most extensible; needs comrak parity (`unsafe_`, footnotes, math, figure captions, summary, word count).                                                                                    |
| Images           | Port Rust preprocessing to `scripts/fetch-pics.ts` using `sharp` (drop ImageMagick) → webp + 400px thumbs + metadata JSON                    | `@sveltejs/enhanced-img` only transforms imported assets; photos are runtime-dynamic. Keep enhanced-img for the logo only. Source the list via `POST /api/search/metadata` — see preflight. |
| Projects fetch   | Port `src/bin/fetch.rs` → `scripts/fetch-projects.ts`; keep committed `projects.toml`; parse with `smol-toml`                                | Keeps `GITHUB_TOKEN` out of the build.                                                                                                                                                      |
| Theme            | `src/lib/theme.svelte.ts` rune module + blocking inline script in `app.html`                                                                 | Avoids FOUC; idiomatic rune equivalent of `theme_switcher.js`.                                                                                                                              |
| Feeds / JSON     | `+server.ts` endpoints with `export const prerender = true`                                                                                  | Idiomatic; emits static `rss.xml`/`atom.xml`.                                                                                                                                               |
| Rust tree        | Keep until parity is confirmed; delete in the cutover phase                                                                                  | Cheap insurance.                                                                                                                                                                            |

### Refinements to the original plan

The starting proposal was: templates with mock data → make idiomatic → data
fetchers → leftovers. Two adjustments:

1. **Make it idiomatic in the first pass, not a separate one.** Data shapes are
   fully known from the Rust structs, so define TS types up front and write each
   page with runes/components against typed fixtures. A separate idiom pass is a
   full rewrite for little gain.
2. **Do the shell/theme first.** The look is ~90% `public/app.css` + `fonts.css`
   - layout. Pages written before that lands all get restyled afterward.

## Route map

| URL                      | Source (Rust)                 | SvelteKit route                         |
| ------------------------ | ----------------------------- | --------------------------------------- |
| `/`                      | `lib.rs::home`                | `routes/+page.svelte`                   |
| `/about`                 | `lib.rs::about`               | `routes/about/+page.svelte`             |
| `/blog/`                 | `blog/mod.rs::home`           | `routes/blog/+page.svelte`              |
| `/blog/<slug>/`          | `BlogEntry::render`           | `routes/blog/[slug]/+page.svelte`       |
| `/blog/rss.xml`          | `blog/feed.rs::rss`           | `routes/blog/rss.xml/+server.ts`        |
| `/blog/atom.xml`         | `blog/feed.rs::atom`          | `routes/blog/atom.xml/+server.ts`       |
| `/media-log/`            | `media/mod.rs::home`          | `routes/media-log/+page.svelte`         |
| `/media-log/<slug>/`     | `MediaLog::render`            | `routes/media-log/[slug]/+page.svelte`  |
| `/projects/`             | `projects/mod.rs::home`       | `routes/projects/+page.svelte`          |
| `/pics/`                 | `pics/mod.rs::home`           | `routes/pics/+page.svelte`              |
| `/pics/<id>/`            | `pics/mod.rs::pic`            | `routes/pics/[id]/+page.svelte`         |
| `/static/media-log.json` | `main.rs::generate_media_log` | prerendered `+server.ts` (preserve URL) |

## Plan

### Phase 0 — Foundation and shell

- Port `public/app.css` → `src/routes/layout.css`, keeping `@theme`,
  `@theme inline`, and `@layer components`; port `fonts.css`.
- **Remove the `@tailwindcss/typography` plugin.** The scaffold imports it, but
  the Rust site hand-rolls its own `.prose` in `@layer components`. Keeping both
  yields two competing `.prose` definitions and a different look.
- `+layout.svelte` + `app.html` (theme bootstrap, favicon, feed `<link>`s,
  `font-mono` body); `src/lib/theme.svelte.ts`.
- Shared components: `Back`, `Tag`, `ThemeToggle`, `Rating`, `BlogEntryCard`,
  `MediaLogCard`, `ProjectCard`, `PhotoThumb`.
- `src/lib/types.ts` (mirror Rust structs), `src/env.ts`, `src/lib/date.ts`
  (port of `media::Date` Display), `src/lib/markdown/` stubs,
  `src/lib/fixtures/`.
- Trim scaffold: drop `experimental.async` / `remoteFunctions` (unused for SSG),
  drop `adapter-auto`, remove the vitest `Welcome` examples once real tests exist.

### Phase 1 — Views against typed fixtures

Build every route with interactive parts as Svelte state (no `.blog-entry` /
`.topic-x` DOM manipulation):

- `/`, `/about`
- `/blog` (topic filter via `$state`/`$derived`) + `/blog/[slug]`
- `/media-log` (type + star-rating filters) + `/media-log/[slug]`
- `/projects`
- `/pics` + `/pics/[id]` (random/prev/next from the id array)

Run `svelte-autofixer` on each component; keep `pnpm check` green; visual-diff
against the old site.

### Phase 2 — Real data layer

- Markdown pipeline: frontmatter, summary (first ~250 chars of text nodes),
  word count, heading level bump, `/blog/<url>` image rewriting, KaTeX.
- Server loaders: `blog.ts` (read dir, parse, sort by date, dev-shows-drafts at
  50% opacity vs build-excludes), `media-log.ts` (+ Wikipedia cover fetch cache),
  `projects.ts`, `pics.ts`.
- `scripts/fetch-projects.ts`, `scripts/fetch-pics.ts`.

### Phase 3 — Wiring and prerender

- Swap fixtures for `+page.server.ts` loads; `entries()` on `[slug]` / `[id]`.
- Root `+layout.ts`: `prerender = true`, `trailingSlash = 'always'`; install
  `adapter-static`.
- `csr = false` on static content pages (`/about`, `/blog/[slug]`,
  `/media-log/[slug]`) — matches today and is a real perf win.
- `pnpm build`; verify output tree, feeds (validator.w3.org), and
  `/static/media-log.json`.

### Phase 4 — Cutover

- Update `flake.nix` deploy (`target/release/site/` → `build/`), `justfile` /
  `package.json` scripts to pnpm, `.env` / `.envrc` / `.env.example`.
- Delete Rust (`src/bin`, `src/blog`, `src/media`, `src/pics`, `src/projects`,
  `src/lib.rs`, `src/main.rs`, `Cargo.*`, `public/`); update README; keep
  `robots.txt` and add a `404`.
- Deploy to Pages and compare side by side.

## File layout

```
src/
  env.ts
  app.html                      # inline theme bootstrap
  lib/
    types.ts  theme.svelte.ts  date.ts
    markdown/{index,blog,media}.ts
    server/{blog,media-log,projects,pics,feeds}.ts
    components/{Back,Tag,ThemeToggle,Rating,BlogEntryCard,MediaLogCard,ProjectCard,PhotoThumb}.svelte
    fixtures/
  routes/
    +layout.ts  +layout.svelte  +page.svelte
    about/+page.svelte
    blog/{+page.server.ts,+page.svelte,[slug]/{+page.server.ts,+page.svelte},rss.xml/+server.ts,atom.xml/+server.ts}
    media-log/{...,[slug]/...}
    projects/+page.{server.ts,svelte}
    pics/{...,[id]/...}
scripts/{fetch-projects,fetch-pics}.ts
```

## Gotchas

- **Typography plugin vs `.prose`** — the single biggest "why does it look
  different" trap. See Phase 0.
- **Feed specifics** — Atom uses CDATA-wrapped HTML content and offset-stripped
  dates; RSS uses `pubDate`/`guid`. Validate both.
- **Drafts** — only excluded in release; dev shows them at 50% opacity. Use
  `building` from `$app/env` (or `dev` from `$app/environment`).
- **Nested content dirs are currently skipped** (`historias-de-quarentena/`,
  `the-point-of-math/older/`, `why-is-jonathan-blow-an-asshole/`). Decide whether
  to keep "top-level `*.md` only" or consciously change it.
- **Blog image assets come from outside the repo** and Rust copied them into the
  output. With Vite hashing, decide between a prebuild copy into a gitignored
  `static/blog/` vs. `?url` imports; the rehype rewrite must match.
- **`/static/media-log.json`** — either a `scripts/build-content.ts` writes it to
  `static/`, or a prerendered `+server.ts` at `static/media-log.json/` preserves
  the URL.
- **Immich builds need LAN access** (`.env` points at `http://192.168.0.40:2283`).
  Pin the cache so photo pages don't re-download every build.
- **`@html` is required** for rendered markdown; content is trusted local files,
  but keep it scoped to markdown output.

## First milestone

Phase 0 plus `/` and `/blog` on fixtures, `pnpm check` green and visually
matching. Validates the theme layer, layout, runes idioms, and types before any
data plumbing.
