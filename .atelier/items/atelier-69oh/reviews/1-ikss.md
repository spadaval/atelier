---
schema: atelier.review
schema_version: 3
round: 1
commit: 34e83c2f4f0782e489d0e99d46afefe83b6632cc
reviewers:
- thread:delegated-task:command%3Amcp%3A7db7e6f0-ebaf-4b57-8ab1-e107ca867579%3Adelegate-task%3A689696ce-1b9c-4436-af0f-de04fff53237
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T03:12:35.698537893Z
---

## Summary

Complexity: low. The change adds one startup note, shares its opening with
the existing note, and tests which note each agent receives.

Value: high. The lead evals need Codex to know how to start a fresh thread
before they can measure how it uses the lead's authority.

Risk of merging it: low. It changes eval instructions and documentation,
with no product behavior or stored-data change, and is easy to revert.

The value is worth the small change. No smaller change would provide the
missing startup instructions and verify their delivery.

I reviewed `git diff master...34e83c2f4`, the surrounding thread setup and
command construction, all four Done when lines, and the Report. The new
Codex note explains `spawn_agent`, `fork_turns: "none"`, and `wait_agent`
without instructing the agent when or why to delegate. Claude Code still
receives its original note; mixed setups still use `delegate`. The tests
cover these paths and verify that Codex receives the new note as a developer
message through `codex debug prompt-input`.

`python3 -m unittest test_run -v` passed all 75 tests, including the three
real Codex prompt-rendering checks. `git diff --check master...34e83c2f4`
passed. The binary's reported build commit b9fd7f223 has the same `crates/`
tree as the reviewed commit.

I read both saved runs under `~/.cache/af-evals/runs/atelier-69oh`, including
their results, final replies, and transcripts, plus the saved child session.
They support the Report: `lead-looks-trivial` starts no child and asks for
authorization; `lead-stalled-item` uses one child for all three items,
reviews them as lead, and closes the mission after their merges. Its saved
facts show all three done on main and no currency defect. The failed merge
by the lead and all three judge answers also match the Report. The retained
first run's prompt contains both the new note and Codex's delegation
restriction. The README and Report accurately disclose that limitation
and do not claim the two runs establish a frequency or remove it.

I did not rerun the model-driven scenarios or independently validate the
older November 11 and atelier-aq82 runs. No findings.
