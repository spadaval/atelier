// The layer's native page (src/layer/layer.ts): the mission page drawn in a shadow root of `host` with T3's
// components. `show` draws a mission, `hide` draws nothing, so a hidden page stops reading the Desk.
import type { HostApi } from "../react/desk";
import { mountReact } from "../react/mount";
import { MissionPage, type MissionPageProps } from "./MissionPage";

export function mountMissionLayer(host: Element, api: HostApi): { show(props: MissionPageProps): void; hide(): void } {
  const view = mountReact(host, api);
  // The page scrolls inside the layer, so its root fills the host.
  const root = host.shadowRoot?.querySelector<HTMLElement>("[data-react-root]");
  if (root) root.style.height = "100%";
  return {
    show: (props) => view.render(<MissionPage key={`${props.desk}\n${props.repo}\n${props.id}`} {...props} />),
    hide: () => view.render(null),
  };
}
