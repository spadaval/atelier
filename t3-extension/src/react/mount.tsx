// Draws a React tree in a shadow root with T3 Code's own components (src/t3/ui), styled as T3
// draws them. A page of the extension mounts with mountReact and never builds its own root.
import type { ReactNode } from "react";
import { createRoot } from "react-dom/client";
import css from "../styles.css?inline";
import { DeskApi, type HostApi } from "./desk";
import { PortalContainer } from "./portal";

let sheet: CSSStyleSheet | null = null;
let properties: CSSStyleSheet | null = null;

/** Tailwind's `@property` rules, which a browser ignores inside a shadow root. */
function propertyRules(text: string): string {
  return text.match(/@property\s+--[\w-]+\s*\{[^}]*\}/g)?.join("\n") ?? "";
}

/**
 * Draws React trees in a new shadow root of `host`: `render(node)` draws or redraws one, and
 * `unmount()` removes it, which also happens when the extension is reloaded or disposed
 * (api.onCleanup). The shadow root adopts the built stylesheet; Base UI's portals go to a container
 * inside it (portal.ts); hooks reach the injector through `api` (desk.ts). Tailwind's `@property`
 * rules, which give its shadow and ring variables their initial values, are registered on the
 * document once, until the extension goes.
 */
export function mountReact(host: Element, api: HostApi): { render(node: ReactNode): void; unmount(): void } {
  if (!sheet) {
    sheet = new CSSStyleSheet();
    sheet.replaceSync(css);
  }
  if (!properties) {
    const registered = new CSSStyleSheet();
    registered.replaceSync(propertyRules(css));
    properties = registered;
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, registered];
    api.onCleanup(() => {
      document.adoptedStyleSheets = document.adoptedStyleSheets.filter((adopted) => adopted !== registered);
      if (properties === registered) properties = null;
    });
  }
  const shadow = host.attachShadow({ mode: "open" });
  shadow.adoptedStyleSheets = [sheet];
  const mount = shadow.appendChild(document.createElement("div"));
  mount.dataset.reactRoot = "";
  const portal = shadow.appendChild(document.createElement("div"));
  portal.dataset.portalContainer = "";
  const root = createRoot(mount, { onUncaughtError: (error) => api.reportError(error instanceof Error ? (error.stack ?? error.message) : String(error)) });
  let mounted = true;
  const unmount = () => {
    if (!mounted) return;
    mounted = false;
    root.unmount();
    mount.remove();
    portal.remove();
  };
  api.onCleanup(unmount);
  return {
    render(node) {
      if (!mounted) throw new Error("mountReact: render after unmount");
      root.render(
        <DeskApi value={api}>
          <PortalContainer value={portal}>{node}</PortalContainer>
        </DeskApi>,
      );
    },
    unmount,
  };
}
