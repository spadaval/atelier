// T3's useNativeTitlebarHeight (apps/web/src/lib/windowControlsOverlay.ts), which its tooltip
// uses to keep clear of the window's title bar. T3's module reads its app's environment, so this
// restates the hook from the same browser state: Electron on macOS draws a 52px title bar; elsewhere
// the Window Controls Overlay, when visible, gives it.
import { useSyncExternalStore } from "react";

const MACOS_TITLEBAR_HEIGHT = 52;
type Overlay = { visible: boolean; getTitlebarAreaRect(): DOMRect; addEventListener(type: "geometrychange", listener: () => void): void; removeEventListener(type: "geometrychange", listener: () => void): void };
const overlay = () => (navigator as Navigator & { windowControlsOverlay?: Overlay }).windowControlsOverlay ?? null;
// T3's preload installs desktopBridge before its code runs, so it is there by the time the extension is.
const isElectron = () => (window as Window & { desktopBridge?: unknown }).desktopBridge !== undefined;

function subscribe(listener: () => void) {
  overlay()?.addEventListener("geometrychange", listener);
  return () => overlay()?.removeEventListener("geometrychange", listener);
}
function height() {
  if (isElectron() && /mac|iphone|ipad|ipod/i.test(navigator.platform)) return MACOS_TITLEBAR_HEIGHT;
  const current = overlay();
  return current?.visible ? current.getTitlebarAreaRect().bottom : 0;
}

export function useNativeTitlebarHeight(): number {
  return useSyncExternalStore(subscribe, height, () => 0);
}
