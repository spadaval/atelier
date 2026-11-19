"""Turn titles into URL slugs."""
import re


def slugify(title: str) -> str:
    """Lowercase `title`, replace each run of other characters with one hyphen, and trim hyphens."""
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).rstrip("-")
