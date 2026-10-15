---
schema: atelier.review
schema_version: 1
id: atelier-b6eq
title: 'Re-reviewed f70bd814 and 56a5abb9. atelier-41yt resolved: the bare source now applies only when ''git --git-dir <common> rev-parse --is-bare-repository'' is true. Verified a separate-git-dir repo gives ''sepwork'' (directory name), hub/.bare plus hub/main gives ''hub'', and proj.git gives ''proj'', the last two labelled ''the bare repository''s name''. A bare repo named odd/.git gives ''odd''. No regressions to earlier fixes: a nested non-repo dir gives its own name, the top level uses origin, a linked non-bare worktree uses the main worktree''s name, a submodule without origin and a plain dir use the directory name, and a rerun with existing config prints no slug. cargo build is clean; repository_names and test_init_slug_comes_from_origin_then_the_main_worktree (now covering separate-git-dir and hub/.bare) pass; fresh evidence atelier-a9os (cargo nextest run --workspace, exit 0) is at f70bd814. One no_action note on separate-git-dir plus linked worktree with no origin.'
target: atelier-5qcs
identity: Independent reviewer (Claude subagent)
commit: 56a5abb90e09f8c9710ab502b4ed448e69faacdc
created_at: 2026-10-15T23:25:03.677985489Z
event:
  event: approval
---

Re-reviewed f70bd814 and 56a5abb9. atelier-41yt resolved: the bare source now applies only when 'git --git-dir <common> rev-parse --is-bare-repository' is true. Verified a separate-git-dir repo gives 'sepwork' (directory name), hub/.bare plus hub/main gives 'hub', and proj.git gives 'proj', the last two labelled 'the bare repository's name'. A bare repo named odd/.git gives 'odd'. No regressions to earlier fixes: a nested non-repo dir gives its own name, the top level uses origin, a linked non-bare worktree uses the main worktree's name, a submodule without origin and a plain dir use the directory name, and a rerun with existing config prints no slug. cargo build is clean; repository_names and test_init_slug_comes_from_origin_then_the_main_worktree (now covering separate-git-dir and hub/.bare) pass; fresh evidence atelier-a9os (cargo nextest run --workspace, exit 0) is at f70bd814. One no_action note on separate-git-dir plus linked worktree with no origin.
