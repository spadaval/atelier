# The Desk's app

The web app of the Desk, the human's tool for planning missions and reading
their results ([product intent](../docs/product/desk.md)). It shows the
repositories, each mission's page, each item's page and the inbox, and it
reads everything from the API that `atelier-desk` serves
([the API](../docs/architecture/desk.md)). It runs no server of its own.

It is React with Vite and react-router, Markdown through react-markdown,
lucide icons and one plain stylesheet with tokens, carried over from the
mission-control spike. The API's types in `src/api/` are generated from
`next/crates/atelier-desk/src/api.rs`; do not edit them by hand.

## Work on the app

A change to the app needs no Rust build. Start `atelier-desk` once, then run
Vite's dev server, which sends `/api` to it:

```console
$ pnpm install
$ ATELIER_DESK=http://127.0.0.1:3780 pnpm dev
```

`ATELIER_DESK` defaults to `http://127.0.0.1:3780`, `atelier-desk`'s default
address. `DESK_DEV_PORT` sets the dev server's port (default 5173).

## Serve the built app from `atelier-desk`

```console
$ pnpm build
$ cd ../next && cargo build -p atelier-desk
$ target/debug/atelier-desk --config ~/.config/atelier-desk.toml
```

with `app` in the configuration naming this directory's `dist`, for
example `app = "/home/me/atelier/desk/dist"`. The server answers every path
outside `/api` with the app.

## Checks

```console
$ pnpm typecheck
$ pnpm test        # unit tests (Vitest), in src/**/*.test.ts
$ pnpm build
$ pnpm exec playwright install chromium-headless-shell   # once
$ pnpm e2e         # Playwright, on the built app
```

`pnpm e2e` serves `dist/` with Vite's preview server and answers every
`/api` request in the browser from `e2e/fixtures.ts`, so it needs no
`atelier-desk`. It loads each page at a desktop and a phone width. CI runs
all four in the `desk` job when a change touches `desk/`.
