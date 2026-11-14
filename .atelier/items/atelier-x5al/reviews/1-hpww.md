---
schema: atelier.review
schema_version: 3
round: 1
commit: 9c2f73d62b24762145f9deb0f32a44a851ceb692
reviewers:
- thread:delegated-task:command%3Amcp%3A59b03aa4-f8c7-4cc4-8b18-3964f7284916%3Adelegate-task%3Ae81d106b-7d05-4a17-91dc-48a91369ff90
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T07:16:11.475847760Z
---

## Summary

Complexity: low. This moves existing code and routes its callers through two existing modules, with no new subsystem.

Value: medium. The CLI and Desk both use these paths, and shared command construction gives each program one home.

Risk of merging: low. The forge commands retain their arguments, environment and errors, and the Git routing is covered by the unchanged integration tests.

The value warrants the change. I found no problems requiring a change before merge.

Reviewed every changed source file and both documents against master at commit 9c2f73d62b24762145f9deb0f32a44a851ceb692 and checked each Done when requirement. The spawn searches reproduce the Report's results: Git is constructed only in core/git.rs, apart from the excluded Desk build script, and gh only in core/forge.rs. The move commit has only the stated import and comment-path changes; git log --follow reaches the original forge history. The later Desk additions preserve their former arguments, JSON fields, prompt setting and error wording. The core takes only existing workspace dependencies, and the CLI now imports its forge types there.

Checked the record reader's batch command, the Desk's working directories and caller-selected unattended settings, and the added --no-replace-objects flag. The record reader already passed the flag. Fetch and push ignore replacement objects, and the stop's local clone does not copy replacement refs; its checkout now also uses the same object view as the existing core reads and commit. The CLI command construction is unchanged. The documentation names the new module ownership and keeps submit and merge in the CLI pending a second caller.

Validation on this checkout, with TMPDIR outside the repository and Cargo jobs capped at four:

- cargo nextest run --workspace -j 4: 362 passed, 1 skipped.
- pnpm build, then DESK_E2E_PORT=4797 pnpm e2e --workers=4: 50 passed.
- cargo fmt -- --check, cargo clippy --all-targets -j 4 -- -D warnings, and git diff --check: passed.
- The CLI tests, Desk tests and interface tests have no changes in the branch diff.

The Desk interface tests use API fixtures; the Rust integration tests cover the server's stop and fetch paths with local repositories and a gh stand-in. I did not run actions against a live forge or test interactive credential prompts. PR #165's Rust CI job was still pending when checked; the local checks above passed.
