---
schema: atelier.review
schema_version: 2
id: atelier-eghd
title: approval on atelier-1sbw @4fc22542
target: atelier-1sbw
identity: gpt-6.1-sol-r5
commit: 4fc225427e2decd5debf6992159e48667cc531de
created_at: 2026-11-10T23:43:41.170405739Z
event:
  event: approval
---

Verdict: merge. A bounded, one-off converter is needed to rehearse and perform the five-repository cutover; keeping it outside the new binary avoids an old-layout compatibility layer.
Sizes: complexity medium (record/config mapping, relationship reconstruction and moved-document links in one script); value high (required for the cutover); risk medium (replaces versioned records and moves strategies, with byte comparison and new-format validation before integration).
Contract: pass against all eleven current Done when lines. The nearest-ancestor mapping and the narrowed link requirement have recorded orchestrator decisions. I accept the disposition of atelier-lbfe, atelier-4ril and atelier-rc3i: their unsupported forms are outside this bounded conversion and do not occur in the surveyed input.
Fitness: pass. The parser, graph, document splitting, path rewriting and fresh-commit verification each serve an explicit requirement. No added moving part lacks one. No new fix_now finding.

Proof rerun at 4fc225427e2decd5debf6992159e48667cc531de:
- Read the branch-own changes since 939a5410, the complete converter and its tests, the requirement edits and dispositions, and the new record validator and transition specification.
- TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -v scripts/cutover: all 14 tests pass, including the prebuilt binary test. Malformed input refuses without conversion; repeated conversion is byte-identical; verification detects changed, missing, added, ignored and restored old files.
- git diff --check HEAD^ HEAD -- . ':!.atelier', cargo fmt -- --check, and cargo metadata --locked --format-version 1 > /dev/null: all pass. Cargo commands ran sequentially, after checking the process count, with CARGO_BUILD_JOBS=4; these two cargo subcommands do not accept -j. No build was run.
- Fresh git clones of all five repositories were made under /home/supadava/.cache/atelier-fcb7/1sbw-review-r5/, all remotes removed before repository commands, and each checked out at the old commit used by the recorded proof. Conversion made no commit and changed no tracked file outside .atelier/ except the moved strategies. --verify passes on each; repeating conversion from the same old commit produces identical bytes. The supplied next/target/debug/atelier check, status and list each exit 0, with check reporting every record valid; these reads leave the converted bytes unchanged. All five new clones were removed; original repositories and earlier scratch clones were untouched.

Counts reproduced (repository: missions, items, decision items included in items; closed left; reviews/evidence left; strategies moved):
- atelier: 9, 172, 2; 374; 1397/819; 6. Convert 0.200 s; check 0.078 s; status 0.030 s; list 0.009 s.
- astromech: 4, 240, 2; 403; 2855/801; 3. Convert 0.219 s; check 0.202 s; status 0.038 s; list 0.011 s.
- demiurge: 1, 19, 2; 135; 410/105; 1. Convert 0.073 s; check 0.034 s; status 0.023 s; list 0.010 s.
- life-on-rails: 3, 47, 2; 56; 224/173; 3. Convert 0.133 s; check 0.059 s; status 0.021 s; list 0.011 s.
- cosplay-studio: 7, 37, 1; 54; 304/126; 1. Convert 0.064 s; check 0.039 s; status 0.021 s; list 0.010 s.

An independent CommonMark parser (markdown-it-py, not the converter's LINK expression) finds the same counts of working relative links before and after: 6, 3, 0, 0, 4. The 8 and 16 already broken destinations in the first two repositories remain unchanged; there is no newly broken destination. Three additional invented-fixture probes cover bare and angle-bracket reference definitions with titles inline or on the following line: eight rendered links resolve in the split mission/design documents in each probe, including copied definitions.

Findings: none new. Earlier rounds 1-3 fix_now findings are covered by passing regressions and this approval. Round 4 findings retain the orchestrator's wont_fix decisions.
Open questions: none blocking.
Residual risk and tests not run: link parsing deliberately does not support all Markdown forms. Repeat link resolution as well as --verify on the actual cutover inputs, since verification proves repeatability rather than independent semantic correctness. The external link checker must be supplied by the cutover runbook as the Report already states. I did not rerun the author's 40 mutation experiments or the unrelated full Rust suite. The code and Report are unchanged by this review; only this review record is committed.
