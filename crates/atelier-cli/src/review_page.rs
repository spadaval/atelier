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
pub(crate) fn inline(text: &str) -> String {
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

/// A review page with a persistent sidebar beside the main column.
pub(crate) fn review_document(title: &str, sidebar: &str, main: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"color-scheme\" content=\"light dark\">\n<title>{}</title>\n<style>{REVIEW_STYLE}</style>\n</head>\n<body>\n<div class=\"layout\">\n<aside>{sidebar}</aside>\n<main>{main}</main>\n</div>\n</body>\n</html>\n",
        escape(title)
    )
}

const REVIEW_STYLE: &str = r#"
:root{--bg:#fafaf8;--side:#f1f0ec;--fg:#1c1c1a;--muted:#6a6963;--faint:#9d9b93;--line:#e6e5e0;--code:#efeee9;--pop:#ffffff;--chip:#f3f2ee;
--done:#2f7d4f;--active:#2457a5;--todo:#9a7b10;--back:#8a8982;--warn:#a05f00;--warn-bg:#fdf3e1;--done-bg:#e4f2e9;--todo-bg:#f6efd6;--link:#1f4f99}
@media (prefers-color-scheme: dark){:root{--bg:#151514;--side:#1a1a18;--fg:#e8e7e2;--muted:#a3a29b;--faint:#74736c;--line:#2b2a27;--code:#262522;--pop:#232320;--chip:#21211f;
--done:#7fd1a3;--active:#8fb4f2;--todo:#e5cf73;--back:#8a8982;--warn:#f0c46b;--warn-bg:#2f2610;--done-bg:#173527;--todo-bg:#2f2a12;--link:#9dbdf4}}
*{box-sizing:border-box}
html{scroll-padding-top:20px}
body{margin:0;background:var(--bg);color:var(--fg);font:14.5px/1.6 system-ui,-apple-system,"Segoe UI",Ubuntu,Cantarell,"Noto Sans","DejaVu Sans","Liberation Sans",Arial,sans-serif}
a{color:var(--link);text-decoration:none}a:hover{text-decoration:underline}
code{font:12.5px ui-monospace,SFMono-Regular,Menlo,monospace;background:var(--code);padding:1px 4px;border-radius:4px}
.k,.s{font-family:ui-monospace,SFMono-Regular,Menlo,monospace}
.layout{display:grid;grid-template-columns:248px minmax(0,1fr)}
aside{position:sticky;top:0;align-self:start;height:100vh;overflow:auto;background:var(--side);border-right:1px solid var(--line);padding:20px 14px}
aside h1{font-size:14.5px;line-height:1.4;margin:0 0 10px}
.meter{display:flex;justify-content:space-between;font-size:12px;color:var(--muted);margin:10px 0 3px}
.bar{height:5px;border-radius:3px;background:var(--line);overflow:hidden}.bar span{display:block;height:100%;background:var(--done)}
nav{margin-top:18px;font-size:13px}
nav .g{font-size:11px;letter-spacing:.06em;text-transform:uppercase;color:var(--faint);margin:16px 6px 4px}
nav a{display:flex;align-items:center;gap:8px;padding:3px 6px;border-radius:6px;color:var(--fg)}
nav a:hover{background:var(--line);text-decoration:none}
nav a.sub{padding-left:24px;color:var(--muted)}
nav a.ask-link{color:var(--warn);font-weight:600}
nav a.dec-link{color:var(--muted);font-size:12.5px}
nav .label{white-space:nowrap;overflow:hidden;text-overflow:ellipsis;min-width:0}
nav .n{margin-left:auto;color:var(--faint);font-size:12px}
.dot{width:9px;height:9px;border-radius:50%;flex:none;border:2px solid currentColor;display:inline-block}
.dot.done{color:var(--done);background:var(--done)}.dot.todo{color:var(--todo)}.dot.back{color:var(--back);border-style:dashed}
.dot.warn{color:var(--warn);background:var(--warn)}.dot.mark{color:var(--muted);background:var(--bg)}
.dot.active{color:var(--active);background:linear-gradient(90deg,var(--active) 50%,transparent 50%)}
main{padding:30px 44px 80px;max-width:1000px}
h2{font-size:23px;line-height:1.3;margin:0 0 6px}
.props{display:flex;flex-wrap:wrap;align-items:center;gap:4px 14px;font-size:13px;color:var(--muted);margin:0 0 10px}
.lozenge{font-size:11.5px;font-weight:650;padding:1px 7px;border-radius:4px}
.lozenge.done{color:var(--done);background:var(--done-bg)}.lozenge.todo{color:var(--todo);background:var(--todo-bg)}
.verdict{font-size:15.5px;margin:4px 0 0}.verdict a{color:var(--fg);border-bottom:1px solid var(--line)}.verdict a:hover{text-decoration:none;border-color:var(--fg)}
.verdict .sep{color:var(--faint);margin:0 4px}
.lede{font-size:15.5px;max-width:76ch;margin:0 0 8px}
section{margin:38px 0 0}
h3{font-size:12px;letter-spacing:.07em;text-transform:uppercase;color:var(--muted);margin:0 0 12px;padding-bottom:6px;border-bottom:1px solid var(--line)}
h4{font-size:13px;font-weight:620;color:var(--muted);margin:18px 0 6px}
p{margin:6px 0}
ul{margin:4px 0;padding-left:18px}li{margin:2px 0}
.prose p{max-width:76ch}
.status,.st{font-size:12px;font-weight:600;white-space:nowrap}
.status.done,.st.done{color:var(--done)}.status.todo,.st.todo{color:var(--todo)}.status.warn,.st.warn{color:var(--warn)}
.st.active{color:var(--active)}.st.back{color:var(--back)}
.chip{position:relative;color:var(--fg);display:inline-flex;align-items:center;gap:5px;max-width:100%;padding:0 6px;border:1px solid var(--line);border-radius:6px;background:var(--chip);font-size:12.5px;line-height:1.6;white-space:nowrap;vertical-align:baseline;cursor:default;outline:none}
.chip:hover,.chip:focus{border-color:var(--faint)}
.chip .dot{width:8px;height:8px}
.chip .k{font-size:11.5px;color:var(--muted)}
.chip .t{overflow:hidden;text-overflow:ellipsis;min-width:0}
.chip .st{font-size:11px;font-weight:550}
.chip.key{padding:0 5px}.chip.rec .t{color:var(--muted)}
.card{display:none;position:absolute;z-index:5;left:-1px;top:calc(100% + 4px);width:340px;background:var(--pop);color:var(--fg);border:1px solid var(--line);border-radius:8px;box-shadow:0 6px 24px rgba(0,0,0,.16);padding:9px 11px;font-size:12.5px;line-height:1.45;font-weight:400;white-space:normal;text-align:left}
.chip:hover>.card,.chip:focus>.card{display:block}
.card b{display:block;font-size:13px;margin-bottom:2px}
.card .m{display:block;color:var(--muted);font-size:11.5px}
.card .s{display:block;margin-top:5px}.card i{font-style:normal;font-weight:600;color:var(--muted)}
ul.chips{list-style:none;padding:0;margin:4px 0}ul.chips li{margin:4px 0}ul.chips li.kid{padding-left:22px}
.claim{display:grid;grid-template-columns:44px minmax(0,1fr);gap:2px 10px;padding:3px 0}
.claim .s,.crit .s{font-weight:600;font-size:12.5px;color:var(--muted)}
.ask{display:grid;grid-template-columns:22px minmax(0,1fr);gap:2px 10px;padding:10px 0;border-bottom:1px solid var(--line)}
.ask:last-child{border-bottom:0}
.ask .k{color:var(--warn);font-weight:700;font-family:inherit}
.ask .t{font-weight:600}
.ask .d{grid-column:2;color:var(--muted);font-size:13.5px}
.crit{display:grid;grid-template-columns:44px 84px minmax(0,1fr) minmax(0,1.1fr);gap:4px 12px;padding:7px 0;border-bottom:1px solid var(--line);align-items:baseline}
.crit:last-child{border-bottom:0}
.crit.head{padding:0 0 4px;font-size:11.5px;color:var(--faint)}
.crit .s{cursor:help}
.age,.note{font-size:12.5px;color:var(--muted)}
.age{color:var(--warn);white-space:nowrap}
.entry{display:grid;grid-template-columns:70px 18px minmax(0,1fr);gap:0 12px}
.entry .time{text-align:right;font-size:12px;color:var(--muted);padding-top:3px;line-height:1.35}
.entry .time b{display:block;color:var(--fg);font-weight:600}
.entry .rail{position:relative}
.entry .rail:before{content:"";position:absolute;left:8px;top:0;bottom:0;width:1px;background:var(--line)}
.entry .rail .dot{position:absolute;left:4px;top:6px}
.entry .body{padding:0 0 26px;min-width:0}
.entry.ev .body{padding:1px 0 16px;font-size:13.5px}
.entry.ev .time{padding-top:2px}
.entry .title{font-size:15.5px;font-weight:620;line-height:1.4}
.entry.quiet .title{font-size:14.5px;font-weight:560}
.entry.quiet .body{padding-bottom:16px}
.entry .meta{font-size:12.5px;color:var(--muted);margin:2px 0 6px}
.entry .outcome{margin:4px 0 8px}
.fact{display:grid;grid-template-columns:92px minmax(0,1fr);gap:2px 12px;font-size:13.5px;padding:3px 0}
.fact>span:first-child{color:var(--muted);font-size:12.5px;padding-top:2px}
.fact p{margin:0 0 4px}
.fact .r,.decs .r{display:block;color:var(--muted);font-size:13px}
ul.decs{margin:4px 0 2px;font-size:13.5px}
section>.fact{padding:8px 0;border-bottom:1px solid var(--line)}section>.fact:last-child{border-bottom:0}
.changed{color:var(--warn)}
.note-line{display:block;font-size:12.5px}
.oneway{font-size:11px;font-weight:700;color:var(--warn);background:var(--warn-bg);border-radius:4px;padding:0 5px;margin-right:5px}
details.more summary{cursor:pointer;color:var(--link);font-size:12.5px;list-style:none;margin-top:2px}
details.more summary::-webkit-details-marker{display:none}
details.more[open] summary{margin-bottom:4px}
details.rcpt{margin:2px 0;font-size:13px}
details.rcpt summary{cursor:pointer;padding:2px 0}
details.rcpt .sum{color:var(--muted)}
details.rcpt[open]{padding-bottom:6px}details.rcpt>p{margin:4px 0 4px 18px;max-width:76ch}
.row{padding:6px 0;border-bottom:1px solid var(--line)}
.row:last-child{border-bottom:0}
.row.kid{padding-left:22px}
.row .from{display:block;font-size:12.5px;color:var(--muted);margin-top:2px}
@media (max-width:900px){.layout{grid-template-columns:1fr}aside{position:static;height:auto}main{padding:22px}.crit{grid-template-columns:44px 80px 1fr}.crit .by,.crit .proof{grid-column:3}.entry{grid-template-columns:56px 16px 1fr}}
"#;

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
        assert!(
            !html.contains("<script") && !html.contains("http"),
            "no scripts or external resources"
        );
    }
}
