# Lead's notes

- 2026-11-18, for atelier-9l8h and atelier-wvst: a T3 thread stopped on an
  approval reads as `running`, not `waiting`. The Desk's full thread read
  (`/api/thread?...&full=1`) carries `pending_requests` since atelier-9f7b,
  and the mission page uses it (`liveWorkers` in
  `t3-extension/src/mission/model.ts`). The polled thread list does not carry
  it; atelier-3zh2 is filed for that.
- The test instance (`node t3-extension/t3-instance.mjs start`) holds a
  thread waiting on an approval, framed pages that connect, and a done
  mission with items, since atelier-5cqz. Give it a root of your own with
  `T3_TEST_ROOT`; the default root is shared between threads.
