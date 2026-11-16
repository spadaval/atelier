---
schema: atelier.review
schema_version: 3
round: 2
commit: d2ee2a4200204831d5058252b7cf47b9d7503ba6
reviewers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3A4572c0cb-f231-4caa-80e2-caeb3f5722cb
model: gpt-6-astra
verdict: approve
created_at: 2026-11-16T02:38:32.409725799Z
---

## Summary

Complexity: high. The mission introduces a host message contract that the Desk and an external extension must both keep, across frontend and server changes.

Value: high. Removing the second frame and following the current thread directly address the stated problem with the Desk in T3 Code.

Risk of merging it: medium. Navigation and appearance reach the user, and host messages expose page context, but the plan now bounds those messages with configured origins and requires interface tests.

The value warrants the scope. The four items separate layout, the host contract, lookup data and navigation, with dependencies that supply each consumer's prerequisites. I found no smaller change that achieves the stated outcome.

I reviewed the mission and all four items at d2ee2a420, compared the revision with round 1, and checked the existing thread map and API types, ThreadTitle and its callers, routes and shell, configuration, core holder and review-round logic, and the Desk product and architecture guidance. `atelier check` passed record validation; it also reported existing empty Why sections in three unrelated missions.

This is a plan review, not an implementation test. I did not run builds or interface tests, inspect the external extension, or verify the desktop application's runtime origin. The configured desktop origin and cross-application integration still need verification during implementation. No new blocking finding remains.

## Finding 1: The thread read gives knx8 too little to choose the page, and knx8 cannot change it
Recommendation: fix now
Was: 1-0pno.1
Now: fixed
atelier-prx5 now requires status and current on each tie, defines current ownership and excludes completed or replaced associations. atelier-knx8 explicitly consumes those marks, specifies precedence and the historical-only inbox fallback, and tests mixed historical and current work. The server work is within prx5 scope.

## Finding 2: The contract does not say which origins it posts to or accepts, and the Desk is framable by any page
Recommendation: fix now
Was: 1-0pno.2
Now: fixed
atelier-fnm5 now requires an empty-by-default origin list supplied by the server, an exact target origin for outbound messages, and checks of both parent source and listed origin for inbound messages. Unlisted parents receive nothing and cannot change the display; the fixture tests this. The mission bounds authority accordingly and names the intended desktop origin.

## Finding 3: Thread links today are web links that open a new tab, not `t3code:` links, so asks-the-host tests the wrong thing
Recommendation: fix now
Was: 1-0pno.3
Now: fixed
The mission and fnm5 now describe web links opening a new tab, explicitly prohibit new tabs and popups in the embedded test, and require the thread identity from Agent.identity or LiveThread.id, with server response changes allowed where needed. This matches the existing ThreadTitle and its callers.

## Finding 4: Define current work in the thread lookup before using it to choose a page
Recommendation: fix now
Was: 1-92k2.1
Now: fixed
The new lookup is no longer constrained to the historical holds map. It owns the current classification and its tests, preserves existing /api/threads behavior, and supplies exactly the marks knx8 uses. Historical-only threads explicitly go to the inbox.
