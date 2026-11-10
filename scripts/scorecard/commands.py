"""What a shell command, its output or an edited path says, for the counted measures.

One owner for the patterns, so the thread reader, the transcript reader and the measures
classify a command the same way.
"""
import re

# Every top-level atelier subcommand seen in the window, across the versions installed then.
SUBCOMMANDS = (
    "brief bundle check close debrief decision doctor evidence help init issue land list log "
    "man merge migrate mission prime prune refresh-update-check review serve setup show start status work"
).split()
_FLAGS = r"(?:[ \t]+(?:-q|--quiet|--log-level[= ]\S+))*"
# The installed binary at command position: not a path to a build (`target/debug/atelier`), not a word in a path.
ATELIER_CALL = re.compile(
    r"(?<![\w./$-])atelier" + _FLAGS + r"[ \t]+(?:(?:" + "|".join(map(re.escape, SUBCOMMANDS)) + r")(?![\w-])|--help|-h(?!\S)|--version|-V(?!\S))"
)
SEGMENT_END = re.compile(r"&&|\|\||[;|\n]")
HELP = re.compile(r"(?:^|[ \t])(?:--help|-h)(?![\w-])|^atelier" + _FLAGS + r"[ \t]+(?:help|man)(?![\w-])")
RECORDS_PATH = re.compile(r"(?<![\w-])\.atelier(?:/|(?=[\s\"'`;)|&]|$))")
# atelier's own refusal and error formats; a harness's "Error: Exit code N" is not one.
REFUSAL = re.compile(
    r"^(?:[a-z][\w-]*: refused\b|Error: (?!Exit code \d)|error: (?:unexpected argument|unrecognized subcommand"
    r"|the argument|the following required arguments|invalid value))",
    re.M,
)
SCRATCH = re.compile(r"(?:^|/)(?:\.atelier|tmp|\.cache|\.claude|\.codex|\.local|scratchpad)(?:/|$)")


def atelier_calls(command):
    """Each atelier invocation in a shell command, as the text up to the end of its pipeline segment."""
    calls = []
    for match in ATELIER_CALL.finditer(command or ""):
        end = SEGMENT_END.search(command, match.end())
        calls.append(command[match.start() : end.start() if end else len(command)])
    return calls


def is_help(command):
    """An atelier call that asks for help: `--help`, `-h`, `atelier help` or `atelier man`."""
    return any(HELP.search(call) for call in atelier_calls(command))


def is_process(command):
    """Runs atelier, or reads or writes a path under `.atelier/` (which covers committing only there)."""
    return bool(atelier_calls(command)) or bool(RECORDS_PATH.search(command or ""))


def refused(output):
    """The output carries one of atelier's refusal or error lines."""
    return bool(REFUSAL.search(output or ""))


def is_product_edit(path):
    """An edited file that is neither a tracker record nor a scratch or harness file."""
    return bool(path) and not SCRATCH.search(path)
