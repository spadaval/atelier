---
schema: atelier.review
schema_version: 3
round: 7
commit: f28f082a123fa34c7de57f1274eee28f8e67309b
reviewers:
- thread:delegated-task:command%3Amcp%3Ad12574ac-fe2d-4335-bf72-c30aa232c7f3%3Adelegate-task%3A92cf485f-6332-485a-9106-bc6b4ed402a5
model: gpt-6-astra
verdict: approve
created_at: 2026-11-17T21:19:56.782806932Z
---

## Summary

Approve the plan at f28f082a123fa34c7de57f1274eee28f8e67309b. The carried
isolation finding is fixed; no new fix-now finding.

Complexity: high. The mission adds a build, React mounting and copied T3
components, rewrites the host integration, moves three pages, and adds a
Linux test instance with provider and fixture setup.

Value: high. The native pages and mission briefing address the stated
problems in the extension in daily use. The test command lets five items
verify their work without waiting on the Windows machine each time.

Risk of merging it: medium. The implementation affects extension loading,
navigation and the host running the daily T3 server. Separate instance
state, per-item checks, a Windows check before release, and retention of
the Desk pages until installation make failures visible and reversible.

The value warrants the work. Restyling the framed pages would not deliver
the requested native components or typed modules. The Linux command has a
concrete verification purpose and does not replace the Windows check.

Read the mission and all nine items, the amendment against master, and the
change at the reviewed commit. Checked dependency order, authority,
release and removal gates against the loader's deskReadUrl, the release
script, run-tests.mjs, test-contract.mjs, fixture-desk.mjs, Desk page and
API code, history types and parser, and the architecture documentation.
Read the mockup markup and the build spike's copying and style-verification
scripts on spike/extension-react. Compared the revised isolation criteria
with T3's OpenCode data-directory resolver and desktop smoke launcher.

`atelier check` validates all 197 items, 13 missions and config.toml;
`git diff --check` passes. The check also reports existing empty Why
sections on three unrelated missions. No implementation tests, browser
visual comparison or live T3 session were run for this plan review. I did
not reproduce the free-model experiment or the in-app OpenCode setup.
The latter remains explicitly untried and must be proved by atelier-diru,
which names the required thread, draft, sidebar state and escalation path.

## Finding 1: diru's isolation check and spike recipe still miss the OpenCode folders T3 reads
Recommendation: fix now
Was: 6-ok78.4
Now: fixed

atelier-diru/item.md line 29 now requires HOME, T3CODE_HOME,
XDG_CONFIG_HOME, XDG_DATA_HOME and XDG_CACHE_HOME under the instance's
own folders. It extends the before/after check to the real
~/.config/opencode and ~/.local/share/opencode as well as ~/.t3, and
requires querying the instance's OpenCode credentials and finding none.
Line 30 retains the prohibition on linking or copying personal folders
and the refusal of a non-free model. Context at line 45 names the same
HOME and XDG requirements, so the implementation recipe no longer omits
the folders identified by the finding.

These requirements cover the XDG_DATA_HOME/home fallback in T3's
packages/provider-opencode/src/server/usage.ts and the XDG isolation used
by scripts/smoke-desktop-app.ts. This is a correction to the plan, not
proof of a running launcher; the implementation must still demonstrate
the stated isolation and credential checks.
