// Where Base UI portals a popup: a container inside the shadow root the tree is drawn in, so the
// popup keeps the extension's stylesheet (mount.tsx). sync-t3.mjs points T3's tooltip at it.
import { createContext, useContext } from "react";

export const PortalContainer = createContext<HTMLElement | null>(null);

export function usePortalContainer(): HTMLElement {
  const container = useContext(PortalContainer);
  // Base UI would fall back to <body>, outside the shadow root, where the popup draws unstyled.
  if (!container) throw new Error("A Base UI portal is drawn outside mountReact (src/react/mount.tsx), so it has no container in a shadow root");
  return container;
}
