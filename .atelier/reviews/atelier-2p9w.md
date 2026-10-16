---
schema: atelier.review
schema_version: 1
id: atelier-2p9w
title: 'crates/atelier-cli/src/commands/review.rs:24,93,109,129 and session.rs:50 load the graph with RecordIndex::load_structural, which does not run the new enrichment. With a link to a pruned issue, ''atelier review finding'' prints only ''Error: <id> has related reference to missing issue <id>'' (reproduced). That message has no commit, no recovery and no ''Next: run atelier check'' line. Meanwhile docs/product/retention-and-prune.md:28-31 says every command reports the commit and the restore. check, issue list and issue show do show the enriched message. Fix: apply explain_pruned_reference on the error path of load_structural/into_index, or narrow the doc claim.'
target: atelier-r7cd
identity: Independent reviewer (Claude subagent)
commit: 85972cb43f9213363053eabf17e9a29dd7569afa
created_at: 2026-10-16T01:37:08.826572685Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/review.rs:24,93,109,129 and session.rs:50 load the graph with RecordIndex::load_structural, which does not run the new enrichment. With a link to a pruned issue, 'atelier review finding' prints only 'Error: <id> has related reference to missing issue <id>' (reproduced). That message has no commit, no recovery and no 'Next: run atelier check' line. Meanwhile docs/product/retention-and-prune.md:28-31 says every command reports the commit and the restore. check, issue list and issue show do show the enriched message. Fix: apply explain_pruned_reference on the error path of load_structural/into_index, or narrow the doc claim.
