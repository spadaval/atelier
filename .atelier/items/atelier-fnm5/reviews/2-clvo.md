---
schema: atelier.review
schema_version: 3
round: 2
commit: f2c071a9416774c63f17bcd0ffa9ffad60e67e0a
reviewers:
- thread:delegated-task:command%3Amcp%3Aab64755b-460f-4ed4-a13c-cfbeb58f9e82%3Adelegate-task%3A49d1e34b-2a91-4300-9f94-ae750e2a64b0
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T03:19:44.086914739Z
---

## Summary

- Complexity: high. This adds a contract between the Desk and its embedding host, connection state and theme mapping.
- Value: high. Host theming and opening threads in the existing window are required for the mission's embedded Desk.
- Risk of merging: medium. Failures affect the embedded interface and host integration, but the configured parent and origin checks limit who can send data, and messages add no write action.

The value warrants the scope; I found no substantially smaller change that delivers the same result. The carried fix-now finding is fixed, and I found no new blocking defect.

Reviewed submitted commit `f2c071a9416774c63f17bcd0ffa9ffad60e67e0a` in my own detached worktree, including the full diff against `master` and the changes since round 1. Checked each Done when line against the configuration and API, parent source and origin checks, navigation reports, every `ThreadTitle` caller, theme mapping, documentation and interface tests. The CLI refused to open the review in a detached checkout and directed me to the item's branch checkout; only review and verdict records were written there.

Independent checks passed: `pnpm test` (51), `pnpm build`, `DESK_E2E_PORT=4895 pnpm e2e --workers=2` (90), `cargo nextest run -j 2 -p atelier-desk` (44, reusing the item's target directory), `cargo fmt -- --check` and `git diff --check master...HEAD`. The browser tests verify the modern-colour scheme transitions, rejection of nested `currentColor`, listed and unlisted hosts, an empty origin list, navigation messages, and a thread click sending exactly one message without opening or navigating a page.

The deferred round-1 finding `1-52g0.2` is also fixed: validation now rejects `currentColor` anywhere in an expression, and the browser regression confirms that the prior background remains.

I did not validate the real desktop app's custom-scheme origin or extension. The installed service still has no `embed_origins` entry. As the Report explains, its existing binary rejects that key, so adding it and restarting must accompany the new binary's installation after merge. That rollout remains to be done; the installer supplies the entry for new configurations and preserves existing ones.

## Finding 1: Infer the scheme for all accepted CSS colour formats
Recommendation: fix now
Was: 1-52g0.1
Response: fixed in 763f68fd2: `scheme()` reads the background's sRGB bytes from a 1-pixel canvas, so `oklch()`, `color()` and `color-mix()` set the scheme; `desk/e2e/host.spec.ts` checks `color-scheme` flips light then dark with those forms.
Now: fixed

`scheme()` now paints the accepted background onto a canvas and reads its sRGB pixel before calculating luminance, removing the restriction to computed `rgb()` strings. The interface test passes for dark-to-light `oklch()` and light-to-dark `color(srgb ...)` backgrounds without an explicit scheme, so both the light/dark palette and native control scheme follow the new background.
