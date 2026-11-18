// The layer over T3's main area, leaving the sidebar usable. It shows one of the extension's own
// pages, drawn in React with no frame: the mission page (src/mission) or a project's home
// (src/home). Or it shows a Desk page in one frame: the inbox, an item or a proposed mission's
// record page. A framed page opened from a native page has a Back button to it. The frame speaks the host contract: themed on desk:ready,
// desk:open-thread navigates T3 and closes the layer, desk:key Escape closes it. Its messages count
// only from its own frame's window with its Desk's origin, as the panel's do (src/panel/panel.ts).
// The layer sits inside T3's main area, so its frame and buttons are DOM nodes, not React.
import { element, icons } from "../core/dom";
import type { Extension } from "../core/extension";
import { expectHostDom, hostSelectors, observeHostTheme, readHostTheme } from "../host/adapter";
import { ProjectHome } from "../home/ProjectHome";
import { MissionPage } from "../mission/MissionPage";
import { repositoryKey } from "../repositories";
import { viewOn } from "../views";
import { mountNativeLayer } from "./native";

/** A page the extension draws itself: a mission's, or a repository's home. */
export type NativePage = { page: "mission"; repo: string; id: string } | { page: "home"; repo: string };

/** What the layer shows: a Desk page, framed, or a native page when `native` is set. */
export type LayerTarget = {
  /** 'inbox', 'mission:…', 'home:…' or 'item:…'; a row with this key in data-t3-layer-key is pressed while it shows. */
  key: string;
  desk: string;
  /** The Desk page's route, without /embed; a native page's is where the browser opens it when T3 has no main area for the layer. */
  path: string;
  /** Draws the extension's page instead of framing `path`. */
  native: NativePage | null;
  /** The layer a framed page was opened from, which Back returns to. */
  back: LayerTarget | null;
};

const encode = encodeURIComponent;
/** The layer targets the views open. */
export const layerTargets = {
  inbox: (desk: string): LayerTarget => ({ key: "inbox", desk, path: "/inbox", native: null, back: null }),
  home: (repo: { desk: string; repo: string }): LayerTarget => ({ key: `home:${repositoryKey(repo)}`, desk: repo.desk, path: `/home/${encode(repo.repo)}`, native: { page: "home", repo: repo.repo }, back: null }),
  item: (desk: string, repo: string, id: string, back: LayerTarget | null = null): LayerTarget => ({ key: `item:${desk}/${repo}/${id}`, desk, path: `/r/${encode(repo)}/item/${encode(id)}`, native: null, back }),
  /** The mission page, or with `native` false its record page, framed: the mission page does not serve a proposed plan. */
  mission: (desk: string, repo: string, id: string, native = true, back: LayerTarget | null = null): LayerTarget => ({
    key: `mission:${desk}/${repo}/${id}`,
    desk,
    path: `/r/${encode(repo)}/mission/${encode(id)}`,
    native: native ? { page: "mission", repo, id } : null,
    back,
  }),
};

export type Layer = {
  open(target: LayerTarget): void;
  /** Opens `target`, or closes the layer when it already shows it. */
  toggle(target: LayerTarget): void;
  close(): void;
  /** Whether the layer is shown with `key`'s page. */
  pressed(key: string | undefined): boolean;
  /** Places the layer in T3's main area and marks the row of what it shows. */
  sync(): void;
};

export function createLayer(ext: Extension, openThread: (threadId: string) => void): Layer {
  const { api, settings } = ext;
  const style = element("style", {
    text: `
  [data-t3-layer] { position: absolute; inset: 0; z-index: 60; background: var(--background); -webkit-app-region: no-drag; }
  [data-t3-layer-close] { position: absolute; top: 8px; right: 12px; z-index: 1; display: inline-flex; align-items: center; justify-content: center; width: 28px; height: 28px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); color: var(--foreground); cursor: pointer; -webkit-app-region: no-drag; }
  [data-t3-layer-close]:hover, [data-t3-layer-back]:hover { background: var(--accent); }
  [data-t3-layer-back] { position: absolute; top: 8px; right: 48px; z-index: 1; display: inline-flex; align-items: center; gap: 4px; height: 28px; padding: 0 10px 0 6px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); color: var(--foreground); font: 13px var(--font-sans, system-ui, sans-serif); cursor: pointer; -webkit-app-region: no-drag; }
  [data-t3-layer-native] { width: 100%; height: 100%; }
  [data-t3-layer-open] [data-sidebar-row][aria-current] { background: transparent !important; }
`,
  });
  document.head.append(style);

  const frame = { iframe: element("iframe", { title: "Atelier Desk" }), desk: null as string | null, path: null as string | null, ready: false, theme: "" };
  frame.iframe.style.cssText = "display:block;width:100%;height:100%;border:0;background:var(--background)";
  const origin = () => new URL(frame.desk ?? "").origin;
  function sendTheme(always: boolean) {
    if (!frame.ready || !frame.iframe.contentWindow) return;
    const theme = readHostTheme(api.root);
    const text = JSON.stringify(theme);
    if (!always && text === frame.theme) return;
    frame.theme = text;
    frame.iframe.contentWindow.postMessage({ type: "host:theme", ...theme }, origin());
  }
  ext.listen(window, "message", (event) => {
    if (!frame.desk || event.source !== frame.iframe.contentWindow || event.origin !== origin()) return;
    const message: unknown = event.data;
    if (!message || typeof message !== "object") return;
    const { type, thread, key } = message as Record<string, unknown>;
    if (type === "desk:ready") {
      frame.ready = true;
      sendTheme(true);
      if (layer && !layer.native) frame.iframe.focus();
    } else if (type === "desk:open-thread" && typeof thread === "string") {
      openThread(thread);
      close();
    } else if (type === "desk:key" && key === "Escape") close();
  });
  ext.onDispose(observeHostTheme(() => sendTheme(false)));

  // A native page draws its own close and keys; a framed page gets the layer's close, and Back
  // when it was opened from a native page.
  const closeButton = element("button", { "data-t3-layer-close": "", "aria-label": "Close", title: "Close (Esc)", html: icons.close });
  const backButton = element("button", { "data-t3-layer-back": "", html: `${icons.back}<span>Back</span>` });
  const native = element("div", { "data-t3-layer-native": "" });
  const nativePage = mountNativeLayer(native, api);
  const node = element("div", { "data-t3-layer": "", hidden: "" });

  let layer: LayerTarget | null = null;
  function open(target: LayerTarget) {
    // Without T3's main area the page opens in the browser at its normal address.
    if (!viewOn("layer")) {
      window.open(target.desk + target.path, "_blank", "noopener");
      return;
    }
    layer = target;
    const { native: page, desk, path } = target;
    if (page?.page === "mission") {
      nativePage.show(
        <MissionPage
          key={`${desk}\n${page.repo}\n${page.id}`}
          desk={desk}
          repo={page.repo}
          id={page.id}
          onClose={close}
          openThread={(thread) => {
            openThread(thread);
            close();
          }}
          openItem={(id) => open(layerTargets.item(desk, page.repo, id, target))}
          openUrl={(url) => window.open(url, "_blank", "noopener")}
        />,
      );
    } else if (page?.page === "home") {
      nativePage.show(
        <ProjectHome
          key={`${desk}\n${page.repo}`}
          desk={desk}
          repo={page.repo}
          onClose={close}
          // An agreed or done mission opens its mission page; a proposed plan its record page, framed, with Back here.
          openMission={(id, proposed) => open(layerTargets.mission(desk, page.repo, id, !proposed, proposed ? target : null))}
          openItem={(id) => open(layerTargets.item(desk, page.repo, id, target))}
        />,
      );
    } else {
      nativePage.hide();
      backButton.title = target.back?.native?.page === "home" ? "Back to the project home" : "Back to the mission";
      if (frame.desk !== desk || frame.path !== path) {
        Object.assign(frame, { desk, path, ready: false, theme: "" });
        frame.iframe.src = `${desk}/embed${path}`;
      }
    }
    // The layer follows at once: a covered window throttles timers, which delays the DOM pass.
    sync();
  }
  function close() {
    if (!layer) return;
    layer = null;
    nativePage.hide();
    sync();
  }
  const toggle = (target: LayerTarget) => (layer?.key === target.key ? close() : open(target));

  // Opens the mission page for any mission of a Desk, as a mission's title does for the open ones the
  // sidebar lists: for checks and the console, such as a done mission (README.md, "The mission page").
  if (globalThis.__t3Custom) globalThis.__t3Custom.openMission = (repo, id, desk = ext.deskUrls[0]) => open(layerTargets.mission(desk, repo, id));

  ext.listen(closeButton, "click", close);
  ext.listen(backButton, "click", () => {
    if (layer?.back) open(layer.back);
  });
  ext.listen(document, "keydown", (event) => {
    if (event.key === "Escape" && layer) close();
  });
  // Going to a thread in the sidebar, or anywhere else in T3, closes the layer.
  ext.listen(
    document,
    "click",
    (event) => {
      if (event.target instanceof Element && event.target.closest(hostSelectors.sidebarRow)) close();
    },
    { capture: true },
  );
  ext.listen(window, "hashchange", close);

  let shown = false;
  function sync() {
    expectHostDom("mainInset", Boolean(ext.host) && !location.hash.includes("/settings"), "T3 showed a thread or new-thread screen");
    const main = document.querySelector(hostSelectors.mainInset);
    shown = Boolean(layer) && Boolean(main) && settings.enabled;
    const current = shown ? layer : null;
    for (const row of document.querySelectorAll<HTMLElement>("[data-t3-layer-key]")) row.setAttribute("aria-pressed", String(row.dataset.t3LayerKey === current?.key));
    document.documentElement.toggleAttribute("data-t3-layer-open", shown);
    if (current && main && node.parentElement !== main) main.append(node);
    // The layer holds a native page or the frame, never both, so a native page has no iframe.
    // Moving the iframe reloads it, so it is moved only when the layer changes between them.
    const content = !current ? [] : current.native ? [native] : [frame.iframe, ...(current.back ? [backButton] : []), closeButton];
    if (content.length && (content.length !== node.children.length || content.some((child, at) => node.children[at] !== child))) node.replaceChildren(...content);
    // The pages take keys: the frame gets focus as it appears, and again on desk:ready for a page
    // still loading; a native page focuses itself as it is drawn (src/mission/MissionPage.tsx, src/home/ProjectHome.tsx).
    const appearing = current && (node.hidden || node.dataset.shown !== current.key);
    node.hidden = !current;
    node.dataset.shown = current?.key ?? "";
    if (appearing && !current.native) frame.iframe.focus();
    if (!current && (document.activeElement === frame.iframe || document.activeElement === native)) (document.activeElement as HTMLElement).blur();
  }

  ext.diagnostics.layer = () => ({ layer: layer && { ...layer, shown: !node.hidden, ready: frame.ready } });
  ext.onDispose(() => {
    node.remove();
    style.remove();
    document.documentElement.removeAttribute("data-t3-layer-open");
  });

  return { open, toggle, close, pressed: (key) => shown && key !== undefined && layer?.key === key, sync };
}
