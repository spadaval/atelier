//! The static HTML page a mission review is written to (atelier-rx22). A
//! page is one self-contained document: inline CSS with its own light and
//! dark palettes, no external resources, no scripts or forms, and every
//! record string escaped. It is a document a human reads, not a second UI.

/// Escapes text for HTML element content and attribute values.
pub(crate) fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

/// Record text as paragraphs and lists: blank lines separate paragraphs,
/// `- ` lines become list items, and `` `code` `` spans are kept. Nothing in
/// the text is trusted as HTML.
pub(crate) fn prose(text: &str) -> String {
    let mut html = String::new();
    let mut paragraph: Vec<String> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    let flush_paragraph = |html: &mut String, paragraph: &mut Vec<String>| {
        if !paragraph.is_empty() {
            html.push_str(&format!("<p>{}</p>", inline(&paragraph.join(" "))));
            paragraph.clear();
        }
    };
    let flush_list = |html: &mut String, list: &mut Vec<String>| {
        if !list.is_empty() {
            html.push_str("<ul>");
            for item in list.iter() {
                html.push_str(&format!("<li>{}</li>", inline(item)));
            }
            html.push_str("</ul>");
            list.clear();
        }
    };
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            flush_paragraph(&mut html, &mut paragraph);
            flush_list(&mut html, &mut list);
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            flush_paragraph(&mut html, &mut paragraph);
            list.push(item.to_string());
        } else if line.starts_with(char::is_whitespace) && !list.is_empty() {
            if let Some(last) = list.last_mut() {
                last.push(' ');
                last.push_str(trimmed);
            }
        } else {
            flush_list(&mut html, &mut list);
            paragraph.push(trimmed.to_string());
        }
    }
    flush_paragraph(&mut html, &mut paragraph);
    flush_list(&mut html, &mut list);
    html
}

/// Escaped text with `` `code` `` spans as `<code>`.
fn inline(text: &str) -> String {
    let mut html = String::new();
    for (position, part) in text.split('`').enumerate() {
        if position % 2 == 1 {
            html.push_str(&format!("<code>{}</code>", escape(part)));
        } else {
            html.push_str(&escape(part));
        }
    }
    html
}

/// A status word as a badge, coloured by its category.
pub(crate) fn badge(word: &str, tone: Tone) -> String {
    format!(
        "<span class=\"badge {}\">{}</span>",
        tone.class(),
        escape(word)
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Tone {
    Good,
    Warn,
    Bad,
    Neutral,
}

impl Tone {
    fn class(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Warn => "warn",
            Self::Bad => "bad",
            Self::Neutral => "neutral",
        }
    }
}

/// A summary tile: a count and what it counts.
pub(crate) fn tile(count: usize, label: &str, detail: &str, tone: Tone) -> String {
    format!(
        "<div class=\"tile {}\"><div class=\"count\">{count}</div><div class=\"label\">{}</div><div class=\"detail\">{}</div></div>",
        tone.class(),
        escape(label),
        escape(detail)
    )
}

/// A page section with an anchor and a heading.
pub(crate) fn section(id: &str, title: &str, body: &str) -> String {
    format!(
        "<section id=\"{}\"><h2><a href=\"#{}\">{}</a></h2>{body}</section>",
        escape(id),
        escape(id),
        escape(title)
    )
}

/// The whole document around `body`.
pub(crate) fn page(title: &str, body: &str, footer: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"color-scheme\" content=\"light dark\">\n<title>{}</title>\n<style>{STYLE}</style>\n</head>\n<body>\n<main>\n{body}\n</main>\n<footer>{}</footer>\n</body>\n</html>\n",
        escape(title),
        escape(footer)
    )
}

const STYLE: &str = r#"
:root{--bg:#fbfbfa;--fg:#1d1d1b;--muted:#62625d;--line:#e2e1dc;--card:#ffffff;--code:#f1f0ec;
--good:#1f7a4d;--good-bg:#e3f3ea;--warn:#8a5a00;--warn-bg:#fbf0d9;--bad:#b3261e;--bad-bg:#fbe4e2;--neutral:#4a4a46;--neutral-bg:#ecebe6;--link:#2457a5}
@media (prefers-color-scheme: dark){:root{--bg:#161615;--fg:#e8e7e2;--muted:#a3a29b;--line:#34332f;--card:#1e1e1c;--code:#2a2926;
--good:#7fd1a3;--good-bg:#173527;--warn:#f0c46b;--warn-bg:#3a2e12;--bad:#f2948d;--bad-bg:#43201d;--neutral:#c9c8c1;--neutral-bg:#2c2b28;--link:#8fb4f2}}
*{box-sizing:border-box}
body{margin:0;background:var(--bg);color:var(--fg);font:15px/1.55 system-ui,-apple-system,"Segoe UI",sans-serif}
main{max-width:980px;margin:0 auto;padding:28px 20px 12px}
footer{max-width:980px;margin:0 auto;padding:12px 20px 32px;color:var(--muted);font-size:13px}
h1{font-size:24px;line-height:1.3;margin:0 0 6px}
h2{font-size:18px;margin:32px 0 10px;padding-bottom:6px;border-bottom:1px solid var(--line)}
h2 a{color:inherit;text-decoration:none}
a{color:var(--link)}
.meta{color:var(--muted);margin:0 0 4px}
.outcome{background:var(--card);border:1px solid var(--line);border-radius:8px;padding:12px 16px;margin:14px 0}
.tiles{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:10px;margin:16px 0}
.tile{background:var(--card);border:1px solid var(--line);border-left:4px solid var(--neutral);border-radius:8px;padding:10px 12px}
.tile.good{border-left-color:var(--good)}.tile.warn{border-left-color:var(--warn)}.tile.bad{border-left-color:var(--bad)}
.tile .count{font-size:24px;font-weight:600}.tile .label{font-weight:600}.tile .detail{color:var(--muted);font-size:13px}
nav.toc{display:flex;flex-wrap:wrap;gap:6px 14px;font-size:14px;margin:6px 0 0}
table{width:100%;border-collapse:collapse;background:var(--card);border:1px solid var(--line);border-radius:8px;overflow:hidden}
th,td{text-align:left;vertical-align:top;padding:7px 10px;border-bottom:1px solid var(--line)}
th{font-size:13px;color:var(--muted);font-weight:600}
tr:last-child td{border-bottom:0}
.badge{display:inline-block;font-size:12px;font-weight:600;padding:1px 8px;border-radius:999px;white-space:nowrap}
.badge.good{color:var(--good);background:var(--good-bg)}.badge.warn{color:var(--warn);background:var(--warn-bg)}
.badge.bad{color:var(--bad);background:var(--bad-bg)}.badge.neutral{color:var(--neutral);background:var(--neutral-bg)}
details{background:var(--card);border:1px solid var(--line);border-radius:8px;margin:8px 0}
details>summary{cursor:pointer;padding:9px 12px;list-style-position:inside}
details>div{padding:0 14px 10px}
details.child{margin-left:24px}
dl{margin:6px 0}dt{font-weight:600;margin-top:8px}dd{margin:2px 0 0 0}
dd p,dd ul,.outcome p{margin:4px 0}
code{background:var(--code);padding:0 4px;border-radius:4px;font-size:13px}
.id{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:13px}
ul.facts{padding-left:20px}ul.facts li{margin:4px 0}
.muted{color:var(--muted)}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_text_is_escaped_and_never_trusted_as_html() {
        assert_eq!(
            escape("<script>alert('x') & \"y\"</script>"),
            "&lt;script&gt;alert(&#39;x&#39;) &amp; &quot;y&quot;&lt;/script&gt;"
        );
        assert_eq!(
            prose("First <b>line</b>\ncontinues `a<b>`\n\n- one\n  more\n- two"),
            "<p>First &lt;b&gt;line&lt;/b&gt; continues <code>a&lt;b&gt;</code></p><ul><li>one more</li><li>two</li></ul>"
        );
        let html = page("T<", "<p>body</p>", "f&");
        assert!(html.contains("<title>T&lt;</title>") && html.contains("<footer>f&amp;</footer>"));
        assert!(!html.contains("<script") && !html.contains("http"), "no scripts or external resources");
    }
}
