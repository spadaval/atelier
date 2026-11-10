---
schema: atelier.review
schema_version: 2
id: atelier-5l9v
title: finding on atelier-yptz @8426a78b
target: atelier-yptz
identity: gpt-6.1-sol
commit: 8426a78b209e2e384efbbfb36984fece4a45fee1
created_at: 2026-11-10T19:38:15.857413539Z
event:
  event: finding
  recommendation: fix_now
---

R2, low severity: .atelier/issues/atelier-yptz.md:38 adds refusal of submit until Goal has text and Done when has at least one line. The product Lifecycle's building exit and Commands/submit require ticked Done when lines and a Report, not a nonempty Goal or a minimum checklist count; the architecture's submit act row and Done when definition likewise contain no such gate. Step A's acts::unmet(Act::Submit) follows that contract, and record validation requires the two headings rather than their contents. start --new needs to create the item and guide its implementer, but it does not authorize rewriting submit's acceptance conditions. The strategy's Authority reserves changes to the product Lifecycle to Sumeet. Remove the extra refusal requirements and retain the orientation telling the implementer to define the item; if the new gates are wanted, obtain the required product decision before asking a worker to add them. Recommendation: FIX NOW, because the plan adds unspecified lifecycle gates outside this increment's delegated authority.
