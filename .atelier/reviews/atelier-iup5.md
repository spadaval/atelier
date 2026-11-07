---
schema: atelier.review
schema_version: 2
id: atelier-iup5
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:14.981013508Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/main.rs:291,283-284,280,170,692 and docs/product/workflow-configuration.md:33,42,44: CLI help and docs still use the five words outside CONTEXT.md's meanings, against Done when line 4 ('CLI help and docs use the words that way'): root help's merge row 'Check merge gates, then integrate' (cli-surface.md now says 'Evaluate'); land --push 'when the checks pass'; land --install 'it lands through the repository's usual review'; review request's long help 'With no ID it reviews the checked-out branch' (it prints a brief); the evidence record example summary 'checked claim X'; workflow-configuration.md 'Land it in one of two supported ways' and twice 'review the generated policy', which also misquotes init's new 'Read the generated policy'. Fix: 'Evaluate the merge gates, then merge ...', 'when its gates pass', 'merges through', 'prints the brief for the checked-out branch', 'verified claim X', 'Commit it in one of two ways', 'read the generated policy'.
