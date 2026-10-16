---
schema: atelier.review
schema_version: 1
id: atelier-6hpy
title: 'git_freshness.rs:738-757 clean_merge runs git merge-tree with the repo''s full config and attributes (working-tree .gitattributes, .git/info/attributes, merge.<name>.driver, merge.renames). Verified with git 2.53: with ''* merge=evil'' in attributes and merge.evil.driver=''echo INJECTED > %A'', merge-tree --write-tree exits 0 and writes content found on neither side. So a merge made under a configured driver counts as clean. Examples are a lockfile-regenerating driver, or merge.ours.driver=true for CHANGELOG, which silently drops the base''s change. The approval stays current, and land''s check_base_commits (commands/git.rs:245) skips the merge entirely, so the driver output reaches the base unreviewed. Out of scope for malicious authors per the strategy, but honest repos do configure such drivers. Suggested fix: run merge-tree isolated from attributes and drivers, for example a temporary GIT_DIR using the repo objects as alternates with empty config, or treat any merge attribute other than the default as unclean. Also mention this in reviews-and-merge.md.'
target: atelier-n7g1
identity: Independent reviewer (Claude subagent)
commit: 1f9c88ded59b5f0bcd19473fb6e9e9c7753f45ca
created_at: 2026-10-16T18:31:51.177116456Z
event:
  event: finding
  recommendation: defer
---

git_freshness.rs:738-757 clean_merge runs git merge-tree with the repo's full config and attributes (working-tree .gitattributes, .git/info/attributes, merge.<name>.driver, merge.renames). Verified with git 2.53: with '* merge=evil' in attributes and merge.evil.driver='echo INJECTED > %A', merge-tree --write-tree exits 0 and writes content found on neither side. So a merge made under a configured driver counts as clean. Examples are a lockfile-regenerating driver, or merge.ours.driver=true for CHANGELOG, which silently drops the base's change. The approval stays current, and land's check_base_commits (commands/git.rs:245) skips the merge entirely, so the driver output reaches the base unreviewed. Out of scope for malicious authors per the strategy, but honest repos do configure such drivers. Suggested fix: run merge-tree isolated from attributes and drivers, for example a temporary GIT_DIR using the repo objects as alternates with empty config, or treat any merge attribute other than the default as unclean. Also mention this in reviews-and-merge.md.
