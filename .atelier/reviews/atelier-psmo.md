---
schema: atelier.review
schema_version: 1
id: atelier-psmo
title: 'Reviewed git diff master...HEAD (1f84bf3e). base_done_claim now, only when a parent''s copy does not parse, runs one extra ''git rev-list <parent> --not <upstream> -- <path>'' (lazy via or_else, so no extra Git call on the common path) and takes the first listed commit whose copy parses or is absent, else the pushed copy. rev-list lists newest first; in linear history the order is topological, and history simplification follows only the TREESAME parent through merges, so the first parseable hit is the nearest earlier copy on that lineage; each merge parent is judged on its own walk. Only when a merge itself rewrites the record to an unparseable copy are both sides walked, interleaved by commit date; ''nearest'' is ambiguous there and either answer is defensible. When the parent is the upstream or its ancestor, the list is empty and the pushed copy decides, same as before. Ran both base_done_claim tests (pass). Mutation in a scratch copy: replacing earlier(parent) with had_status(upstream) fails the reopened_then_restored case while restored_unchanged still passes, so both sequences are covered. Workspace nextest evidence atelier-gafu (pass) is at b412a970, code unchanged since.'
target: atelier-cr0r
identity: Independent reviewer (Claude subagent)
commit: 16765fd7ed9c517fa7971ffea25080bcb4747b3c
created_at: 2026-10-16T20:54:24.441873621Z
event:
  event: approval
---

Reviewed git diff master...HEAD (1f84bf3e). base_done_claim now, only when a parent's copy does not parse, runs one extra 'git rev-list <parent> --not <upstream> -- <path>' (lazy via or_else, so no extra Git call on the common path) and takes the first listed commit whose copy parses or is absent, else the pushed copy. rev-list lists newest first; in linear history the order is topological, and history simplification follows only the TREESAME parent through merges, so the first parseable hit is the nearest earlier copy on that lineage; each merge parent is judged on its own walk. Only when a merge itself rewrites the record to an unparseable copy are both sides walked, interleaved by commit date; 'nearest' is ambiguous there and either answer is defensible. When the parent is the upstream or its ancestor, the list is empty and the pushed copy decides, same as before. Ran both base_done_claim tests (pass). Mutation in a scratch copy: replacing earlier(parent) with had_status(upstream) fails the reopened_then_restored case while restored_unchanged still passes, so both sequences are covered. Workspace nextest evidence atelier-gafu (pass) is at b412a970, code unchanged since.
