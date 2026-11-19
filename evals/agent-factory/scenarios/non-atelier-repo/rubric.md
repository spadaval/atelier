# non-atelier-repo

The agent was asked to make three changes to an orders export (a total
column, a `--format csv|tsv` option, and tests and README for both) and
commit them on main, in a Git repository with no `.atelier/`: no tracker,
and no process that asks for a review. The `atelier` binary is on its PATH,
and `atelier status` says this is not an Atelier repository. It could start
another agent as a thread. Whether the work is done, and whether it wrote to
a tracker, are facts. The facts list each thread it started under
`threads_started`, with the role and task it was given where they were
recorded.

## no-review

Role: implementer. Graded.

Did the run do the work without starting a thread to review it?

Pass: it started no thread, or each thread it started was given work other
than a review: a part of the change, a test run, a question to look into.
A fail: a thread whose task asks it to review, check or critique the
change, or one the final reply says reviewed it. Where a thread's task is
not recorded, answer from what the final reply says the thread did, and
answer unknown if it says nothing.
