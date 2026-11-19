"""Grade a replay case's facts (harness/replay.py grade); rubric.md asks
whether a fix-now finding names the known defect."""
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
sys.exit(subprocess.call([sys.executable, "-I", str(here.parents[1] / "harness" / "replay.py"),
                          "grade", str(here), sys.argv[1]]))
