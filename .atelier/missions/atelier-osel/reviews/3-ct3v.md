---
schema: atelier.review
schema_version: 3
round: 3
commit: a374930344618ca1e024c2c45af85a2da2adadcf
reviewers:
- thread:delegated-task:command%3Amcp%3A909059fb-c6b7-42f1-873e-abaee1bead8d%3Adelegate-task%3A9dc0f4b5-f2b3-4c49-869e-1c84796b24e8
model: gpt-6-astra
verdict: approve
created_at: 2026-11-17T19:48:29.955943793Z
---

## Summary

Complexity: high. Eight items change the build and release pipeline, introduce copied T3 components and shadow-root rendering, rewrite the extension, and move three views before removing their Desk copies.

Value: high. The native pages and mission briefing directly implement the human's stated direction, and the typed modules support their maintenance.

Risk of merging it: medium. Implementation can break the installed extension and its navigation, but failures are visible, changes are reversible, and removing the old pages waits for installation of the replacement release.

The value warrants the work. A smaller styling change would not deliver the requested native views and typed codebase. The sequence gives the first native page its build and loader permissions before the larger rewrite, and preserves the framed pages through the cutover.

Reviewed the plan at `a374930344618ca1e024c2c45af85a2da2adadcf` in my own detached checkout. I read the mission, all eight items, earlier findings and the mockup HTML on `spike/extension-react`. I checked the plan against the current Desk views and API types, the loader's read gate and injection contract, the frame tests, release and installer scripts, CI configuration, history reader and architecture documentation. Both carried findings are fixed; I found no new findings. The earlier deferred fixture mismatch is also resolved: `needs-you-first` now refers to the fixtures collectively.

`atelier check` passes record validation and `git diff --check` passes. The check also reports the existing empty Why sections on unrelated missions atelier-4pzj, atelier-icv4 and atelier-uf97, already noted in round 2. I did not render the mockups, rerun the build spike, run application suites or verify Legend's T3 instance. This approval covers the plan; its component-style comparisons, page fixtures and real-app verification remain implementation acceptance criteria.

## Finding 1: 9l8h and wvst leave test-extension.mjs asserting the framed This thread and project home
Recommendation: fix now
Was: 2-h00o.9
Now: fixed

`atelier-9l8h/item.md:31` now converts the panel bootstrap, host:thread and frame-follow assertions into native This thread checks, retaining framed record-link checks. Its line 27 preserves a chosen framed page across T3 thread switches. `atelier-wvst/item.md:29` converts both breadcrumb and more-missions navigation checks to the native project home without an iframe. These assignments cover the assertions identified in the finding.

## Finding 2: Specify how the retro handles merged PRs the Desk cannot return
Recommendation: fix now
Was: 2-zm4k.9
Now: fixed

`atelier-ut89/item.md:32` now keeps every merged item in the review-ranked list, explicitly marks a missing pull-request URL and opens the item's framed Desk page instead. Its line 38 requires a merged item with null pull_request in the done-mission fixture. The mission's retro criterion at lines 98–101 matches that fallback, and Scope explicitly excludes historical PR discovery. This settles the data limitation without requiring an unassigned server change.
