---
schema: atelier.review
schema_version: 2
id: atelier-ggic
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:15.484268176Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/adr/0028-simpler-lifecycle.md:368-370 and 616-621: decision 5 still says start's hidden --base is accepted 'until atelier-ml3r updates the guidance that still passes it', and the Consequences entry for atelier-nn67 lists dropping branch_base but not --base. This item updates the guidance yet leaves --base in place, and atelier-nn67's Done when owns dropping it, so once this merges the ADR names the wrong item for that work. Fix: name atelier-nn67 in decision 5 and add --base to the Consequences entry.
