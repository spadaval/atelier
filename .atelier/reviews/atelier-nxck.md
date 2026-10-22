---
schema: atelier.review
schema_version: 1
id: atelier-nxck
title: 'Reviewed eae0d4b4 (atelier-0cj6/mlnb fixes), base merge 699b0732 and records through a8819c8a. R1 resolved: both prior cases are in a_close_after_approval_is_covered_by_merge_and_land_alike under both strategies; scratch variants under both strategies (base work by Reviewer merged after approval, side branch also on base, Reviewer-authored branch work as control) give the same verdict from merge and land; a side branch not on the base fails only review.current while land passes, confirming the asymmetry is one-directional. Merge 699b0732: remerge-diff shows only conflict-marker removal; claims.reviewed stays and master''s moved-base retry follows it before any output; every non-record file''s changes from both sides survive line-for-line; work-model.md keeps both sentences; 7b6e adds discovered_from atelier-83w3. merge --check on a clone with master at 8836e9c6: every gate passes except findings.disposed, review.current and claims.reviewed, which await this approval; merged-tree shell checks pass. Full suite 861/861 with test-hooks, cargo fmt --check and git diff --check clean.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: a8819c8a0cfbbc44f43ff284cd4cac857ef70549
created_at: 2026-10-22T22:03:44.643217202Z
event:
  event: approval
---

Reviewed eae0d4b4 (atelier-0cj6/mlnb fixes), base merge 699b0732 and records through a8819c8a. R1 resolved: both prior cases are in a_close_after_approval_is_covered_by_merge_and_land_alike under both strategies; scratch variants under both strategies (base work by Reviewer merged after approval, side branch also on base, Reviewer-authored branch work as control) give the same verdict from merge and land; a side branch not on the base fails only review.current while land passes, confirming the asymmetry is one-directional. Merge 699b0732: remerge-diff shows only conflict-marker removal; claims.reviewed stays and master's moved-base retry follows it before any output; every non-record file's changes from both sides survive line-for-line; work-model.md keeps both sentences; 7b6e adds discovered_from atelier-83w3. merge --check on a clone with master at 8836e9c6: every gate passes except findings.disposed, review.current and claims.reviewed, which await this approval; merged-tree shell checks pass. Full suite 861/861 with test-hooks, cargo fmt --check and git diff --check clean.
