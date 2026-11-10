"""What a shell command, its output or an edited path says, for the counted measures.

One owner for the patterns, so the thread reader, the transcript reader and the measures
classify a command the same way. This is a heuristic over the closed list of command forms in
`README.md`, not a shell: a command is split into simple commands as a shell would split it, and
each simple command is read on its own, with the heredoc it is fed and the directory it runs in.
"""
import re
import shlex
from dataclasses import dataclass, field

PUNCTUATION = ";&|()<>\n"
WRAPPERS = {"command", "env", "exec", "ionice", "nice", "stdbuf", "time", "timeout", "xargs"}
KEYWORDS = {"!", "{", "do", "elif", "else", "if", "then", "until", "while"}
SHELLS = {"bash", "sh", "zsh"}
# A heredoc: its delimiter (not one already lifted), the rest of its line, and its body up to the delimiter
# on a line of its own (which may end a quoted `bash -c` script or a `$(...)`).
HEREDOC = re.compile(r"<<-?[ \t]*(['\"]?)(?!__heredoc_)(\w+)\1([^\n]*)\n(.*?)(?:\n[ \t]*\2[ \t]*(?=[\n'\")]|$)|\Z)", re.S)
PLACEHOLDER = re.compile(r"__(heredoc|subst)_(\d+)__")
PYTHON = re.compile(r"python[\d.]*")
RECORDS_PATH = re.compile(r"(?<![\w-])\.atelier(?:/|(?=[\s\"'`;)|&]|$))")
COMMIT_LINE = re.compile(r"^\[[^\]\s]+(?: \(root-commit\))? ([0-9a-f]{7,40})\]", re.M)
PATCH_FILE = re.compile(r"^\*\*\* (?:Add|Update|Delete) File: (\S+)", re.M)
# Python writes: open(<path>, "w"), Path(<path>).write_text(), <path>.open("w"), where <path> is a literal
# or a name bound to one (`p = Path("src/a.rs")`, `p = "src/a.rs"`).
PYTHON_BIND = re.compile(r"""\b(\w+)\s*=\s*(?:(?:pathlib\.)?Path\(\s*)?["']([^"'\n]+)["']""")
_TARGET = r"""(?:(?:pathlib\.)?Path\(\s*)?(?:["']([^"'\n]+)["']|\b(\w+))\s*\)?"""
PYTHON_WRITE = re.compile(r"\bopen\(\s*" + _TARGET + r"""\s*,\s*["'][wax]|""" + _TARGET + r"""\.(?:write_(?:text|bytes)\(|open\(\s*["'][wax])""")
# atelier's own refusal and error formats; a harness's "Error: Exit code N" is not one.
REFUSAL = re.compile(
    r"^(?:[a-z][\w-]*: refused\b|Error: (?!Exit code \d)|error: (?:unexpected argument|unrecognized subcommand"
    r"|the argument|the following required arguments|invalid value))",
    re.M,
)
SCRATCH = re.compile(r"(?:^|/)(?:\.atelier|\.cache|\.claude|\.codex|\.local|target|node_modules|tmp[^/]*|[^/]*scratch[^/]*)(?:/|$)")
# A shell write counts as an edit only to a source, configuration or documentation file, not captured output.
SOURCE_FILE = re.compile(r"\.(?:rs|ts|tsx|js|jsx|mjs|cjs|svelte|kt|kts|py|go|java|swift|md|toml|json|ya?ml|sh|css|html|sql)$")


@dataclass
class Simple:
    """One simple command: its words after wrappers, the files it redirects output to, and the heredoc it reads."""
    words: list = field(default_factory=list)
    writes: list = field(default_factory=list)
    stdin: str = ""


def simple_commands(command, _bodies=None):
    """The simple commands a shell would run for this text, in order, entering `$(...)` and `bash -c` scripts."""
    bodies = [] if _bodies is None else _bodies  # shared, since a lifted heredoc can sit inside either

    def lift_heredoc(match):
        bodies.append(match.group(4))
        return f"<< __heredoc_{len(bodies) - 1}__{match.group(3)}\n"

    text = HEREDOC.sub(lift_heredoc, (command or "").replace("\\\n", " "))
    text, substitutions = _lift_substitutions(text)
    result = [s for inner in substitutions for s in simple_commands(inner, bodies)]
    lexer = shlex.shlex(text, posix=True, punctuation_chars=PUNCTUATION)
    lexer.whitespace, lexer.commenters = " \t\r", ""
    lexer.wordchars += "$@%+:,{}^!#"  # keep `$f` and `${dir}/x` whole
    found, current, redirect = [], Simple(), None
    try:
        for token in lexer:
            if token and all(c in PUNCTUATION for c in token):
                if "<" in token or ">" in token:
                    redirect = token
                    continue
                found.append(current)
                current = Simple()
            elif redirect:
                if ">" in redirect and not redirect.endswith("&"):
                    current.writes.append(token)
                elif redirect == "<<" and (lifted := PLACEHOLDER.fullmatch(token)):
                    current.stdin = bodies[int(lifted.group(2))]
                redirect = None
            else:
                current.words.append(token)
    except ValueError:  # an unbalanced quote: keep what parsed before it
        pass
    found.append(current)
    for simple in found:
        words = _unwrap(simple.words)
        script = _shell_script(words)
        if script is not None:
            result += simple_commands(script, bodies)
        elif words or simple.writes:
            result.append(Simple(words, simple.writes, simple.stdin))
    return result


def _lift_substitutions(text):
    """Replace each `$(...)` a shell would run (not single-quoted, not escaped) with a placeholder; return the text and their contents."""
    out, inner, i, quote = [], [], 0, None
    while i < len(text):
        char = text[i]
        if char == "\\" and quote != "'":
            out.append(text[i : i + 2])
            i += 2
            continue
        if char == "'" and quote != '"':
            quote = None if quote == "'" else "'"
        elif char == '"' and quote != "'":
            quote = None if quote == '"' else '"'
        elif quote != "'" and text.startswith("$(", i):
            depth, end = 1, i + 2
            while end < len(text) and depth:
                depth += {"(": 1, ")": -1}.get(text[end], 0)
                end += 1
            inner.append(text[i + 2 : end - 1])
            out.append(f"__subst_{len(inner) - 1}__")
            i = end
            continue
        out.append(char)
        i += 1
    return "".join(out), inner


def _unwrap(words):
    """Drop leading variable assignments and wrappers such as `nice -n 19` or `timeout 60`."""
    i = 0
    while i < len(words):
        if re.match(r"^[A-Za-z_]\w*=", words[i]) or words[i] in KEYWORDS:
            i += 1
        elif words[i] == "command" and words[i + 1 : i + 2] in (["-v"], ["-V"]):
            return []  # a lookup, not a run
        elif words[i].rsplit("/", 1)[-1] in WRAPPERS:
            i += 1
            while i < len(words) and (words[i].startswith("-") or re.match(r"^\d", words[i])):
                i += 1
        else:
            break
    return words[i:]


def _shell_script(words):
    if words and words[0].rsplit("/", 1)[-1] in SHELLS:
        for flag, script in zip(words[1:], words[2:]):
            if re.fullmatch(r"-[a-z]*c[a-z]*", flag):
                return script
    return None


def atelier_calls(command):
    """The arguments of each call of the installed atelier in a shell command."""
    return [s.words[1:] for s in simple_commands(command)
            if s.words and (s.words[0] == "atelier" or s.words[0].endswith("/.cargo/bin/atelier"))]


def is_help(command):
    """An atelier call that asks for help: `--help`, `-h`, `atelier help` or `atelier man`."""
    for args in atelier_calls(command):
        first = next((a for a in args if not a.startswith("-")), None)
        if "--help" in args or "-h" in args or first in ("help", "man"):
            return True
    return False


def is_process(command, output="", record_commits=frozenset()):
    """Runs atelier, names a path under `.atelier/`, or made a commit that changed only `.atelier/`.

    A commit is recognised by the hash `git commit` printed, looked up in `record_commits`.
    """
    if atelier_calls(command) or RECORDS_PATH.search(command or ""):
        return True
    if record_commits and any(s.words[:1] == ["git"] and "commit" in s.words for s in simple_commands(command)):
        return any(full.startswith(short) for short in COMMIT_LINE.findall(output or "") for full in record_commits)
    return False


def refused(output):
    """The output carries one of atelier's refusal or error lines."""
    return bool(REFUSAL.search(output or ""))


def is_product_edit(path):
    """An edited file that is neither a tracker record nor a scratch, build or harness file."""
    return bool(path) and not path.startswith("/dev/") and "$" not in path and not SCRATCH.search(path)


def shell_writes(command):
    """Files a shell command writes: redirections, `tee`, `sed -i`, `perl -i`, `cp`/`mv` targets, `apply_patch`
    files and Python writes in code a Python interpreter runs (`-c` or a heredoc).

    A relative path is taken from the directory the last `cd` before the write changed to, if any.
    """
    targets, cwd = [], ""

    def add(paths):
        targets.extend(p if p.startswith(("/", "~")) or not cwd else f"{cwd.rstrip('/')}/{p}" for p in paths)

    for simple in simple_commands(command):
        if simple.words[:1] == ["cd"] and len(simple.words) > 1:
            cwd = simple.words[1] if simple.words[1].startswith(("/", "~")) or not cwd else f"{cwd}/{simple.words[1]}"
        add(simple.writes)
        name, args = (simple.words[0].rsplit("/", 1)[-1], simple.words[1:]) if simple.words else ("", [])
        positional = [a for a in args if not a.startswith("-")]
        if name in ("sed", "perl") and any(re.match(r"^-\w*i|^--in-place", a) for a in args):
            files, script_option, skip = [], False, False
            for arg in args:
                if skip:
                    skip = False
                elif arg.startswith("-"):
                    skip = arg in ("-f", "--expression", "--file") or bool(re.fullmatch(r"-\w*e", arg))
                    script_option = script_option or skip
                else:
                    files.append(arg)
            add(files if script_option else files[1:])
        elif name == "tee":
            add(positional)
        elif name in ("cp", "mv") and len(positional) >= 2:
            add(positional[-1:])
        elif name == "apply_patch":
            add(PATCH_FILE.findall("\n".join([simple.stdin, *args])))
        elif PYTHON.fullmatch(name):
            add(_python_writes(args[args.index("-c") + 1] if "-c" in args[:-1] else simple.stdin if args[:1] in ([], ["-"]) else ""))
    return targets


def _python_writes(code):
    """Paths Python code writes with a literal path, or a name bound to one earlier in the code."""
    binds = [(m.start(), m.group(1), m.group(2)) for m in PYTHON_BIND.finditer(code)]
    for match in PYTHON_WRITE.finditer(code):
        literal, name = match.group(1) or match.group(3), match.group(2) or match.group(4)
        bound = [path for at, var, path in binds if var == name and at < match.start()]
        yield from [literal] if literal else bound[-1:]


def writes_product(command):
    return any(is_product_edit(path) and SOURCE_FILE.search(path) for path in shell_writes(command))
