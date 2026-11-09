import ReactMarkdown, { type Components } from "react-markdown";

/**
 * Whether a link or image destination is kept: relative paths, fragments,
 * `http`, `https` and `mailto`. Any other scheme (`javascript:`, `data:`,
 * `vbscript:` and the rest) is not. Browsers ignore whitespace and control
 * characters inside a scheme, so they are dropped before it is read.
 */
export function allowedUrl(url: string): boolean {
  const cleaned = url.replace(/[\u0000- \u007f]/g, "");
  const scheme = /^([^/?#:]*):/.exec(cleaned);
  if (!scheme) return true;
  return ["http", "https", "mailto"].includes(scheme[1].toLowerCase());
}

const components: Components = {
  a({ href, children }) {
    if (href === undefined || !allowedUrl(href)) return <span className="unlinked">{children}</span>;
    return <a href={href}>{children}</a>;
  },
  img({ src, alt }) {
    if (typeof src !== "string" || !allowedUrl(src)) return <span className="unlinked">{alt}</span>;
    return <img src={src} alt={alt ?? ""} />;
  },
  table({ children }) {
    return (
      <div className="scroll">
        <table>{children}</table>
      </div>
    );
  },
};

/**
 * Markdown from the records, with raw HTML skipped and links and images
 * under one URL policy (`allowedUrl`).
 */
export function Markdown({ text }: { text: string }) {
  return (
    <div className="markdown">
      <ReactMarkdown skipHtml components={components} urlTransform={(url) => url}>
        {text}
      </ReactMarkdown>
    </div>
  );
}
