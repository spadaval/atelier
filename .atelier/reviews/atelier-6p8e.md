---
schema: atelier.review
schema_version: 1
id: atelier-6p8e
title: 'Re-reviewed 511a2384 (fix for atelier-apxi) with the HEAD binary in scratch repo /tmp/rv4n7v2. A parent named by the client_ref of an update entry whose existing record is already a child is now refused in preview (''would nest a parent under a parent''), matching the id form. Valid cases still apply: a parent named by an update entry''s client_ref for a record without a parent, and restating the current parent through an update entry''s client_ref. atelier check passes afterwards. The check compares other.id, which fill_update_from_record has already resolved to the full id, so suffix ids are covered. All earlier fixes are unchanged: done-body refusal, per-record lint and strategy-file checks, duplicate update ids, refusal to move or nest parents, and the client_ref mapping output. HEAD evidence records the suite run at 511a2384.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: 8923d6a58f8edca096dc360f1da3c7af1b055710
created_at: 2026-10-16T00:49:31.388104901Z
event:
  event: approval
---

Re-reviewed 511a2384 (fix for atelier-apxi) with the HEAD binary in scratch repo /tmp/rv4n7v2. A parent named by the client_ref of an update entry whose existing record is already a child is now refused in preview ('would nest a parent under a parent'), matching the id form. Valid cases still apply: a parent named by an update entry's client_ref for a record without a parent, and restating the current parent through an update entry's client_ref. atelier check passes afterwards. The check compares other.id, which fill_update_from_record has already resolved to the full id, so suffix ids are covered. All earlier fixes are unchanged: done-body refusal, per-record lint and strategy-file checks, duplicate update ids, refusal to move or nest parents, and the client_ref mapping output. HEAD evidence records the suite run at 511a2384.
