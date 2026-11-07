---
schema: atelier.review
schema_version: 2
id: atelier-h7y2
title: 'crates/atelier-app/src/git_freshness.rs:504-516 plain_author: the fast path treats an unset i18n.logOutputEncoding as UTF-8, but git falls back to i18n.commitEncoding, and a failing ''git config'' read is also taken as UTF-8. With i18n.commitEncoding=latin1 and a UTF-8 commit that has no encoding header, ''git show'' would re-encode a non-ASCII name and the old String::from_utf8 would fail, while the fast path returns the correct UTF-8 name. The only effect is that an error becomes a correct answer. If exact parity matters, also check i18n.commitEncoding and the config exit status.'
target: atelier-0349
identity: Independent reviewer (Claude subagent)
commit: 6d0300f6202777ceeb5a1b28f93efc8d5541dd00
created_at: 2026-10-16T02:50:22.318656891Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: 9e34b74562cb93834ae75bab527ff1b33c14b65d
    recorded_at: 2026-10-16T02:51:49.377649189Z
    reason: 'Agreed: the only behavioural difference is a former error case now returning the correct UTF-8 name.'
    migrated_from: atelier-qzvl
---

crates/atelier-app/src/git_freshness.rs:504-516 plain_author: the fast path treats an unset i18n.logOutputEncoding as UTF-8, but git falls back to i18n.commitEncoding, and a failing 'git config' read is also taken as UTF-8. With i18n.commitEncoding=latin1 and a UTF-8 commit that has no encoding header, 'git show' would re-encode a non-ASCII name and the old String::from_utf8 would fail, while the fast path returns the correct UTF-8 name. The only effect is that an error becomes a correct answer. If exact parity matters, also check i18n.commitEncoding and the config exit status.
