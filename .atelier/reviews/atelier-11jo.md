---
schema: atelier.review
schema_version: 1
id: atelier-11jo
title: 'low: crates/atelier-cli/src/commands/merge.rs:73-90 sweep() opens <dir>/lock with create(true), and any read_dir or remove_dir_all error aborts the merge. A live merge in another worktree that is finishing (MergedCheckout Drop: TempDir removes the directory, including its lock file, while the creation lock is not held) can race the sweep: the sweep recreates the lock file in a half-deleted dir, takes it, and then its remove_dir_all may hit NotFound, which fails an otherwise good merge with ''could not remove leftover''. Separately, a merge run by an older installed binary has no lock file, so a new binary sweeps its live checkout mid-check. Both are narrow and transient (rerun succeeds; the old-binary case ends after reinstall). Fix: treat NotFound from remove_dir_all as removed, and consider skipping or warning instead of failing the merge when a leftover cannot be removed.'
target: atelier-c3kh
identity: Independent reviewer (Claude subagent)
commit: f5733347e44e5cbc88ce25b5680ea8e91d92ddfc
created_at: 2026-10-22T20:52:27.977959389Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/merge.rs:73-90 sweep() opens <dir>/lock with create(true), and any read_dir or remove_dir_all error aborts the merge. A live merge in another worktree that is finishing (MergedCheckout Drop: TempDir removes the directory, including its lock file, while the creation lock is not held) can race the sweep: the sweep recreates the lock file in a half-deleted dir, takes it, and then its remove_dir_all may hit NotFound, which fails an otherwise good merge with 'could not remove leftover'. Separately, a merge run by an older installed binary has no lock file, so a new binary sweeps its live checkout mid-check. Both are narrow and transient (rerun succeeds; the old-binary case ends after reinstall). Fix: treat NotFound from remove_dir_all as removed, and consider skipping or warning instead of failing the merge when a leftover cannot be removed.
