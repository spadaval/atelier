"""Fixtures for a later-round case: harness/later_round_tests.py, run on
this case. Run with `python3 -m unittest` here, with EVAL_ATELIER naming
the binary."""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "later_round_tests", HERE.parents[1] / "harness" / "later_round_tests.py")
later_round_tests = importlib.util.module_from_spec(spec)
spec.loader.exec_module(later_round_tests)


class LaterRoundTest(later_round_tests.LaterRoundCase):
    case = HERE
    wrong = True
