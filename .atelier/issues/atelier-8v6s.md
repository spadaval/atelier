---
created_at: "2026-11-08T18:09:03.254408372+00:00"
id: "atelier-8v6s"
issue_type: "decision"
labels:
- "land"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mvzi"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T18:09:08.239974014+00:00"
status: "done"
title: "Publish what land passes: drop land's unpushed-base-commits warning"
updated_at: "2026-11-08T18:09:08.239974014+00:00"
---

## Context

Reverses the warning atelier-mvzi added (commit 69bd4929, 2026-10-22).

## Question

Should `atelier land` keep warning about unpushed base commits the current owner did not make (atelier-mvzi), or is passing land enough to push?

## Options

- Keep the warning as atelier-mvzi built it.
- Keep a neutral notice, without "confirm", skipped when there is no owner.
- Remove it: once work is merged and land passes, what is on the base may be published.

## Decision

Remove it. Whether a change should go out is decided at review and merge, not at push. Land already refuses unreviewed code on the base and allows record-only commits, so a base that passes land is fit to publish. atelier-mvzi's problem, one thread's push publishing another thread's records without it knowing, is about awareness, not harm: the other thread's later push is a no-op. A neutral notice was rejected too, since it still names commits an agent has no reason to act on.

Case (2026-11-08): in a Life on Rails session on `main` with no owner, where planning records go straight to `main`, the warning listed five commits that were all that session's own as "commits no owner made". Its "confirm they are meant to go out" read as a request for the human's sign-off, so the agent repeatedly asked Sumeet whether to push finished, landed work. Decided by the human on 2026-11-08: "I'm really struggling to think of any reason why I'd ever want us to not push to main. The changes were complete. By atelier rules, all code is reviewed and validated."

## Report

Land no longer warns about unpushed base commits; once work is merged and land passes, agents push. Reverses atelier-mvzi. Decided by the human on 2026-11-08.
