# Notes of atelier-bnek

What the old mission record held beyond its mission sections.

## Outcome

Every `atelier` command an agent runs inside a T3 thread records which thread, harness, provider, model, effort and role did it, and review records name the reviewer's provider and model in structured fields. The site from mission atelier-yfqu then shows who is working on what, links each thread into T3, and reports work, review outcomes and token cost by model and role, so Claude and Codex models can be compared within each role. See `.atelier/missions/atelier-bnek/`.

## Success criteria

- who-did-what: for one real item implemented by a Claude model and reviewed by a Codex model after attribution lands, the item's page names each thread's provider, model, effort and role, links each thread into T3, and gives each role's token cost, matching T3's thread records and the provider transcripts within the unattributed share the attribution validation reports. Shown by validation item atelier-wo22.

## Background

Split out of atelier-yfqu on 2026-11-08 at Sumeet's direction (planning review finding atelier-i8h1). Decision atelier-7bs1 and ADR 0030 (atelier-tu8c) govern both missions.
