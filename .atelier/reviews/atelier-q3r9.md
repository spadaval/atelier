---
schema: atelier.review
schema_version: 2
id: atelier-q3r9
title: finding on atelier-94pv @6605ca7f
target: atelier-94pv
identity: gpt-6.1-sol-94pv-r1
commit: 6605ca7fce2df0611d022d32464bd26d569333da
created_at: 2026-11-11T06:40:03.295952541Z
event:
  event: finding
  recommendation: fix_now
---

Low: desk/src/pages/Mission.tsx:457 passes only data to Rail, whose line 56 shows skeletons whenever data is absent. The missions-request error appears only in center-pane at line 463, and that pane is hidden on phone widths for /r/:repo. Opening /r/broken with the existing fixture at 390px after networkidle leaves five skeletons and no visible error although the request has finished with 404. A repository read failure or network failure has the same path. Pass the loading/error state into the rail and show the error there at phone widths, with skeletons only while loading; add a failed repository-route test.
