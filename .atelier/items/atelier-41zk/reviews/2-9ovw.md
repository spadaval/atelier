---
schema: atelier.review
schema_version: 3
round: 2
commit: ca6b61965d6045c8e81e11b24d6650aa47f25f2e
reviewers:
- thread:delegated-task:command%3Amcp%3A03454c8e-b27f-4061-997e-27a4558850ca%3Adelegate-task%3A55729716-391d-40ab-a55d-7db2c2208763
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T05:19:23.697676696Z
---

## Summary

Complexity: low. The change is one README and four screenshots, with no new runtime behavior.
Value: high. The requested introduction now explains the tool, the skill and the Desk without requiring the reader to open the specification first.
Risk of merging it: low. Documentation errors show when followed and are cheap to undo.
The value warrants the size; no smaller change would cover the requested features and examples.

Read the full README and the changes since round 1 against each Done when line. Compared the descriptions with the product specifications, the skill, extension and eval documentation, and the setup, push-guard, log and CI-selection implementations. Resolved all 32 relative link and image targets, inspected all six referenced PNGs, and ran `git diff --check`.

With installed `atelier 0.7.0`, independently replayed the two-round item loop in a scratch repository using alice, bob and carol identities, through changes requested, approval and merge. Checked the log before and after merge. Also replayed Claude and Codex bootstrap against local bare remotes, and submitted and merged a record-only backlog branch without review. The Claude recipe works; the Codex alternative fails at its staging command. Did not recapture the live Desk or repeat the author's GitHub rendering check. CI had a pending test job when inspected, with the evals, Desk and extension jobs skipped.

## Finding 1: Publish the harness settings before the base-branch push guard takes effect
Recommendation: fix now
Was: 1-68ip.1
Response: fixed in a53a0ff2a
Now: fixed

The reordered Claude recipe commits and pushes both the config and harness settings before installing the Git hooks. Replayed it against a local bare remote: the push succeeds, then `status` and `check` succeed. The separate Codex staging error is Finding 5.

## Finding 2: Explain that ordinary record-only changes require no review
Recommendation: fix now
Was: 1-68ip.2
Response: fixed in a53a0ff2a
Now: fixed

README.md:145-157 now describes all three branch kinds and their merge requirements, including record-only validation without review. Independently submitted and merged a draft-item backlog branch without a review, matching the table.

## Finding 3: Mark the missing merge line in the log transcript
Recommendation: fix now
Was: 1-68ip.3
Response: fixed in a53a0ff2a
Now: withdrawn

The rebuttal holds. On a base with no earlier item or mission, my two-round 0.7.0 reproduction prints seven log actions before merge and the same seven afterward, with no merge event. The path-limited Git walk rewrites that merge to one parent, as the separately filed atelier-3sym explains. The previous finding's assertion that this run must print a merge line was wrong. README.md:86-109 now explicitly shows the log before merge, so no final event is cut or hidden.

## Finding 4: State which changes actually run the eval fixtures in CI
Recommendation: fix now
Was: 1-68ip.4
Response: fixed in a53a0ff2a
Now: fixed

README.md:336-339 now scopes the fixture checks to binary, skill and eval changes, matching the eval README and `needs_evals` in scripts/ci-jobs.sh.

## Finding 5: Stage the Codex hooks when choosing the Codex setup alternative
Recommendation: fix now

README.md:405 offers `atelier setup codex` as an alternative, but line 406 always stages `.claude`. On a repository without Claude settings, following that alternative creates `.codex/hooks.json`, then `git add .atelier .claude` exits 128 with `fatal: pathspec '.claude' did not match any files`; the chained commit and push never run. If `.claude` already exists, the new Codex hooks remain unstaged instead. Reproduced with 0.7.0 in a fresh repository and a local bare remote.

Make the staging path follow the selected harness, for example by showing a Codex-specific staging command alongside the Claude command. Both advertised setup paths must publish their generated hooks before `atelier setup` installs the base-branch push guard.
