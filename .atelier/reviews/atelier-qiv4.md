---
schema: atelier.review
schema_version: 2
id: atelier-qiv4
title: approval on atelier-8bj8 @5cf579c4
target: atelier-8bj8
identity: reviewer-8bj8
commit: 5cf579c407d1090c215bb64d63ba9bf9b57608f3
created_at: 2026-11-08T06:14:08.459426563Z
event:
  event: approval
---

Read the whole branch d4887d29..5cf579c4: skill_link.rs, setup arg changes and dispatch in main.rs, 8 tests in cli_integration/skill_link.rs (cfg(unix); CI runs Linux and macOS), and the three docs/product changes. Each Done when line holds: link replaces symlinks and lock-listed copies and prints prior state; refuses a checkout without SKILL.md or an unlisted real dir with an mv hint, inspecting both paths before any change; drops only agent-factory from the lock (paths match skills 1.7.6 source: XDG_STATE_HOME/skills or ~/.agents, CLAUDE_CONFIG_DIR trimmed); unlink removes links and prints the reinstall line; setup --help says to link a checkout that stays on master, via a detached worktree. Confirmed in skills source that global update iterates only lock entries, so an untracked link survives. Tests can fail: the lock-equality and refusal assertions would break if removal or the untracked check were removed. Ran: cargo fmt --check, git diff --check, cargo metadata --locked all pass; cargo nextest run -j 4 -p atelier-cli 808 passed; scripts/merge-ci-passed.sh says HEAD is not a merge commit (runs inside atelier merge only). Manual scratch-HOME runs: default checkout resolves to the worktree root; findings atelier-pak1 (defer) and atelier-w6dz (no_action). Did not rerun npx skills 1.7.6; relied on the Report's transcript plus the source read.
