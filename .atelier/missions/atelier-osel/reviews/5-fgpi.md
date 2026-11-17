---
schema: atelier.review
schema_version: 3
round: 5
commit: 0f246df4feaf0a965c6f1f04f2665c9e42563dd4
reviewers:
- 01a1eb52-296c-78d1-9cf4-720c1691f304
model: grok-4.6
verdict: changes
created_at: 2026-11-17T21:03:52.777142397Z
---

## Summary

Checked the plan at 0f246df as a TypeScript and React rewrite of the T3
extension plus a native mission page, with this round's amendment moving
"seen in T3" from Legend onto a Linux test instance on this host
(atelier-diru).

Read the mission, all nine items, and the amendment diff against
b540c23ab. Checked those claims against `t3-extension/t3-custom.mjs`
(`deskReadUrl`), `test-contract.mjs`, `fixture-desk.mjs`, `custom.js`
(1,177 lines, 89 KB), the Desk pages `MissionControl.tsx`,
`ThisThread.tsx`, `ProjectHome.tsx` and `Glance.tsx`,
`docs/architecture/desk.md`, `docs/product/desk.md`, the Vite spike and
`mission-concepts.html` on `spike/extension-react`, and the Linux spike
at `~/t3-linux-spike` (AppImage `0.1.11-nightly.20261117.4838`,
`run3.log`, OpenCode config and share directories present on this host).

Did not start the Linux T3 instance, did not enable OpenCode inside it,
did not open the mockup in a browser, and did not read other review
files for this mission.

Complexity: high. A new build, a shadow-root component copy, three
native pages, a loader-gate change, and a second T3 Code process on this
host.

Value: high. This is how Sumeet reads Atelier in T3 Code today.

Risk of merging it: medium. A bad page shows and an install rolls it
back. The new test instance shares the human's OpenCode folders and
runs Chromium with the sandbox off on the machine that serves his daily
T3 Code.

The rewrite, the three pages, the additive loader gate, the mockup-to-
ut89 mapping, and the lead's Windows check before the release are
worth building. The amendment is not, until atelier-diru is a startable
command whose isolation matches its Out, and until the mission page is
not blocked on that command.

## Finding 1: atelier-ut89 must not wait on atelier-diru before it starts
Recommendation: fix now
atelier-ut89 lists `atelier-diru` under `blocked_by`
(`.atelier/items/atelier-ut89/item.md` lines 11–12). That means the
mission page cannot start until the Linux test instance is done. The
other four items that "are seen in T3 Code" (atelier-toee, atelier-40dc,
atelier-9l8h, atelier-wvst) only name diru on their last done-when line,
which is the right shape: implement, then look, and if the instance
cannot start, tick nothing not seen and ask, as the mission already
says (`.atelier/missions/atelier-osel/mission.md` lines 185–187).

ut89's own work does not need diru. It mounts into the layer the old
script already draws, and its fixture checks live in `t3-extension/`.
diru is only the place those screenshots are taken. Putting it on
`blocked_by` also serializes toee → 40dc → 9l8h and wvst behind an
item whose T3-thread path the spike did not walk (Finding 2).

Drop `atelier-diru` from ut89's `blocked_by`. Keep the seen-in-T3
done-when pointing at diru's command.

## Finding 2: atelier-diru's "every hook" line names a path the spike did not try, and omits setup test-contract.mjs already requires
Recommendation: fix now
The new success criterion `test-instance`
(`.atelier/missions/atelier-osel/mission.md` lines 119–122) and
atelier-diru's done-when (`.atelier/items/atelier-diru/item.md` lines
30–32) require a T3 Code thread with an assistant message, and
`node test-contract.mjs --port <port>` passing with every hook.

The spike did not do that. The item says so: "Not tried: enabling
OpenCode inside T3 Code and sending the first message"
(`.atelier/items/atelier-diru/item.md` line 44). What it did try is an
`opencode run` from the isolated home, which is not a T3 thread.
`test-contract.mjs` without `--thread` on a window that has no thread
fails `route:thread` and skips the sidebar, panel and thread-list
hooks (lines 22–25 and 28–43), which is the "15 hooks" the mission
already reports.

The same test also fails unless T3 has a new-thread draft with a
project (`route:draft` / `state:composer-prompt`, lines 52–77: "T3
holds no new-thread draft with a project; open a new thread in a
project, or pass --draft") and Labs → Boring sidebar
(`layout:threadList`, line 35). diru's done-when never names those.
"One command … has a project and a thread" is not enough for the
command it cites.

Name the T3-side steps in diru (enable OpenCode, send one message,
turn on boring sidebar, leave a new-thread draft on the project), or
split "the instance starts and is isolated" from "it has a thread and
every hook", with the ask path on the second. Do not leave "one
command" as the proof of a path nobody has walked.

## Finding 3: Linking the human's live OpenCode folders into the test instance breaks the isolation diru claims
Recommendation: fix now
diru says the instance cannot touch the T3 Code this host already runs,
and a check lists `~/.t3` before and after (`.atelier/items/atelier-diru/item.md`
lines 29 and 40). The way it gets a thread is "OpenCode's configuration
folders linked into the instance's home" (line 30), which the context
names as `~/.config/opencode` and `~/.local/share/opencode` (line 44).
Out forbids signing in any provider that spends the human's allowance
(line 40). The command only refuses a non-free model *it is given*
(line 30). T3 Code, once OpenCode is enabled, can use whatever those
linked folders hold.

On this host those folders exist: `~/.config/opencode/service.json` is
mode 600, and `~/.local/share/opencode` holds `opencode.db` and session
logs. A link lets the test instance read credentials, write the
human's OpenCode sessions, and pick a paid model from that config. The
`~/.t3` check would still pass.

Give the instance its own OpenCode home with only the free model copied
in. Keep the refuse-non-free check. Extend the isolation check to those
folders, or stop linking them.

## Finding 4: needs-you-first says "for each mission" and "the first three"
Recommendation: defer
The criterion at `.atelier/missions/atelier-osel/mission.md` lines
103–109 asks that "for each mission" the page show "a row for each of
the first three" of a needs-human item, a thread that began waiting a
minute ago, a quiet live thread, and two live threads that have
changed. atelier-ut89 splits those across four fixture checks (item.md
lines 39–40): one running page with needs-human, quiet and two live;
one with a one-minute waiting thread and no needs-human; one with
nothing needing the human; one done. No single fixture mission holds
all of them, so "for each mission" and "the first three" will not
match the item that is supposed to prove the criterion.

Rewrite the criterion to list the same cases ut89 already names. The
item is clear; the mission line is what a debrief will score.
