"""Fixtures for a removal replay: harness/removed_tests.py, run on this case.
Run with `python3 -m unittest` here, with EVAL_ATELIER naming the binary."""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "removed_tests", HERE.parents[1] / "harness" / "removed_tests.py")
removed_tests = importlib.util.module_from_spec(spec)
spec.loader.exec_module(removed_tests)


class RemovedTest(removed_tests.RemovedCase):
    case = HERE
