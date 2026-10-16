---
schema: atelier.review
schema_version: 1
id: atelier-vxc9
title: 'Re-reviewed c2f02115 and a2af4438 against findings atelier-240w, atelier-doea and atelier-uslt; all three are resolved. Mutation checks: with the base-has-policy refusal (Some(_) if install) disabled, an_adopted_repositorys_branch_cannot_pass_as_an_install now fails, and removing the new ''rerun atelier land without --install'' recovery text also fails it. The earlier owner, base and flag-required mutations were already caught. The 7 install-filtered tests pass and cargo fmt --check is clean. The docs now list land --install in the sessions-and-hooks synopsis and in cli-surface, describe the Stop hook on installing branches, and the tracker_changes_since comment is accurate. One wording nit, not blocking: ''blocks once ... then warns'' describes a single Stop cycle; each later turn''s Stop blocks again.'
target: atelier-bibd
identity: Independent reviewer (Claude subagent)
commit: a2af4438b39af10b5b1e08d022e740ad1dbc09b6
created_at: 2026-10-16T23:29:26.892873041Z
event:
  event: approval
---

Re-reviewed c2f02115 and a2af4438 against findings atelier-240w, atelier-doea and atelier-uslt; all three are resolved. Mutation checks: with the base-has-policy refusal (Some(_) if install) disabled, an_adopted_repositorys_branch_cannot_pass_as_an_install now fails, and removing the new 'rerun atelier land without --install' recovery text also fails it. The earlier owner, base and flag-required mutations were already caught. The 7 install-filtered tests pass and cargo fmt --check is clean. The docs now list land --install in the sessions-and-hooks synopsis and in cli-surface, describe the Stop hook on installing branches, and the tracker_changes_since comment is accurate. One wording nit, not blocking: 'blocks once ... then warns' describes a single Stop cycle; each later turn's Stop blocks again.
