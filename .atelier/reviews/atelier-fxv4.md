---
schema: atelier.review
schema_version: 2
id: atelier-fxv4
title: approval on atelier-7tul @69c75c98
target: atelier-7tul
identity: codex-reviewer (gpt-6.1-sol)
commit: 69c75c984cca7cee4a669f1035b1d40875eb4988
created_at: 2026-11-09T03:44:30.142272752Z
event:
  event: approval
---

Verdict: merge. The optional GitHub forge and typed CI gate remove redundant catch-up runs while keeping local shell checks on the merged tree. Commit 69c75c98 fixes both findings from atelier-ppqm without introducing another fix_now.
Sizes: complexity medium (provider, policy and shared validator context); value high (required forge-delivery increment and two reproduced contract fixes); risk medium (merge and handoff gates used across repositories).
Contract: pass. All five Done when scenarios are covered by passing integration tests. ci.passed judges captured base/head pins during merge and the actual branch HEAD during completed-branch land; valid slash-containing Git remote names resolve through Git before URL parsing.
Fitness: pass. Both merge paths pass MergeHeads captured before making the synthetic checkout. merge --check shares the same path. The only None context that runs merge_checks is evaluate_session on a completed work branch, where local base versus HEAD is appropriate. The transition and completion-claim callers pass None but policy parsing rejects ci.passed outside merge_checks, so they cannot misread a synthetic merge. Light-path land passes no checks context and does not run merge_checks.

Findings
- No new findings or recommended FIX NOW.
- Earlier R1 / atelier-oqrm (medium, FIX NOW) is fixed: explicit heads replace the inference from HEAD^2. The new completed-branch land regression passes both before and after a record-only base sync merge, and refuses untested branch code in both cases.
- Earlier R2 / atelier-kldl (low, FIX NOW) is fixed: Github::open first resolves forge.remote with git remote get-url --. The new upstream/github test passes and checks that the API reads the other/repo remote, not origin. This approval of the later head resolves both findings.

Proof
Independently read atelier issue show atelier-7tul --full, atelier review request atelier-7tul, atelier-ppqm, atelier-oqrm and atelier-kldl, ADR 0031, CI on a forge, the forge-delivery strategy, the whole branch's code/doc/record changes and git show 69c75c98. Traced every ValidatorRequest, review_gate, light_path and evaluate_without_issue caller.
Fresh commands on 69c75c984cca7cee4a669f1035b1d40875eb4988, with CARGO_TARGET_DIR=/home/supadava/.cache/atelier-target/7tul-review, TMPDIR=/home/supadava/.cache/atelier-tmp/7tul-review and -j 4 for tests; read the full output:
- cargo nextest run -j 4 -p atelier-cli --test forge_ci --test merge_gates: 106/106 passed, including both new regressions.
- The same command with --features test-hooks: 108/108 passed.
- cargo nextest run -j 4 -p atelier-app -p atelier-workflow: 150/150 passed, including URL parsing and rejection of ci.passed in transitions.
- cargo nextest run -j 4 -p atelier-cli --test light_path: 11/11 passed.
- cargo fmt -- --check; git diff --check HEAD^ HEAD -- . ':!.atelier'; git diff --check master..HEAD; cargo metadata --locked --format-version 1: passed.
- The branch binary's atelier check passed with existing Done when warnings.
- The branch binary's atelier merge atelier-7tul --check exercised the retained base-policy scripts/merge-ci-passed.sh in the synthetic merge checkout. It refused on the stale planning approval, findings awaiting this approval, and no successful CI run. Read PR 62: head is 69c75c98; CI run 42557761727 was in progress on Linux and macOS. CI and the script gate remain separate from this review approval.

Open questions
None.

Residual risk and tests not run
Full atelier-cli suites and cross-repository smoke were not independently rerun. The two regressions were run on the fixed head; their claimed failures on the old code were not independently rerun in this review. The changed paths, no-forge merge compatibility and failure hooks are covered by the targeted suites above. PR CI and the retained base-policy script must pass before merge; this review does not claim they passed. As ADR 0031 explicitly accepts, a base that gains code can leave the combined tree untested until base CI runs. Merge with the reviewed branch binary because the installed binary cannot parse the new policy.

Changes and review records
Only this review approval record, pinned to 69c75c984cca7cee4a669f1035b1d40875eb4988, by codex-reviewer (gpt-6.1-sol). No code changes, merge or push.
