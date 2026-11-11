# Notes of atelier-g6rp

What the old mission record held beyond its mission sections.

## Outcome

Agent Factory's only copy lives in this repository under `skills/agent-factory`, with its evals, and installs with Vercel's `skills` CLI. `atelier doctor` and `status` warn when the binary or skill is stale, `atelier setup --link` lets Sumeet edit the live skill from a checkout, the skill steps aside outside Atelier repositories, and spadaval/.agents loses the moved skill and the unused Codex skills, leaving Sumeet to decide whether to retire it (atelier-rfxf). The strategy is [.atelier/missions/atelier-g6rp/](.).

## Notes

- 2026-11-08 [decision] file-pr stays a personal skill in ~/.claude/skills, not part of Agent Factory.
  Reason: It is about PR prose for any repository, Agent Factory targets Atelier repositories only. Sumeet confirmed on 2026-11-08 to keep it separate for now. (An earlier reason, that its ready-PR rule conflicts with draft PRs here, was wrong: drafts gated nothing, and docs/no-draft-prs drops them from AGENTS.md.)
- 2026-11-08 [note] 2026-11-08: merged atelier-pmib (ADR 0029, PR #49), atelier-8bj8 (setup --link, PR #50) and atelier-45gr (freshness in doctor and status, PR #51); each reviewed independently and smoke-tested across 5 repositories where code changed. Follow-ups filed: atelier-67ay, atelier-muac, atelier-taxm. Next: atelier-dkul once atelier-l58v merges into spadaval/.agents.
- 2026-11-08 [note] 2026-11-08 progress. All 8 items are done: l58v's skill change merged into spadaval/.agents main, then dkul (PR #54), 9s9i (PR #55), nz6a (PR #56), bxs7, 6oxt and hrgx. Fresh checks: the published skill installs with npx skills add into a scratch HOME, identical to master, and doctor reports it current. The live skill on this host links ~/projects/atelier-skill. Still open: spadaval/.agents main (dfa5f3e, 19 commits ahead) is unpushed and waits for Sumeet (atelier-tee8, needs-human). Close the mission and walk the debrief once it is pushed. Narrowed on the way: l58v's rare-edge-chain eval is recorded inconclusive (atelier-pju0, now under atelier-jxce).
