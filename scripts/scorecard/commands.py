"""What a shell command, its output or an edited path says, for the counted measures.

One owner for the patterns, so the thread reader, the transcript reader and the measures
classify a command the same way. A command is parsed into simple commands the way a shell
would split it (quotes, heredoc bodies, `&&`, `;`, pipes, `env`/`nice`-style wrappers and
`bash -c` scripts), so a word inside a quoted argument or a heredoc is never taken for a call.
"""
import re
import shlex
from dataclasses import dataclass, field

PUNCTUATION = ";&|()<>\n"
WRAPPERS = {"command", "env", "exec", "ionice", "nice", "stdbuf", "time", "timeout", "xargs"}
KEYWORDS = {"!", "{", "do", "elif", "else", "if", "then", "until", "while"}
SUBSTITUTION = re.compile(r"\$\(([^()]*)\)")
SHELLS = {"bash", "sh", "zsh"}
HEREDOC = re.compile(r"<<-?[ \t]*(['\"]?)(\w+)\1[^\n]*\n.*?(?:\n[ \t]*\2[ \t]*(?=\n|$)|\Z)", re.S)
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
    """One simple command: its words after wrappers, and the files it redirects output to."""
    words: list = field(default_factory=list)
    writes: list = field(default_factory=list)


def simple_commands(command):
    """The simple commands a shell would run for this text, recursing into `bash -c` scripts."""
    text = HEREDOC.sub(lambda m: m.group(0).split("\n", 1)[0] + "\n", (command or "").replace("\\\n", " "))
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
                redirect = None
            else:
                current.words.append(token)
    except ValueError:  # an unbalanced quote: keep what parsed before it
        pass
    found.append(current)
    result = []
    for simple in found:
        for word in simple.words:  # a substitution in double quotes, "$(atelier ...)", runs; one in single quotes is taken as running too
            for inner in SUBSTITUTION.findall(word):
                result += simple_commands(inner)
        words = _unwrap(simple.words)
        script = _shell_script(words)
        if script is not None:
            result += simple_commands(script)
        elif words or simple.writes:
            result.append(Simple(words, simple.writes))
    return result


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
    """Files a shell command writes: redirections, `tee`, `sed -i`, `perl -i`, `cp`/`mv` targets, patches and Python `open(..., 'w')`.

    A relative path is taken from the directory the command last changed to with `cd`, if any.
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
    add(PATCH_FILE.findall(command or ""))
    binds = [(m.start(), m.group(1), m.group(2)) for m in PYTHON_BIND.finditer(command or "")]
    for match in PYTHON_WRITE.finditer(command or ""):
        literal, name = match.group(1) or match.group(3), match.group(2) or match.group(4)
        bound = [path for at, var, path in binds if var == name and at < match.start()]
        add([literal] if literal else bound[-1:])
    return targets


def writes_product(command):
    return any(is_product_edit(path) and SOURCE_FILE.search(path) for path in shell_writes(command))
