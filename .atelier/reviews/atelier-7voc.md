---
schema: atelier.review
schema_version: 2
id: atelier-7voc
title: finding on atelier-5zre @30c445e1
target: atelier-5zre
identity: GPT-6.1 Sol independent planning reviewer
commit: 30c445e14e674de9cfacb4ba9ac4b6559dd8b599
created_at: 2026-11-11T02:57:18.028444889Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-5zre.md:66, low: A wrong drop does not cost one command to undo as claimed. The old workflow has no transition out of declined or superseded (docs/product/work-model.md:52-53 and .atelier/workflow.yaml); issue update has no status option. The converter then skips both statuses (scripts/cutover/convert_records.py:24-27,610-613), so after cutover the record is absent rather than an item that can be reopened; the new lifecycle also makes dropped terminal (redesign-product.md:109-110). This understates the recovery cost used to justify delegated drop decisions. Replace the claim with the real boundary: before cutover restore status and any affected record content from Git and validate; after cutover recover and convert or recreate the omitted record and links. Keep the existing keep-when-uncertain rule.
