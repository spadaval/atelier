# The Desk's app

The web app of the Desk, the human's tool for seeing what is proposed and in
motion across his repositories, stopping a mission, and reading results
([product intent](../docs/product/desk.md)). It shows the repositories,
each mission's page, each item's page, what is proposed, what is in motion,
what is disputed, the inbox and T3 Code's live threads. It reads everything
from the API that `atelier-desk` serves ([the
API](../docs/architecture/desk.md)), and sends the human's two actions there:
the stop of a proposed plan, and a planning session. It runs no server of
its own.

It is React with Vite and react-router, Markdown through react-markdown,
lucide icons and one plain stylesheet with tokens, carried over from the
mission-control spike. The API's types in `src/api/` are generated from
`crates/atelier-desk/src/api.rs`; do not edit them by hand.

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
$ target/debug/atelier-desk --config ~/.config/atelier-desk/config.toml
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
`atelier-desk`. Its `World` performs the stop and sessions on a fresh copy
of the fixtures for each test, and records each request it was sent. It loads each page at a desktop and a phone width. CI runs
all four in the `desk` job when a change touches `desk/`.

## Against the real server

Two commands in `live/` check the Desk against the live repositories and
the CLI built from this checkout. They are not part of `pnpm e2e` or the
`desk` CI job, which builds no Rust. Each prints one line per comparison
and exits 1 on any difference. Build first: `pnpm build`, and at the
repository's root, `cargo build -j 4 -p atelier-cli` and `cargo build -j 4
--release -p atelier-desk`.

**desk-matches-cli** reads the running service. For every mission
`atelier list` shows in each repository given, proposed ones included, it
loads the mission's page in a headless browser, fails on a page or console
error, and compares the title, status and per-status item counts with
`atelier show <mission>` there; then it compares each repository's
needs-human items in the inbox with `atelier list --needs-human` there:

```console
$ pnpm desk-matches-cli http://buildbot.tail47515a.ts.net:3780 ~/projects/atelier \
    ~/projects/astromech ~/projects/demiurge ~/projects/life-on-rails ~/projects/cosplay-studio
```

**action-is-a-record** never touches the repository it is given or the
service. Under a new scratch directory in `~/.cache`, it makes a bare copy
of the repository's base branch to stand for the forge, proposes a mission
there on its own branch, and clones it twice. It stops the plan from the
mission's page on its own `atelier-desk` (loopback, no T3 Code, a `gh`
stand-in) in one, and runs `atelier drop <mission> --reason <reason>` and a
commit in the other. It checks that the Desk pushed one commit and closed
the pull request, compares the two commits' messages, paths and records,
checks the page shows the plan stopped, and removes the scratch directory.
The repository must name a forge:

```console
$ pnpm action-is-a-record ~/projects/atelier
$ pnpm action-is-a-record ~/projects/atelier --differ   # must fail
```

`--differ` gives the CLI's drop another reason, to show the comparison can
fail. `--atelier`, `--desk` and `--app` name other builds; by default they
are this checkout's `target/debug/atelier`, its release `atelier-desk` (else
the debug one) and `dist/`. `desk-matches-cli` takes `--atelier` too.
