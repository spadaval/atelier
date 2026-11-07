---
schema: atelier.review
schema_version: 2
id: atelier-uv6a
title: 'G15: valid root-born reviews make land fail on Git 2.30.2. The append_only_history root fallback invokes ls-tree --format=%(objectmode), which this older supported Git rejects. Independent fixture /tmp/session-review-git230-root-review-am1wxf1i/repo has a root-born finding pinned to a real ancestor via an unrelated-history merge; exact 5b5dca62c10736c4a13b19f56d350f71e8cadd96 passes land on Git 2.47/2.39 and fails only on 2.30 with unknown --format. Parse ordinary NUL-framed ls-tree mode headers while preserving rejection of non-regular root review modes and newline paths. This predates snapshot consolidation; previous older-Git proof did not cover root introductions.'
target: atelier-1ioo
identity: session-reviewer
commit: 5b5dca62c10736c4a13b19f56d350f71e8cadd96
created_at: 2026-10-02T01:33:20.898343809Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: session-reviewer
    commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    recorded_at: 2026-10-02T02:02:59.633249948Z
    resolution_commit: 84395a9d3ba6d28550023c3234fb166cc6cd5211
    reason: 'G15 independently verified at root composition 84395a9d: actual Git 2.30.2 accepts retained legitimate root-born review. Independent root executable and symlink introduction fixtures refuse even with current regular files and log.showRoot=false. Ordinary NUL-delimited ls-tree mode headers preserve root introduction semantics; root production guard files equal reviewed 808ee9b2. Full-hooks proof 554/554, zero skipped.'
    migrated_from: atelier-bme9
---

G15: valid root-born reviews make land fail on Git 2.30.2. The append_only_history root fallback invokes ls-tree --format=%(objectmode), which this older supported Git rejects. Independent fixture /tmp/session-review-git230-root-review-am1wxf1i/repo has a root-born finding pinned to a real ancestor via an unrelated-history merge; exact 5b5dca62c10736c4a13b19f56d350f71e8cadd96 passes land on Git 2.47/2.39 and fails only on 2.30 with unknown --format. Parse ordinary NUL-framed ls-tree mode headers while preserving rejection of non-regular root review modes and newline paths. This predates snapshot consolidation; previous older-Git proof did not cover root introductions.
