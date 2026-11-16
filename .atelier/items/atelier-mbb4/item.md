---
schema: atelier.item
schema_version: 3
id: atelier-mbb4
title: 'Release 0.7.0: the Desk inside T3 Code, and what has merged since 0.6.0'
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A9c525daa-e7f7-44f4-9529-6989140fcc76%3Adelegate-task%3A6c2a19c7-6107-40fc-8c1b-e6650e24e1b2
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T16:34:09.395442100Z
---

## Goal

Publish a release that holds the Desk's T3 Code views. Sumeet asked on
2026-11-16 for a release now that atelier-61gh's four items, atelier-jzaw
and atelier-cdxg have merged; v0.6.0 was tagged on 2026-11-14 before any of
them. This item sets the version to 0.7.0 and writes its changelog; once it
merges, the thread that started it tags `v0.7.0` on the merge commit and the
release workflow builds it.

## Done when

- [x] The workspace version in `Cargo.toml` is 0.7.0, and `Cargo.lock` agrees.
- [x] `CHANGELOG.md` has a 0.7.0 entry, written from `git log
  v0.6.0..origin/master --first-parent`, that says what changed for agents
  using `atelier`, what changed in the Desk, and what a host must do when it
  updates. Each claim matches the merged change it describes.
- [x] The entry says whether a 0.6 repository's records, hooks or
  configuration need anything, and says so prominently if they do.
- [x] Nothing else in the tree states the current version as 0.6.0; history
  in records and the design record is left as written.

## Scope

The version, the lockfile and the changelog, plus any line elsewhere that
names the current version. Tagging and the release build happen after the
merge, outside the branch.

## Report

The version is 0.7.0 in `Cargo.toml` and in `atelier-cli` and
`atelier-core` in `Cargo.lock` (`cargo update -w --offline`). `atelier-desk`
stays 0.0.0: it is not part of the release (`dist = false`), as in 0.6.0. A
build from this branch prints `atelier 0.7.0`.

Minor, not patch: the changes since v0.6.0 add the Desk's embedded mode, its
host contract, four views and eleven reads, and a new planner practice in the
skill. Only atelier-t83m is a fix.

Nothing breaks a 0.6 checkout or record. I compared the record and config
types (`model/config.rs`, `item.rs`, `mission.rs`, `review.rs`) and the
`records/` parser at v0.6.0 and on the base: they differ only in `use` paths
and comments. The hooks still call the binary. The command line lost and
gained no command or flag; only help text and the status-change refusal say
"action" for "act" (atelier-nl43's Report lists the lines).

The entry was written from the 24 first-parent commits since `v0.6.0`:
items atelier-ni3d, nl43, x5al, t83m, b344, vg1l, prx5, fnm5, knx8, jzaw and
cdxg, plus record-only merges (mission atelier-dy62's plan reviews and
close, mission atelier-61gh's plan and close, the t83m filing). Checked
against the merged code:

- The route list against the Desk server's router (`/api/action`, `/embed`,
  `/records/{id}`, `/ties`, `/thread`, `/sidebar`, `/launch`, `/history`,
  `/roadmap`, `/checks`; `/api/act` is gone).
- `ActionRequest` has one variant, `Stop`, of a proposed plan, so a stale
  app can only fail at that.
- The messages against `desk/src/lib/host.ts` and "The host contract" in
  `docs/architecture/desk.md`. atelier-jzaw's and knx8's page choice was
  replaced by atelier-cdxg's `/t`, so the entry describes only `/t`.
- The catch-up rule against `docs/design.md`'s hook section and
  atelier-t83m's Report.
- The host step: v0.6.0's Desk `Config` is `deny_unknown_fields`, so an
  old Desk refuses `embed_origins`; `install.sh` keeps an existing
  configuration and writes the line only into a new one.

No other line in the tree names 0.6.0 as the current version;
`scripts/cutover/README.md` says "0.5.0 or later", which still holds.

Checks: `cargo fmt -- --check`, `cargo metadata --locked`, `git diff
--check` and `atelier check` pass.
