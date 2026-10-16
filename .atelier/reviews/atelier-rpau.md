---
schema: atelier.review
schema_version: 1
id: atelier-rpau
title: 'crates/atelier-app/src/record_index.rs:220-232: the lookup takes the latest commit that deleted the path and always says ''<id> was pruned in commit X''. Two cases are wrong. (a) Deleting the file by hand in a commit is also reported as ''pruned''. Reproduced: a commit titled ''hand delete'' was reported as pruned. (b) A record that was pruned, restored, edited and committed, then deleted only in the working tree (not committed), points at the old deletion commit. Running the suggested git show then silently restores the stale pre-prune version, losing the later committed edits (reproduced: title ''Lonely v2'' came back as ''Lonely''). Fix: if the path exists at HEAD (git cat-file -e HEAD:<path>), suggest ''git restore -- <path>''. Otherwise say ''was deleted in commit X'' rather than ''pruned''.'
target: atelier-r7cd
identity: Independent reviewer (Claude subagent)
commit: 85972cb43f9213363053eabf17e9a29dd7569afa
created_at: 2026-10-16T01:37:08.756844054Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_index.rs:220-232: the lookup takes the latest commit that deleted the path and always says '<id> was pruned in commit X'. Two cases are wrong. (a) Deleting the file by hand in a commit is also reported as 'pruned'. Reproduced: a commit titled 'hand delete' was reported as pruned. (b) A record that was pruned, restored, edited and committed, then deleted only in the working tree (not committed), points at the old deletion commit. Running the suggested git show then silently restores the stale pre-prune version, losing the later committed edits (reproduced: title 'Lonely v2' came back as 'Lonely'). Fix: if the path exists at HEAD (git cat-file -e HEAD:<path>), suggest 'git restore -- <path>'. Otherwise say 'was deleted in commit X' rather than 'pruned'.
