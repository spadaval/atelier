---
schema: atelier.review
schema_version: 1
id: atelier-ub58
title: finding on atelier-4pzj @8889e8b5
target: atelier-4pzj
identity: claude-reviewer-4pzj
commit: 8889e8b5c0798c42541dcb49ade0b8eb513e83c8
created_at: 2026-11-04T23:04:59.797375905Z
event:
  event: finding
  recommendation: fix_now
---

medium: 4pzj S4 is stricter than the items it names, so it can stay unmet after they are done. 'Ready work lists exactly the items start would accept, net of items in flight on other branches' contradicts atelier-mfyd (in-flight rows stay listed with marks; only the ready count and Next Action are netted; 'Classification does not change'), and 'with one readiness gate' rules out atelier-b17k's allowed outcome of keeping the gate with a comment and test. Fix: reword S4, e.g. 'Ready work offers only items start would accept, or marks the rest; ready counts and Next Actions exclude items in flight elsewhere; work ready has one readiness gate or a recorded reason for the second'.
