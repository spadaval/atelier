## Zen of Programming

1. We are not Microsoft.
   - Show useful technical error messages.
   - When something fails, expose the concrete operation, the relevant input or
     identifier, and the actionable reason.
   - Do not hide real failures behind vague or generic messages.

2. We are not Apple.
   - Do not assume the first implementation will be correct.
   - Build incrementally and expect things to break.
   - Prioritize observability as the product grows.
   - Users should be able to understand what the product is doing. They should
     not need to trust that it just works.

3. Code is expensive.
   - Less is more.
   - Prefer the simplest implementation that satisfies the real requirement.
   - Do not add defensive "just in case" guards, abstractions, or edge-case
     handling unless there is a specific reason to be worried about that case.
