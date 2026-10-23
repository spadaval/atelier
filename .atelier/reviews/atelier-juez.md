---
schema: atelier.review
schema_version: 1
id: atelier-juez
title: 'low: /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s6.sh (outside the branch). s6.sh runs in the atelier repository and calls ''git merge-tree --write-tree'' on each catch-up merge, which writes tree and blob objects into that repository''s object store, so it is not strictly confined to TMPDIR. Impact: unreachable objects only, removed by gc; no refs, index or working tree change. The other scripts guard with lib.sh''s TMPDIR and PWD checks; s4 and s5 reruns left both checkouts clean. No action for this branch; if the scripts are reused, point GIT_OBJECT_DIRECTORY at TMPDIR with the repo''s objects as an alternate.'
target: atelier-prjr
identity: Independent reviewer (Claude subagent)
commit: 050d55dd9d3f746766f97939103ffeda6b789efc
created_at: 2026-10-23T00:10:12.377202149Z
event:
  event: finding
  recommendation: no_action
---

low: /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s6.sh (outside the branch). s6.sh runs in the atelier repository and calls 'git merge-tree --write-tree' on each catch-up merge, which writes tree and blob objects into that repository's object store, so it is not strictly confined to TMPDIR. Impact: unreachable objects only, removed by gc; no refs, index or working tree change. The other scripts guard with lib.sh's TMPDIR and PWD checks; s4 and s5 reruns left both checkouts clean. No action for this branch; if the scripts are reused, point GIT_OBJECT_DIRECTORY at TMPDIR with the repo's objects as an alternate.
