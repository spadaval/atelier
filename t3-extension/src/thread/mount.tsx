// The panel's native page (src/panel/panel.ts): This thread drawn in a shadow root of `host` with
// T3's components. `show` draws it for a thread, `hide` draws nothing, so a hidden page stops
// reading the Desk; `refresh` draws it afresh, which reads everything again.
import type { HostApi } from "../react/desk";
import { mountReact } from "../react/mount";
import { ThisThread, type ThisThreadProps } from "./ThisThread";

export function mountThisThread(host: Element, api: HostApi): { show(props: ThisThreadProps): void; hide(): void; refresh(): void } {
  const view = mountReact(host, api);
  // The page scrolls inside the panel, so its root fills the host.
  const root = host.shadowRoot?.querySelector<HTMLElement>("[data-react-root]");
  if (root) root.style.height = "100%";
  let shown: ThisThreadProps | null = null;
  let generation = 0;
  const draw = () => view.render(shown ? <ThisThread key={`${shown.desk}\n${shown.repository}\n${shown.thread}\n${generation}`} {...shown} /> : null);
  return {
    show(props) {
      shown = props;
      draw();
    },
    hide() {
      if (!shown) return;
      shown = null;
      draw();
    },
    refresh() {
      generation++;
      draw();
    },
  };
}
