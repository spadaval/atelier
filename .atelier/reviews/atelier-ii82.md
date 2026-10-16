---
schema: atelier.review
schema_version: 1
id: atelier-ii82
title: 'Re-reviewed 254420f3 (parent-reference gaps) on top of the earlier fixes, with the HEAD binary in scratch repo /tmp/rv4n7v2. A record that already has children is refused a parent in preview, with the one-level message. Restating the current parent through an update entry''s client_ref, or through an id suffix, passes; apply writes only updated_at, and the edge is deduplicated. Creating a child under a parent named by suffix works. Nesting under an existing child named by suffix is refused in preview. A move expressed through an update entry''s client_ref is refused with the issue update --parent pointer. The earlier fixes still hold: issue_body_done, per-record lint and strategy-file checks, duplicate update ids, and the client_ref mapping output. One fail-closed preview/apply gap remains (a parent given as the client_ref of an update entry for an existing child) and is filed as a defer finding; apply rejects it without changing anything. HEAD evidence atelier-d1r1 records the suite run.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a14de2e608c00dd108b1f4f397432cfb0ac441c1
created_at: 2026-10-16T00:47:52.950589280Z
event:
  event: approval
---

Re-reviewed 254420f3 (parent-reference gaps) on top of the earlier fixes, with the HEAD binary in scratch repo /tmp/rv4n7v2. A record that already has children is refused a parent in preview, with the one-level message. Restating the current parent through an update entry's client_ref, or through an id suffix, passes; apply writes only updated_at, and the edge is deduplicated. Creating a child under a parent named by suffix works. Nesting under an existing child named by suffix is refused in preview. A move expressed through an update entry's client_ref is refused with the issue update --parent pointer. The earlier fixes still hold: issue_body_done, per-record lint and strategy-file checks, duplicate update ids, and the client_ref mapping output. One fail-closed preview/apply gap remains (a parent given as the client_ref of an update entry for an existing child) and is filed as a defer finding; apply rejects it without changing anything. HEAD evidence atelier-d1r1 records the suite run.
