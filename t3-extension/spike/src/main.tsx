// The spike's entry, injected as one script the way t3-custom.mjs injects custom.js.
// `__spike` picks what to mount, so the check can try each way.
import { createRoot } from "react-dom/client";
import { Spike } from "./Spike";
import css from "./styles.css?inline";

type Options = { shadow?: boolean; registerProperties?: boolean; reference?: boolean };
const options: Options = (globalThis as { __spike?: Options }).__spike ?? { shadow: true, registerProperties: true };

/** Tailwind's `@property` rules, which a browser ignores inside a shadow root. */
function propertyRules(text: string): string {
  return text.match(/@property\s+--[\w-]+\s*\{[^}]*\}/g)?.join("\n") ?? "";
}

if (options.reference) {
  // The same tree in the page itself, styled by the host's stylesheet alone.
  const host = document.body.appendChild(document.createElement("div"));
  host.id = "reference";
  createRoot(host).render(<Spike />);
}
if (options.shadow) {
  const host = document.body.appendChild(document.createElement("div"));
  host.id = "extension";
  const root = host.attachShadow({ mode: "open" });
  const sheet = new CSSStyleSheet();
  sheet.replaceSync(css);
  root.adoptedStyleSheets = [sheet];
  if (options.registerProperties) {
    const registered = new CSSStyleSheet();
    registered.replaceSync(propertyRules(css));
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, registered];
  }
  const mount = root.appendChild(document.createElement("div"));
  const portal = root.appendChild(document.createElement("div"));
  createRoot(mount).render(<Spike portal={portal} />);
}
