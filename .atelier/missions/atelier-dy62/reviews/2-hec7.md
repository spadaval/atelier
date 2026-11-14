---
schema: atelier.review
schema_version: 3
round: 2
commit: e3cfd2a467e794be46eaf4747f0ee26dc9f033b9
reviewers:
- 01a1d8ab-0968-7f02-8df8-a576fde1a79f
model: grok-4.6
verdict: approve
created_at: 2026-11-14T06:07:45.232502677Z
---

## Summary

The round-1 answers at e3cfd2a close the six carried findings. The criteria as written can be ticked after the items, yclc writes to the crate it will meet, and dropping nl43 no longer breaks a frozen module name.

Checked the plan at e3cfd2a against c22b137: mission.md, atelier-ni3d, atelier-nl43, atelier-x5al, the reworded atelier-yclc and atelier-hj6b, and atelier-o91z (named in round 1, left as it is). Against that, read the workspace Cargo.toml and each library crate's Cargo.toml; counted non-test lines (core 639, records 1761, app 5995); grepped `Command::new("git")` and gh spawns under `crates/`; read `atelier-app/src/git.rs`, `atelier-records/src/at_commit.rs`, `atelier-desk/src/fetch.rs`, `stop.rs` and `build.rs`; confirmed the cited test lines (`atelier-records/tests/records.rs:649`, `load.rs:65`, `atelier-app/tests/acts.rs:1253`, `atelier-desk/tests/dependencies.rs:8-13` and `acts.rs:344-360`, `desk/e2e/acts.spec.ts:125-138`); read `docs/design.md` crate table (78-93) and Decisions (1629-1694); and opened atelier-8b0a, atelier-qb4f, atelier-0019 and `docs/product/desk.md:119`. Did not run the test suite, merge the gardener branches, or read another round-2 review.

- Complexity: medium. The crate merge is a move of about 8,400 non-test lines and their tests, then git/gh call sites and a rename, sequenced through atelier-yclc of atelier-to83.
- Value: high. Agents already guess the wrong crate (gardener run atelier-qb4f), "the core" in the documents is not `atelier-core`, and the Desk cannot reach helpers that sit in the crate above `atelier-records`.
- Risk of merging it: medium. Behaviour is meant to stay the same and the tests exist. The round-1 holes in the success criteria, the yclc crate path, and the nl43 drop path are closed.

Could not check: whether `git mv` across crate directories produces the `git diff -M` shape ni3d asks for; how atelier-qb4f's crate-table edit will conflict with ni3d's three-row table; whether folding the Desk's `GIT_TERMINAL_PROMPT=0` fetch into the git module stays behaviour-preserving.

## Finding 1: Compare behavior against each item's base, allowing the required open-mission stop feature
Recommendation: fix now
Was: 1-atz2.1
Now: fixed

nothing-changed (mission.md:73-80) now judges each of the three items against its own base, and says what atelier-yclc changes between them is that item's own. x5al still waits on yclc (item.md:10-12), so its "tests pass unchanged" (item.md:33) is after the open-mission stop. The cited assertions (`crates/atelier-desk/tests/acts.rs:344-360`, `desk/e2e/acts.spec.ts:125-138`) still refuse that stop today, which is yclc's change.

## Finding 2: Allow mechanical crate-path changes inside test bodies
Recommendation: fix now
Was: 1-atz2.2
Now: fixed

ni3d (item.md:31) and nothing-changed (mission.md:74-78) now allow crate and module paths wherever they occur in a test file, and name `atelier_records::put_first` in a test's body as the example. The cited sites still exist (`crates/atelier-records/tests/records.rs:649` and `:658`, `load.rs:65`, `crates/atelier-desk/tests/acts.rs:367`).

## Finding 3: one-git still fails after x5al because the Desk build script spawns git
Recommendation: fix now
Was: 1-kh04.1
Now: fixed

Why (mission.md:38-40) now counts `build.rs` and leaves it. one-git (mission.md:83-85) and x5al Done when (item.md:29) except build scripts. x5al Out (item.md:40) names `crates/atelier-desk/build.rs` and says it cannot call the crate it builds for. `build.rs:10` still spawns `git` for `ATELIER_DESK_BUILT_FROM`.

## Finding 4: nothing-changed and ni3d's test-diff rule forbid edits the crate merge must make
Recommendation: fix now
Was: 1-kh04.2
Now: fixed

nothing-changed (mission.md:74-78) and ni3d (item.md:31) now allow moved fixture paths and the crate list in `crates/atelier-desk/tests/dependencies.rs`. The cited sites still exist: `TRACKER` at dependencies.rs:8-13, and `crates/atelier-app/tests/acts.rs:1253-1254` joining `../atelier-records/tests/fixtures/open-missions`.

## Finding 5: atelier-yclc still tells its implementer to write atelier-app after ni3d deletes it
Recommendation: fix now
Was: 1-kh04.3
Now: fixed

yclc Done when (item.md:33) and In (item.md:41) now name `atelier-core`, and its Context says it waits on ni3d. hj6b Done when (item.md:26) names the core as `atelier-app`, or `atelier-core` once ni3d has merged. o91z does not name `atelier-app` as a write target; its body readers are `next/crates/atelier-records/src/body.rs`.

## Finding 6: dropping nl43 leaves Authority's frozen module name `actions` unmet
Recommendation: fix now
Was: 1-kh04.4
Now: fixed

Authority (mission.md:122-130) freezes `model`, `records`, `git` and `forge`, not `actions`. Outcome (mission.md:50-51) says the module is `acts` until nl43 renames it `actions`. If nl43 is dropped the module stays `acts`, one-word is not met, and the Result says so.
