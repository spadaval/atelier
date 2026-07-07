---
created_at: "2026-07-06T19:19:11.996329261+00:00"
id: "atelier-neip"
evidence_type: "review"
captured_at: "2026-07-06T19:19:11.996319992+00:00"
agent_identity: "independent-publish-reviewer"
target:
  kind: "issue"
  id: "atelier-durs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-durs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "REQUEST CHANGES — final publish review of PR 51 at a563d7ce found two mission-blocking contract/documentation defects. (1) Required c0mp public-contract sequencing is absent: commit 60634f2b is not an ancestor of mission/atelier-durs, and the mission versions of docs/product/cli-surface.md plus command-audit/issue.md and work.md regress or omit the approved flat issue inventory, plural Mission Overview, and singular mission drill-down split. This contradicts atelier-36uv’s PASS claim; integrate/rebase onto the c0mp contract and resolve shared docs in favor of that contract before revalidation. (2) atelier-p0am’s stale-guidance outcome is not met: normative PRODUCT_INTENT.md still lists removed prime/issue-list---ready/doctor forms as representative commands, CONTEXT.md still assigns branch preparation to removed atelier start, and current quality guidance still tells workers to use retired direct issue-close/lint/doctor paths. Replace live guidance with issue transition/check/check --fix or explicitly classify genuinely historical transcripts. Code-focused scenarios passed 7/7; cargo fmt -- --check, git diff --check master...HEAD, and atelier check passed. cargo nextest run passed 713/713 with an isolated HOME; the ambient configured HOME reproduces one pre-existing environment-sensitive provider-secret assertion failure. Epic/mission evidence and terminal state were inspected, including atelier-4qzb, atelier-kt5s, atelier-0ui8, atelier-vgpr, atelier-36uv, atelier-6t9d, atelier-vu1i, and atelier-aw1h. No implementation changes were made."
updated_at: "2026-07-06T19:19:18.215868991+00:00"
---

REQUEST CHANGES — final publish review of PR 51 at a563d7ce found two mission-blocking contract/documentation defects. (1) Required c0mp public-contract sequencing is absent: commit 60634f2b is not an ancestor of mission/atelier-durs, and the mission versions of docs/product/cli-surface.md plus command-audit/issue.md and work.md regress or omit the approved flat issue inventory, plural Mission Overview, and singular mission drill-down split. This contradicts atelier-36uv’s PASS claim; integrate/rebase onto the c0mp contract and resolve shared docs in favor of that contract before revalidation. (2) atelier-p0am’s stale-guidance outcome is not met: normative PRODUCT_INTENT.md still lists removed prime/issue-list---ready/doctor forms as representative commands, CONTEXT.md still assigns branch preparation to removed atelier start, and current quality guidance still tells workers to use retired direct issue-close/lint/doctor paths. Replace live guidance with issue transition/check/check --fix or explicitly classify genuinely historical transcripts. Code-focused scenarios passed 7/7; cargo fmt -- --check, git diff --check master...HEAD, and atelier check passed. cargo nextest run passed 713/713 with an isolated HOME; the ambient configured HOME reproduces one pre-existing environment-sensitive provider-secret assertion failure. Epic/mission evidence and terminal state were inspected, including atelier-4qzb, atelier-kt5s, atelier-0ui8, atelier-vgpr, atelier-36uv, atelier-6t9d, atelier-vu1i, and atelier-aw1h. No implementation changes were made.
