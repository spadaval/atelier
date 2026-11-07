---
schema: atelier.review
schema_version: 2
id: atelier-d7lz
title: 'crates/atelier-app/src/evidence_freshness.rs:227-228: stale_from_checkout uses proof_matches(issue, record, false), which is the retained-claim check (edges pin..claim_endpoint, and claim_endpoint returns the pin when the terminal claim precedes it, so the edge list is empty), not ''fresh minus the working tree'' (evidence_matches_edges over pin..HEAD without code_clean). For a current terminal claim whose pass was recorded after the claim, any committed code change after the pass yields stale_from_checkout=true. Reproduced in a scratch worktree: Fixture + epic_child, terminal child, fail then pass committed, then commit a change to ''code''; ''atelier land <owner>'' with a CLEAN checkout reports ''the later pass ... was recorded before the uncommitted changes in this checkout'' (no uncommitted changes exist), and after writing an unrelated untracked file it also prints ''Uncommitted changes make passing proof stale: unrelated.'' -- the exact misattribution the issue set out to remove, and the both-reasons case gets checkout wording. Fix: compute from_checkout as fresh && edges(pin, HEAD) match (evidence_matches_edges) && !code_clean, and add a test for a current claim with a committed post-pass change plus an unrelated dirty file. Other checks fine: serde(skip) on a Serialize-only struct leaves merge --check --json unchanged; the (0,1) tuple comparison matches the old ordering; the OR on merge is correct; the new retained-claim test does exercise its scenario.'
target: atelier-x3bj
identity: Independent reviewer (Claude subagent)
commit: 3cb639795058e20eee1e6a51fd626d95cd621e8a
created_at: 2026-10-16T20:21:24.922017232Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 409a44549696c2d99627a1b2c48e94761455bf73
    recorded_at: 2026-10-16T20:28:39.101630040Z
    resolution_commit: cd50ea4e1aa7054d4111e659643f6cbe0f563d81
    reason: 'Resolved: from_checkout is now fresh && matches_commits_to_head (evidence_matches_edges over pin..HEAD, no code_clean) && !code_clean, replacing the vacuous retained-claim proof_matches(.., false). Dirty only -> true; later commit only -> false; both -> false. The new test a_current_claim_whose_pass_a_later_commit_staled_does_not_blame_the_checkout replays the d7lz reproduction (clean and unrelated-dirty) and fails when the old expression is restored (scratch copy: clean case prints ''recorded before the uncommitted changes in this checkout''); it passes on the fix.'
    migrated_from: atelier-b50e
---

crates/atelier-app/src/evidence_freshness.rs:227-228: stale_from_checkout uses proof_matches(issue, record, false), which is the retained-claim check (edges pin..claim_endpoint, and claim_endpoint returns the pin when the terminal claim precedes it, so the edge list is empty), not 'fresh minus the working tree' (evidence_matches_edges over pin..HEAD without code_clean). For a current terminal claim whose pass was recorded after the claim, any committed code change after the pass yields stale_from_checkout=true. Reproduced in a scratch worktree: Fixture + epic_child, terminal child, fail then pass committed, then commit a change to 'code'; 'atelier land <owner>' with a CLEAN checkout reports 'the later pass ... was recorded before the uncommitted changes in this checkout' (no uncommitted changes exist), and after writing an unrelated untracked file it also prints 'Uncommitted changes make passing proof stale: unrelated.' -- the exact misattribution the issue set out to remove, and the both-reasons case gets checkout wording. Fix: compute from_checkout as fresh && edges(pin, HEAD) match (evidence_matches_edges) && !code_clean, and add a test for a current claim with a committed post-pass change plus an unrelated dirty file. Other checks fine: serde(skip) on a Serialize-only struct leaves merge --check --json unchanged; the (0,1) tuple comparison matches the old ordering; the OR on merge is correct; the new retained-claim test does exercise its scenario.
