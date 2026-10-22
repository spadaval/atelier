---
schema: atelier.review
schema_version: 1
id: atelier-83w3
title: 'low: .atelier/issues/atelier-as7i.md decision note, crates/atelier-app/src/review_gate.rs:1084: the stated reason is wrong. With --diff-merges=first-parent a merge that takes the path unchanged from its second parent is NOT simplified away: in a scratch repo (git 2.53) ''git log --root --reverse --diff-merges=first-parent --no-patch --diff-filter=A -- r.md'' lists both the side commit and the clean merge, where the old query listed only the side commit. Behaviour stays correct because callers use only lines().next() and --reverse puts an ancestor before any merge that descends from it, so the true introducer still wins; but the extra merge entries are real and the note/comment claim otherwise. Fix: correct the decision note (and rely on ''first line = oldest'' explicitly in a comment) so a later caller that counts or takes the last line is not misled.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: 2c839460af8fe7d9b811edfc3da3c62945008b69
created_at: 2026-10-22T20:55:37.479525024Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/atelier-as7i.md decision note, crates/atelier-app/src/review_gate.rs:1084: the stated reason is wrong. With --diff-merges=first-parent a merge that takes the path unchanged from its second parent is NOT simplified away: in a scratch repo (git 2.53) 'git log --root --reverse --diff-merges=first-parent --no-patch --diff-filter=A -- r.md' lists both the side commit and the clean merge, where the old query listed only the side commit. Behaviour stays correct because callers use only lines().next() and --reverse puts an ancestor before any merge that descends from it, so the true introducer still wins; but the extra merge entries are real and the note/comment claim otherwise. Fix: correct the decision note (and rely on 'first line = oldest' explicitly in a comment) so a later caller that counts or takes the last line is not misled.
