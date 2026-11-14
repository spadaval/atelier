---
schema: atelier.review
schema_version: 3
round: 4
commit: 38c5d6b4f387f7cd03744a818b78ff7f208359db
reviewers:
- 01a1d8df-36dc-73a0-b8fe-5057f998fc7f
model: grok-4.6
verdict: approve
created_at: 2026-11-14T07:04:43.947996589Z
---

## Summary

Complexity: medium. Two of the three items have merged. What is left is
moving `forge.rs` into the core, routing the Desk's and `at_commit.rs`'s
Git and `gh` spawns through it, and rewriting this mission's Decision that
still waits on atelier-yclc.

Value: high. The crate merge is in, "the core" matches `atelier-core`, and
the remaining split is the one Sumeet asked about: Git and `gh` still have
several homes, and the Desk still cannot reach the forge code.

Risk of merging it: medium. The remaining item touches the CLI's forge and
the Desk's fetch and stop. The tests exist. A Git-only finish is now a
Result, not a failed criterion.

The value warrants finishing. The two holes at 3f0c166 are closed at
38c5d6b.

Checked the plan at 38c5d6b: mission.md, atelier-ni3d (done, with Report),
atelier-nl43 (done, with Report), atelier-x5al (ready), atelier-f3ax (the
amendment's new draft, outside the mission), and the two carried findings
from 3-k13f. Against that, read the workspace and crate manifests; grepped
`Command::new("git")` and `gh` spawns under `crates/`; read
`atelier-core/src/git.rs`, `records/at_commit.rs` (spawn at 376),
`atelier-cli/src/forge.rs` (`tempfile` at 312-313, `git::run` via the
core re-export) and `git.rs`, `atelier-desk/src/fetch.rs` and `stop.rs`;
grepped `atelier-yclc` in the tree; read `docs/design.md` crate table
(82-86) and Decisions (1698-1699), and every remaining yclc site
f3ax names. Did not read another round-4 review.

Could not check: whether `git mv` of `forge.rs` across crates gives the
use-line diff x5al asks for; whether wrapping the Desk's fetch and clone
in `git()` (which adds `--no-replace-objects` and always sets
`current_dir`) changes any test; the Rust and Desk test suites for x5al,
which is not implemented here.

## Finding 1: x5al's yclc correction names four lines and misses the rest
Recommendation: fix now
Was: 3-k13f.7
Now: fixed

x5al no longer rewrites the wait lines. It rewrites only this mission's
Decision at `docs/design.md:1698`, so the-rules-are-written still holds.
Every other `atelier-yclc` mention outside `.atelier/` is on atelier-f3ax:
`docs/design.md:802`, `docs/architecture/desk.md:610`, `:761` and `:938`
(kept as the reason for one `POST /api/action`, no longer resting on
yclc), `docs/product/desk.md:119` and `:171`, the comments in `stop.rs`,
`api.rs:160` / `MissionPage.ts`, and `Actions.tsx:82`, plus the two Desk
refusals, whose wording x5al now leaves alone. `git grep atelier-yclc`
after f3ax must find nothing outside `.atelier/`. A grep on this commit
matches that list and 1698, and nothing else.

## Finding 2: Stopping at the Git half leaves one-git unmet
Recommendation: fix now
Was: 3-k13f.8
Now: fixed

Authority and x5al's move line now name `tempfile` the same way as
`serde_json`. It is in `[workspace.dependencies]`; the core has it only as
a dev-dependency; `forge.rs:312` uses it. one-git (mission.md:83-87) and
the hatch (x5al item.md:33) now say a Git-only finish meets the `git` half
and not the `gh` half, and the Result says so.
