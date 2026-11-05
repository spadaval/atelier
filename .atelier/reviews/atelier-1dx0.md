---
schema: atelier.review
schema_version: 1
id: atelier-1dx0
title: approval on atelier-cej5 @a24719e5
target: atelier-cej5
identity: claude-review-cej5
commit: a24719e5068fd78cd750a17b8793fa0860622efc
created_at: 2026-11-04T23:23:48.769457149Z
event:
  event: approval
---

Read the path_log.rs diff: the combined date-order test is now matches_git_when_dates_run_backwards (step -60) and matches_git_when_dates_tie (step 0). Each builds one branchy repo and runs one assert_parity, through a shared helper. Repo::step drives the commit clock, so the two tests differ. .config/nextest.toml is unchanged (60s default). CI runs 41144966277 and 40725124384 confirm the diagnosis: the old test passed at 48.6s/50s+ in the plain macOS pass and hit TIMEOUT at 60.0s in the test-hooks pass. Locally, cargo nextest run -p atelier-app path_log: 9/9 pass, each split test 3.1s, same as matches_git_over_merges_octopus_and_criss_cross. Also ran git diff --check, cargo fmt --check and cargo metadata --locked; all clean. PR 18 CI run 41931339897 at a24719e5: success on ubuntu and macos. In the macOS log, plain pass: backwards 23.8s, tie 23.6s, octopus 24.4s. test-hooks pass: backwards 19.4s, tie 19.3s, octopus 18.5s. merge-ci-passed.sh only runs inside a merge checkout. I checked its conditions by hand: master has no code change since fork f5047b87, and CI passed on the head.
