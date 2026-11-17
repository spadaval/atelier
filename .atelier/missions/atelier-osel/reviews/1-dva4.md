---
schema: atelier.review
schema_version: 3
round: 1
commit: 35a63b1a89e5947daa4f0304360ad0c8c4ac50ba
reviewers:
- thread:delegated-task:command%3Amcp%3A909059fb-c6b7-42f1-873e-abaee1bead8d%3Adelegate-task%3A2948faa2-5036-43b5-9733-6ec68fc7b107
model: gpt-6-astra
verdict: changes
created_at: 2026-11-17T19:29:22.381276203Z
---

## Summary

Complexity: high. Eight items replace the extension's rendering and build system, share Desk code, and retire part of the host contract.

Value: high. The work implements the chosen native presentation and makes the mission page answer whether the human is needed.

Risk of merging it: medium. The plan changes installed clients and the Desk they depend on; failures reach users, but the changes are reversible and the removal waits for installation.

The value warrants the work. The build-first sequence, retained behaviour checks and delayed removal are useful boundaries. I found three gaps to settle before implementation.

I reviewed commit `35a63b1a89e5947daa4f0304360ad0c8c4ac50ba` in a detached checkout. I read the mission, all eight items, the referenced mockup HTML on `spike/extension-react`, the existing Desk views and API types, the loader's read gate, the extension's tests and release script, and the architecture documentation. I executed the actual `deskReadUrl` definition in Node with a configured fixture origin to check the proposed reads. I did not run T3 Code on Legend, render the mockups, repeat the build spike, or run the application suites; this is a plan review with no implementation changes.

## Finding 1: Plan the loader read permissions the native pages require
Recommendation: fix now

`.atelier/items/atelier-ut89/item.md:47` says a hook over `desk-get` is enough to read history and full thread details. The equivalent assumption appears in `atelier-9l8h/item.md:42` and `atelier-wvst/item.md:40`. However, `t3-extension/t3-custom.mjs:176` only admits the repository list, inbox, sidebar, record lookup, individual items and missions, plus a special case for launch. It rejects history, thread, ties, roadmap and the per-repository missions list. Extracting and executing its current function allowed `/api/repos/atelier/missions/atelier-osel` but rejected `/api/history?repo=atelier&ids=atelier-osel`, `/api/thread?id=thread-a&full=1`, `/api/ties?thread=thread-a` and `/api/roadmap?repo=atelier` with `Desk read is not allowed`.

These are existing server reads, but they are not supported loader reads. No item explicitly owns extending that gate, while the mission's Authority at lines 143–149 requires asking before changing how the loader talks to the extension. Assign the extension of the allowlist to an item before its first native consumer, authorize the exact read paths and query forms needed by all three pages, and require tests that exercise them through the real loader while retaining its origin, traversal and write rejection checks. Include the loader restart needed to activate those permissions in the release/verification instructions. A new React fetch hook alone cannot supply these pages' data.

## Finding 2: Show threads awaiting human input immediately in Needs you
Recommendation: fix now

`.atelier/items/atelier-ut89/item.md:28` restricts Needs you to needs-human items and quiet threads; lines 29–30 only make a thread quiet after 30 minutes and require saying nothing needs the human when those sets are empty. A mission thread with `status: waiting`, updated a minute ago, and an unlabelled item therefore produces no Needs you row and an incorrect all-clear sentence.

This state already exists in the data contract. `crates/atelier-desk/src/t3.rs:17` includes `waiting` among live statuses; `server.rs:670` puts such threads in the inbox immediately, and `server/views.rs:107` counts them against the mission's sidebar attention indicator. This thread also displays them under Waiting on you. The new mission page would disagree with both while claiming to answer whether it needs the human.

Include mission threads explicitly waiting on a person in the sentence and Needs you rows without applying the quiet threshold. Deduplicate a thread that is both waiting and quiet. Add a fixture with a recently updated waiting thread and no needs-human items, so the all-clear case cannot mask it.

## Finding 3: Give proposed missions a supported navigation destination
Recommendation: fix now

`.atelier/items/atelier-wvst/item.md:26–29` preserves Now, Next and Later and says every mission title opens the new mission page. But `.atelier/items/atelier-ut89/item.md:43` excludes proposed plans, and the mission's Why says the page need not serve them because they do not appear in the sidebar. They do appear in project home: `desk/src/pages/ProjectHome.tsx:231` selects `state === "proposed"` for Later, and its `MissionLine` at line 77 links their titles to mission control. This thread also shows missions that its thread planned, including proposed ones, under Work under this thread; `atelier-9l8h/item.md:28` likewise sends a mission title to the new page.

Consequently a normal click from a retained view reaches a state the new destination is explicitly not required to handle. Keep proposed-plan rendering out of this mission if desired, but specify that proposed mission titles in project home and This thread open the retained framed mission record page (or another supported destination). Add navigation coverage for the proposed mission in the Later fixture and for a planning thread. This resolves the contradiction without adding a third design to the native mission page.
