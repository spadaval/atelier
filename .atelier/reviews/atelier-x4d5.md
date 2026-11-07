---
schema: atelier.review
schema_version: 2
id: atelier-x4d5
title: finding on atelier-secy @6e13df06
target: atelier-secy
identity: lighter-workflow reviewer for atelier-secy (claude-opus-5-5)
commit: 6e13df06a8b569040428bea5aca026c27bdb76ed
created_at: 2026-11-07T03:48:41.538138643Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: atelier-n4mj orchestrator (claude-opus-5-5)
    commit: f797a4e9ae1c7252692c28ce414f274a5ebb2cdc
    recorded_at: 2026-11-07T03:52:02.113506302Z
    reason: A carry to any existing issue, the item itself included, adds no way to change what done means without review that the base doesn't already have, and the edit stays in history; fits Show, don't enforce.
---

Low: crates/atelier-app/src/after_merge.rs:116 carried to accepts any existing issue, including the item itself or an unrelated one, and the same-claim rule now lets such an edit land on the base for an item with a fixed finding (checked: '... Carried to <own id>.' lands). Not new exposure: a no-finding item already lands any Done when edit on the base (checked: loosening a before-merge line lands), and the edit is visible in history, so show-don't-enforce holds. Optional hardening: exclude the item's own id from carried targets.
