import contextlib
import io
import os
import tempfile
import unittest

import hours

ROWS = """date,project,hours
2026-11-02,alpha,4
2026-11-03,alpha,2
2026-11-04,beta,2.5
"""


def run(rows, *args):
    """hours.py's output, errors and exit status on a timesheet of `rows`."""
    with tempfile.TemporaryDirectory() as tmp:
        path = os.path.join(tmp, "hours.csv")
        with open(path, "w") as f:
            f.write(rows)
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = hours.main(["--file", path, *args])
        return out.getvalue(), err.getvalue(), code


class TotalTest(unittest.TestCase):
    def test_total(self):
        self.assertEqual(run(ROWS, "total"), ("total 8.5\n", "", 0))


if __name__ == "__main__":
    unittest.main()
