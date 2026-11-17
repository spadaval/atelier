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

3. Code is not free.
   - Before you write something, say what it is for: what needs it, and
     what would be worse without it. If you can't say, don't write it.
   - This goes for everything you write.
     - A guard: which failure does it catch, and can that failure happen?
     - A test: which bug would make it fail? A test that cannot fail, or
       that only restates the code, tests nothing.
     - A tool: what does it let you do or check that you couldn't before?
   - Prefer the simplest implementation that satisfies the real requirement.
   - Size is not the measure. A large change with a clear purpose is fine.
     A small one without a purpose is not.
