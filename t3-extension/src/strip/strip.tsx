// Project strip: under the new-thread composer of a project with a Desk repository, the cards of
// ProjectStrip.tsx. The strip is placed over T3's chat canvas, below the composer and as wide as it,
// on each DOM pass; its cards are React in a shadow root of their own.
import { element } from "../core/dom";
import type { Extension } from "../core/extension";
import { expectHostDom, hostSelectors } from "../host/adapter";
import { layerTargets, type Layer } from "../layer/layer";
import { mountReact } from "../react/mount";
import { repositoryKey } from "../repositories";
import { viewMissing, viewOn } from "../views";
import { ProjectStrip } from "./ProjectStrip";

export function createProjectStrip(ext: Extension, layer: Layer): void {
  const { api } = ext;
  const host = element("div", { "data-t3-strip": "", hidden: "" });
  host.style.cssText = "position:absolute;z-index:21;box-sizing:border-box;height:176px";
  const view = mountReact(host, api);
  // The cards fill the strip's fixed height.
  const root = host.shadowRoot?.querySelector<HTMLElement>("[data-react-root]");
  if (root) root.style.height = "100%";
  // The composer's width changes without a window resize (panels, sidebar); follow the canvas.
  const resize = new ResizeObserver(() => ext.scheduleDOM());
  // What the strip was last drawn with; it is drawn again only when that changes.
  let drawn = "";

  // Only into the composer of the draft the strip was drawn for: a click never fills another thread's.
  async function fill(draftId: string, text: string) {
    const reason = ext.host?.fillComposer ? await ext.host.fillComposer(draftId, text) : "T3's composer hook is missing";
    if (reason) api.reportError(new Error(`Project strip: the composer was not filled: ${reason}`));
  }

  ext.onDOM(() => {
    const draft = ext.settings.enabled ? (ext.host?.draftContext?.() ?? null) : null;
    const repo = draft && ext.repositories.mapped(draft);
    expectHostDom("composerOverlay", Boolean(repo), "T3 showed a new-thread screen");
    expectHostDom("composerEditor", Boolean(draft), "T3 showed a new-thread screen");
    const overlay = repo && viewOn("strip") ? document.querySelector(hostSelectors.composerOverlay) : null;
    expectHostDom("composerStack", Boolean(overlay), "T3 showed its composer overlay");
    const stack = overlay?.querySelector(hostSelectors.composerStack);
    const canvas = overlay?.parentElement;
    if (!draft || !repo || !stack || !canvas) {
      host.hidden = true;
      if (drawn) view.render(null);
      drawn = "";
      return;
    }
    if (host.parentElement !== canvas) {
      canvas.append(host);
      resize.disconnect();
      resize.observe(canvas);
    }
    const area = canvas.getBoundingClientRect();
    const composer = stack.getBoundingClientRect();
    // Below 700px the In motion card goes; the composer-filling Next up card stays.
    const narrow = composer.width < 700;
    host.toggleAttribute("data-narrow", narrow);
    Object.assign(host.style, { top: `${Math.round(composer.bottom - area.top + 12)}px`, left: `${Math.round(composer.left - area.left)}px`, width: `${Math.round(composer.width)}px` });
    host.hidden = false;
    const composerMissing = viewOn("composer") ? null : viewMissing("composer").join(", ");
    const key = repositoryKey(repo);
    const state = JSON.stringify([key, draft.draftId, narrow, composerMissing]);
    if (state === drawn) return;
    drawn = state;
    view.render(
      <ProjectStrip
        key={key}
        repo={repo}
        narrow={narrow}
        composerMissing={composerMissing}
        threads={() => new Map((viewOn("missionThreads") ? (ext.host?.threadList?.() ?? []) : []).map((thread) => [thread.id, thread]))}
        openInbox={() => layer.open(layerTargets.inbox(ext.deskUrls[0]))}
        openItem={(id) => layer.open(layerTargets.item(repo.desk, repo.repo, id))}
        openHome={() => layer.open(layerTargets.home(repo))}
        showThread={(thread) => ext.host?.showThread(thread.environmentId, thread.id)}
        fill={(text) => void fill(draft.draftId, text)}
      />,
    );
  });
  ext.onDispose(() => {
    resize.disconnect();
    host.remove();
  });
}
