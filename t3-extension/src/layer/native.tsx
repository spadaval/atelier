// The layer's native page (src/layer/layer.ts): the mission page or the project home, drawn in a
// shadow root of `host` with T3's components. `show` draws a page, `hide` draws nothing, so a
// hidden page stops reading the Desk.
import type { ReactNode } from "react";
import type { HostApi } from "../react/desk";
import { mountReact } from "../react/mount";

export function mountNativeLayer(host: Element, api: HostApi): { show(page: ReactNode): void; hide(): void } {
  const view = mountReact(host, api);
  // The page scrolls inside the layer, so its root fills the host.
  const root = host.shadowRoot?.querySelector<HTMLElement>("[data-react-root]");
  if (root) root.style.height = "100%";
  return {
    show: (page) => view.render(page),
    hide: () => view.render(null),
  };
}
