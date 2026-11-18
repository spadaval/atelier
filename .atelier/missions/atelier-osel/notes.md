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
- 2026-11-18: extension release 0.2.0 (`t3-extension-v0.2.0`, commit
  9db7a2018) holds the three native pages. atelier-yy4j waits only for the
  human to say their installs run it (`t3-custom update`, or **Update to
  0.2.0** in Customizations). atelier-wvst's Report says `layerTargets.home`
  in `t3-extension/src/layer/layer.tsx` still opens the Desk's
  `/home/<repo>` when T3 has no main area; atelier-yy4j must change it.
