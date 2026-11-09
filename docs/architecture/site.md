# Site

`atelier serve` is a read-only server over every Atelier repository on this
host ([ADR 0030](../adr/0030-a-read-only-site-a-session-database-and-t3-code.md)).
It serves each mission's brief and debrief as JSON, read from one captured
commit of the repository's base branch, never from a checkout, and writes
nothing to any repository. The design is in
[the fleet-dashboard strategy](../strategies/fleet-dashboard.md) and decisions
atelier-c5vk and atelier-8p7u; this page says how the code does it.

## Crates

`crates/atelier-site` owns discovery, the routes, the cache and the load
limits. `atelier-cli` parses `atelier serve [--bind <addr>] [--root <dir>...]`
(defaults `127.0.0.1:7341` and `~/projects`) and calls `atelier_site::serve`,
so the site ships in the one `atelier` binary. The direction is
`atelier-cli` → `atelier-site` → `atelier-app`; the site never depends on the
CLI. HTTP is `axum` on a `tokio` runtime with two worker threads and at most
four blocking threads.

`crates/atelier-site/web/` holds only two generated directories until the app
arrives (atelier-yn69): `src/api/`, the TypeScript types of every response,
and `src/fixtures/`, each route's response on the fixture repositories.

## Reading a repository at a commit

`atelier_app::commit_records::load` builds a `RecordIndex` from a commit's
tree in memory: it lists `.atelier/` through the read phase's
`git cat-file --batch` process (`ReadContext::files_under`), parses each blob,
and validates the records against the workflow policy and link types that
same commit holds (`RecordIndex::from_commit_files`). There is no temporary
directory, and the index names no repository, so no error message or
not-found lookup consults a checkout or another ref.

The brief and debrief views take a `ReadContext`. One made with
`ReadContext::at_commit(common_dir, commit)` reads one commit and nothing
else:

- `HEAD` is the captured commit, so `generated_from`, criteria marks, receipt
  ages and the history walks read it, not the common directory's `HEAD`,
  which is whatever the main checkout has out.
- Files such as the strategy are read from the commit's tree
  (`ReadContext::file_text`), for the brief's sections, the debrief's Why and
  the criteria.
- The history walk does not look for a fork from the base branch, since the
  commit is the base (`record_history::fork`).
- `code_clean` is true: a commit holds no uncommitted code.

A `ReadContext::new` made by the CLI reads the working tree and `HEAD` as
before, so `atelier brief` and `atelier debrief` print what they did.

## Discovery

`discovery::discover` walks each root, at most four directories deep, for
checkouts holding `.atelier/`. It does not descend into a checkout, hidden
directories, `node_modules` or `target`. Checkouts are grouped by
`git rev-parse --git-common-dir`, so a repository with several worktrees
appears once.

Each common directory is read from the checkout's `.git` as Git reads it: a
directory, or a `gitdir:` file naming a worktree's directory, whose
`commondir` file names the common one. `discovery::base` reads a
repository's base branch from refs and commits alone, with one
`git for-each-ref` in the common directory:

1. The default branch is the one `refs/remotes/origin/HEAD` names, else
   whichever one of local `main` and `master` exists.
2. The workflow policy committed on the default branch names the base branch.
3. If that is another branch, the policy committed there must name it too.

A repository with no default branch, with both `main` and `master` and no
`origin/HEAD`, whose base has no local branch, or whose base's policy names
another branch, shows that error in `/api/repos` and from its routes. Nothing
falls back to `HEAD` or a working tree.

A repository's name in URLs is the `project_slug` committed on its base,
else its directory name. In common-directory order, the first repository
with a name keeps it, and each repeat takes the first `<name>-<n>`, from 2,
that no repository has, so `demo`, `demo` and `demo-2` are served as `demo`,
`demo-3` and `demo-2`.

## API

Every route is a `GET` returning JSON. The bodies are `atelier-app` view
models serialized with `serde`, plus the base commit and links
(`crates/atelier-site/src/api.rs`):

| Route | Body |
| --- | --- |
| `/api/repos` | Each repository: base branch and commit, `atelier status`'s counts, and its missions, or the error that stopped them. |
| `/api/<repo>/mission/<id>` | One mission's id, title, status, category, links and `review`. |
| `/api/<repo>/mission/<id>/brief` | The `Brief` view `atelier brief` prints. |
| `/api/<repo>/mission/<id>/debrief` | The `Debrief` view `atelier debrief` prints. |
| `/api/<repo>/mission/<id>/strategy` | The strategy's path and Markdown as committed. |

`review` is the one judgment the server makes: `brief` while the mission is
in its workflow's initial status, `debrief` afterwards, read from the
committed policy. A mission accepts a full or unique short ID, as the CLI
does.

An unknown repository, mission, route or a mission without a strategy is a
404, and a repository whose base or records cannot be read is a 500 naming
the error, from its own routes only; the others answer. Every error body is
`{"error": "..."}`.

The TypeScript types in `web/src/api/` come from the Rust types through
`ts-rs`, one file per type. `api::tests::typescript_types_are_current` fails
when they are stale. The fixture JSON in `web/src/fixtures/` is written by
`crates/atelier-cli/tests/serve/api.rs` from two fixture repositories built
with fixed dates, and `fixture_responses_match_the_checked_in_json` fails when
it is stale. Rewrite both with `ATELIER_UPDATE_FIXTURES=1` and the same tests.

## Cache and load limits

Only work that depends on nothing but a base commit is cached: the records
loaded from it and each view built from them, as serialized JSON, keyed by
repository and base commit. A newer base commit replaces a repository's
entry, and a scan drops the entries of repositories that are gone, so only
the latest commit of each repository is kept. What the policy and config of
a default or base commit say is kept the same way. Only successes are kept:
a load, view or policy read that fails is tried again by the next request,
since the failure may be Git failing for a moment rather than the commit.
The views read a missing strategy as no strategy, so a read phase over a
commit tells a file the commit does not hold from one Git could not read
(`ReadContext::failed_read`); after the second, the view or the commit's
facts are an error, never kept, rather than a page that misstates the
commit.

Refs are never cached. Every request scans the roots and reads each
repository's refs again, so it reads the base as of its arrival; requests
that arrive while a scan is running wait for the next one, which serves them
all.

A record load or view build runs on a blocking thread once it holds one of
two build slots, which it keeps until the work ends even if its request has
gone, and checks the cache again once it has one, so twenty
requests for the same cold view build it at most twice. A request that waits
more than five seconds for a slot gets a 503 with `Retry-After: 1`
(`builds_are_capped_and_a_long_wait_is_a_503`). The cap is a semaphore around
builds rather than a `tower` concurrency layer, so warm requests never queue
behind cold ones.

## Writing nothing

`Site::new` turns Git's optional locks off for every later Git command
(`git_freshness::read_only`, `GIT_OPTIONAL_LOCKS=0`). The server runs only
reading Git commands, in the common directory, where no index is refreshed.
`crates/atelier-cli/tests/serve/read_only.rs` makes every fixture
repository's working tree and `.git` unwritable and checks that every route
answers as before and that refs, the index and `git status` are unchanged.
