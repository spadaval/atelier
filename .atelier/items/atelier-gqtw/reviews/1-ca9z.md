---
schema: atelier.review
schema_version: 3
round: 1
commit: 919e3952bdaef9566e796afa15a1a548cac0b2dd
reviewers:
- thread:delegated-task:command%3Amcp%3A5c01d7ea-dd61-466c-857a-4beef42607cc%3Adelegate-task%3A03a91f02-314c-416a-b86f-1f62413d647f
model: gemini-4-neon
verdict: changes
created_at: 2026-11-22T09:40:19.975190874Z
---

## Summary

- **Complexity:** medium — spans single-branch drops, post-merge PR closing and branch deletion, base-fetch claim/submit guards, detached-HEAD protection, status hygiene reporting, Desk PR fallback, eval fixtures, and docs across `atelier-core`, `atelier-cli`, and `atelier-desk`, with each piece fitting the existing architecture.
- **Value:** high — eliminates two-branch drop ceremony and prevents ongoing accumulation of merged local and remote branches while preserving claim safety, unattached commits, and Desk PR links.
- **Risk of merging it:** medium — `merge` now deletes local and remote branches and `start`/`submit` fetch the remote base, affecting every clone and remote; lease-checked deletion, base-containment gating, and `Located::record` rule 1 contain the branch-deletion risk, but Finding 1 breaks Desk's shared mission cache between `GET /api/repos/{repo}/missions/{id}` and `POST /api/sessions`.

Checked:
- Read the full diff against `master` at `919e3952bdaef9566e796afa15a1a548cac0b2dd` across all 38 files, plus surrounding code in `crates/atelier-cli/src/{cleanup.rs,commands.rs,git.rs,merge.rs,plan.rs,review.rs,check.rs,output.rs}`, `crates/atelier-core/src/{forge.rs,git.rs,plan.rs,repo.rs,say.rs,view.rs}`, `crates/atelier-desk/src/{server.rs,read.rs}`, tests, evals, and docs against every Done when line and Report claim.
- Ran `cargo fmt -- --check`, `git diff --check`, and `cargo clippy --all-targets --locked -- -D warnings`: clean.
- Ran `cargo nextest run --locked` (whole workspace): 456 passed, 1 skipped.
- Built `target/debug/atelier` and ran `evals/agent-factory/test.sh target/debug/atelier` and `target/debug/atelier check`: all eval fixtures and harness tests pass, and every record validates.
- Ran `desk/` tests (`vitest run` via pnpm): 10 files, 57 passed.
- Verified PR #278 (`919e3952bdaef9566e796afa15a1a548cac0b2dd`) CI checks: `changes`, `plan`, `test`, `evals`, and `extension` all succeeded.

Could not check:
- Live GitHub lease-checked remote branch deletion and PR close (only exercised via local bare remote and fake `gh`).
- Browser-backed `t3-extension/run-tests.mjs` tests locally: `test-fixture-desk.mjs` and `test-update.mjs` passed, while headless Chrome in `test-components.mjs` stalled in this environment (CI's `extension` check passed on the same head commit).

## Finding 1: Desk's `"mission {id}"` cache entry collides in type between `mission` and `session`, causing 404s
Recommendation: fix now

In `crates/atelier-desk/src/server.rs:572-574`, the `GET /api/repos/{repo}/missions/{id}` handler changed its cached value under route key `format!("mission {id}")` from `MissionPage` to `(MissionPage, String)`:

```rust
let (mut page, own) = desk.repo_route(&repo, &format!("mission {id}"), missing, |r| {
    Ok(read::mission(r, &id)?.map(|page| (page, r.branch_of(&id))))
})?;
```

However, `session` (`POST /api/sessions`) at `crates/atelier-desk/src/server.rs:890-892` still queries the exact same cache key `format!("mission {mission}")` with type `MissionPage`:

```rust
let page = desk.repo_route(&repo, &format!("mission {mission}"), missing, |r| {
    read::mission(r, &mission)
})?;
```

In `Desk::read` (`crates/atelier-desk/src/server.rs:225-230`), a cache hit returns `Ok(kept.downcast_ref::<T>().cloned())`, and `Desk::repo_route` (`crates/atelier-desk/src/server.rs:281-285`) turns `Ok(None)` into `Failure::NotFound` (HTTP 404). Because the two handlers store incompatible types under the same `(repo, "mission {id}")` slot:
- Opening a mission page (`GET /api/repos/{repo}/missions/{id}`) and then starting a planning session for it (`POST /api/sessions`) while refs are unchanged fails `downcast_ref::<MissionPage>()` and returns HTTP 404 `"No mission {id} is on the base or on a branch."`.
- Conversely, calling `POST /api/sessions` first poisons the `"mission {id}"` slot with `MissionPage`, causing subsequent `GET /api/repos/{repo}/missions/{id}` requests to fail `downcast_ref::<(MissionPage, String)>()` and return HTTP 404 until the repository's refs change.

`crates/atelier-desk/tests/sessions.rs` did not catch this because none of its tests call `GET /api/repos/demo/missions/{MISSION}` before or after `POST /api/sessions`. Make both callers cache the same type under `"mission {id}"` (or use distinct cache keys / put the branch lookup outside the cached tuple) and add a regression test covering `GET /api/repos/{repo}/missions/{id}` followed by `POST /api/sessions` (and vice versa) with unchanged refs.

## Finding 2: `added_there` prefers a stale local branch over a newer fetched remote-tracking branch
Recommendation: defer
Dropped by mcp:launch:b3b5ffd62a4c227e4ec8ce23f1e32b83538dd28e5e6c91e0a2d3d91ead863ef6: fixed on the item's branch in ce50941a9, with a regression test, before it merged

In `crates/atelier-cli/src/plan.rs:383-414`, `added_there` builds `tips` with the local branch first and the remote-tracking branch second, then deduplicates by item ID with `added.entry(other.to_string()).or_insert(found)`:

```rust
if let Some(tip) = crate::git::branch_tip(&root, &branch)? {
    tips.push((branch.clone(), tip));
}
if let Some(remote) = repo.remote() {
    if let Some(tip) = atelier_core::git::remote_tip(&root, remote, &branch)? {
        tips.push((format!("{remote}/{branch}"), tip));
    }
}
```

If one clone has both a local branch `<branch>` and a fetched `<remote>/<branch>` that is strictly ahead (for instance, after another clone took over the item, updated a newly filed item's title/Goal/Done when, and pushed, and this clone fetched), `drop` prints the stale local title and `git checkout <branch> -- .atelier/items/<other>` rather than `git checkout <remote>/<branch> -- .atelier/items/<other>`, unlike `Repo::tip` (`crates/atelier-core/src/repo.rs:392`), which prefers the remote-tracking tip when the local tip is its ancestor. Checking the remote-tracking tip first when local is an ancestor of remote avoids rescuing an older version of a newly filed item.
