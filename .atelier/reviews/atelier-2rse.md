---
schema: atelier.review
schema_version: 1
id: atelier-2rse
title: 'crates/atelier-cli/src/commands/git.rs:349-353 (base_done_claim): a parent whose record does not parse never counts as a prior status, and a commit whose own record does not parse is skipped (line 334). So two unpushed base commits get around the gate: commit A rewrites an open item''s record so it no longer parses (e.g. schema: "bogus" with status: done), then commit B restores a valid record with status done. A is skipped and B''s parent is Some(None), so land passes with ''Landing checks passed'' (reproduced with a probe test in a scratch copy). This needs a broken intermediate record, so it is unlikely by accident. A possible fix: when the parent does not parse, compare with the nearest ancestor (or the upstream) where the record parses, or with its status at the upstream.'
target: atelier-fj1t
identity: Independent reviewer (Claude subagent)
commit: 943c7ee83e3c3e9a1d43a3bb50280b263bf25ef1
created_at: 2026-10-16T18:48:25.624751408Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/git.rs:349-353 (base_done_claim): a parent whose record does not parse never counts as a prior status, and a commit whose own record does not parse is skipped (line 334). So two unpushed base commits get around the gate: commit A rewrites an open item's record so it no longer parses (e.g. schema: "bogus" with status: done), then commit B restores a valid record with status done. A is skipped and B's parent is Some(None), so land passes with 'Landing checks passed' (reproduced with a probe test in a scratch copy). This needs a broken intermediate record, so it is unlikely by accident. A possible fix: when the parent does not parse, compare with the nearest ancestor (or the upstream) where the record parses, or with its status at the upstream.
