// The extension's settings section's React parts (custom.js draws the rest with the DOM).
import { Badge } from "../t3/ui/badge";
import type { HostApi } from "./desk";
import { mountReact } from "./mount";

/** Draws the installed release in T3's Badge in `host`; call the result with each status's release. */
export function mountReleaseBadge(host: Element, api: HostApi): (release: string) => void {
  const view = mountReact(host, api);
  return (release) =>
    view.render(
      <Badge variant="outline" data-release-badge="">
        {release}
      </Badge>,
    );
}
