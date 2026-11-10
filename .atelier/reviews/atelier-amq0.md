---
schema: atelier.review
schema_version: 2
id: atelier-amq0
title: finding on atelier-syyj @6b5dd8e6
target: atelier-syyj
identity: gpt-6.1-sol
commit: 6b5dd8e69230fb8a2571741c17919e872843784b
created_at: 2026-11-10T18:07:43.314198045Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/merge.rs:138 Validate movement after the merge checks and before pushing or opening a pull request. The only moved() call is at line 176, after the forge writes, and pending/failed CI returns before reaching it. Reproduced offline against the pinned head using the test FAKE_GH and a local bare origin: hold the configured merge check, commit a source change on the work branch, then release the check. With no CI workflow, merge pushes the captured old head and opens PR #7 before exiting 1 claiming it wrote nothing. With ci.yml pending, it pushes and opens the PR, exits 3 and asks to wait for CI on the obsolete head instead of reporting that the branch moved. The local base stays unchanged in both runs, but the remote branch and PR were created. This violates the Done when moving-input requirement and Scope confirmation before any write. Check movement before the first forge write, retain the final confirmation before advancing the base, and cover moving inputs with a forge as well as without one. Update the Merge section ordering accordingly. Verdict: merge after the blocking findings are fixed and independently reviewed; no approval of this head. Sizes: complexity high (snapshots, temporary worktrees, locking and forge integration), value high (completes approved-to-done), risk high (writes shared base state and must preserve review gates). Contract and Fitness fail on the reproduced gaps. Every added moving part otherwise traces to the stated requirements. Fresh proof: HEAD-parent and whole-range whitespace checks; root and next formatting and offline locked metadata; next clippy --offline -j 4 --all-targets -- -D warnings; nextest --offline --build-jobs 4 -j 4, 102 passed and none skipped. The prior head-snapshot and status findings are fixed by inspection and their regressions pass. All cargo builds used this worktrees next/target, and every test used the requested TMPDIR. No code was edited. Scratch repositories were removed. Open questions: none. Residual risk: real GitHub and network behavior were deliberately not exercised.
