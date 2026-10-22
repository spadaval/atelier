---
schema: atelier.review
schema_version: 1
id: atelier-n9en
title: 'low: .atelier/issues/*.md (backfill b41a458a) Compatibility: once merged, master''s records carry discovered_from on 56 issues, so the installed atelier (which rejects the unconfigured link type) fails every command on master, including the Claude Code hooks (status, land --hook), until reinstalled; without the backfill only newly created links would break it. Acceptable as a reinstall-after-merge step: the new binary reads all older records (follow_up_ids was never written), a simulated merge into master in a scratch clone passes target/debug/atelier check and land with an approval, and the repo already reinstalls after policy changes. Sequence it: merge with the worktree''s target/debug/atelier, then ''cargo install --path crates/atelier-cli'' immediately, before land, push or other lanes'' sessions.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: eb6ee5cf688f70c5521050bad0eac7d1d58da725
created_at: 2026-10-22T21:45:01.108199932Z
event:
  event: finding
  recommendation: no_action
---

low: .atelier/issues/*.md (backfill b41a458a) Compatibility: once merged, master's records carry discovered_from on 56 issues, so the installed atelier (which rejects the unconfigured link type) fails every command on master, including the Claude Code hooks (status, land --hook), until reinstalled; without the backfill only newly created links would break it. Acceptable as a reinstall-after-merge step: the new binary reads all older records (follow_up_ids was never written), a simulated merge into master in a scratch clone passes target/debug/atelier check and land with an approval, and the repo already reinstalls after policy changes. Sequence it: merge with the worktree's target/debug/atelier, then 'cargo install --path crates/atelier-cli' immediately, before land, push or other lanes' sessions.
