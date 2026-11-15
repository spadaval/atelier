---
schema: atelier.review
schema_version: 3
round: 1
commit: 0c40b04062c60cf8d53fa345530bebe1932f1329
reviewers:
- thread:delegated-task:command%3Amcp%3Ab4fa2cab-3330-4c69-8257-673a9c07a7c0%3Adelegate-task%3A7dd94a18-4ac4-4379-baeb-b4c4a20455fb
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-15T06:24:34.935782638Z
---

## Summary

Complexity: medium. One new helper establishes merge-parent authority from
committed config and Git refs, with a separate release-compatible patch.
Value: high. This unblocks legitimate catch-up commits that the installed
v0.6.0 hook currently refuses.
Risk of merging it: medium. The exception affects lifecycle guards in other
repositories, but is limited to byte-identical imports from a verified base.
The value warrants this small implementation and its adversarial coverage.

Reviewed commit 0c40b04062c60cf8d53fa345530bebe1932f1329 against master,
including every Done when claim, the complete pre-commit path, Git and staged
record readers, hook fixtures, documentation, baseline evidence and compatible
patch. The exception preserves staged parsing and validation, uses HEAD's
config, refuses staged config changes, requires one full incoming commit ID
matching an explicitly named base ref, and compares complete record bytes.
Existing base-write, review, merge and pre-push checks remain in force.

Independently ran the complete atelier-cli test suite successfully (one
pre-existing performance test ignored), including the five new real-hook tests
and existing hook, review and merge scenarios. Reproduced the expected installed
v0.6.0 failure through the real pre-commit hook without a skip. Verified the
retained patch and binary hashes. Extracted pinned release
c121ced0037874ec7fc1e3f928c3e01cdee026f8 into a disposable directory, applied
the patch, and verified that every extracted source file matches the prepared
compatibility source. Its copied regression suite matches this reviewed head;
all five tests also passed when built and run in that compatibility source.
The helper and lifecycle exception in the compatible patch match the submitted
implementation. This review covers that exact compatible candidate as well.

Formatting and all-target Clippy with warnings denied passed for both source
trees. git diff --check and atelier check passed in the review checkout.
No fix-now or deferred findings. Consumer application and live forge CI were
not tested; consumer use remains a separate authorized action, and forge CI
remains a merge gate. No consumer state, global binary or shared hooks changed.
