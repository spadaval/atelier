---
schema: atelier.review
schema_version: 2
id: atelier-bicv
title: finding on atelier-l2aj @1cc3679c
target: atelier-l2aj
identity: gpt-6.1-sol-l2aj-r1
commit: 1cc3679c97b24f2ac05f48910f35f0d229263f59
created_at: 2026-11-11T07:40:03.131439195Z
event:
  event: finding
  recommendation: fix_now
---

low: desk/src/pages/Inbox.tsx:93 R2: Stop showing a committed answer as pending. RepoInbox stores successful acts in local answered state and unconditionally renders PendingMark whenever the inbox no longer lists that item. Committing the answer also leaves the question out of the inbox, so no poll clears this marker. A fixture-only Chromium reproduction at phone width answered atelier-uwxy from the inbox, moved the answer into the committed decisions, cleared both item and mission pending_answers, advanced the inbox commit and triggered the normal visibility refresh. The inbox still showed 'Answered from the Desk, not committed yet', despite the item API reporting the committed answer and no pending answers. This fails the Done when requirement that the pending state comes from pending_answers and lasts until a commit holds it; client act history is not that source of truth. Reconcile each locally remembered answer with its item's existing endpoint and derive the pending marker from pending_answers, removing or updating the notice when committed. Cover the pending-to-committed transition while staying in the inbox. No Rust change is needed.
