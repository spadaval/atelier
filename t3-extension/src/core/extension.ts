// What every part of the extension shares: the injected api, the settings, the thread T3 shows,
// T3's hooks once found, and one DOM pass that each part adds its sync to.
import { messageOf, type ExtensionApi, type Settings } from "../host/api";
import type { HostAdapter, ThreadContext } from "../host/adapter";
import { projectKey, Repositories } from "../repositories";

// localStorage keeps only view state: which threads had the Desk open.
const viewKey = "t3-custom:atelier:view:v1";

export class Extension {
  readonly deskUrls: string[];
  /**
   * The settings the Customizations section edits live in config.json; saving one rewrites it
   * through the injector, which applies a change of settings alone in place.
   */
  readonly settings: Settings;
  readonly repositories: Repositories;
  readonly view: { openThreads: Record<string, boolean> };
  host: HostAdapter | null = null;
  /** Why T3's context could not be read, shown in Customizations. */
  hostError: Error | null = null;
  /** The thread T3 shows, or null on any other screen. */
  current: ThreadContext | null = null;
  /** The last error a part reported to the human, shown in Customizations. */
  shownError: string | null = null;
  /** Each part's contribution to `__t3Custom.diagnostics()`. */
  readonly diagnostics: Record<string, () => unknown> = {};
  private readonly passes: (() => void)[] = [];
  private readonly contextListeners: ((context: ThreadContext | null) => void)[] = [];
  private readonly mutationListeners: ((records: MutationRecord[]) => void)[] = [];
  private observer: MutationObserver | null = null;
  private readonly disposers: (() => void)[] = [];
  private domFrame: ReturnType<typeof setTimeout> | 0 = 0;
  private contextGeneration = 0;
  private lastProject: string | null = null;
  private routeSeen = "";

  constructor(readonly api: ExtensionApi) {
    this.deskUrls = api.config.urls;
    this.settings = { enabled: true, references: true, followProject: true, autoUpdate: false, pinnedRepo: null, mappings: {}, ...api.config.settings };
    // Manual repository overrides from config.json: extra repository keys or a different slug.
    this.repositories = new Repositories(api, this.deskUrls, api.config.repositories, this.settings);
    let view: Partial<Extension["view"]>;
    try {
      view = JSON.parse(localStorage.getItem(viewKey) ?? "{}");
    } catch {
      view = {};
    }
    this.view = { openThreads: {}, ...view };
    // On T3's settings page no thread is shown; the last one is kept so a project can be linked.
    if (location.hash.includes("/settings")) {
      try {
        this.current = JSON.parse(sessionStorage.getItem(viewKey + ":context") ?? "null");
      } catch {}
    }
    api.onCleanup(() => {
      clearTimeout(this.domFrame);
      // One part failing to clean up must not leave the others' changes to T3's page in place.
      for (const dispose of this.disposers.reverse()) {
        try {
          dispose();
        } catch (error) {
          console.error("[T3 Custom] Cleanup failed", error);
        }
      }
    });
  }

  saveView(): void {
    localStorage.setItem(viewKey, JSON.stringify(this.view));
  }

  /** Adds a listener that goes when the extension does. */
  listen<K extends keyof DocumentEventMap>(node: Document, name: K, callback: (event: DocumentEventMap[K]) => void, options?: AddEventListenerOptions): void;
  listen<K extends keyof WindowEventMap>(node: Window, name: K, callback: (event: WindowEventMap[K]) => void, options?: AddEventListenerOptions): void;
  listen<K extends keyof HTMLElementEventMap>(node: HTMLElement, name: K, callback: (event: HTMLElementEventMap[K]) => void, options?: AddEventListenerOptions): void;
  listen(node: EventTarget, name: string, callback: (event: never) => void, options: AddEventListenerOptions = {}): void {
    node.addEventListener(name, callback as EventListener, { ...options, signal: this.api.signal });
  }

  /** Runs `dispose` when the extension goes, after everything added later. */
  onDispose(dispose: () => void): void {
    this.disposers.push(dispose);
  }

  /** Adds a part's sync to the DOM pass, which runs them in the order added. */
  onDOM(sync: () => void): void {
    this.passes.push(sync);
  }

  /** Runs the DOM pass soon, once however often it is asked for before it runs. */
  scheduleDOM(): void {
    if (this.domFrame || this.api.signal.aborted) return;
    // Electron suspends animation frames while its window is hidden.
    this.domFrame = setTimeout(() => {
      this.domFrame = 0;
      for (const sync of this.passes) sync();
    }, 0);
  }

  /**
   * Tells `listener` of each change T3 makes to its page, after any change starts a DOM pass.
   * aria-expanded is watched too: a closed folder holding only the open thread's row reopens with no
   * other change.
   */
  onMutation(listener: (records: MutationRecord[]) => void): void {
    if (!this.observer) {
      const observer = new MutationObserver((records) => {
        for (const listener of this.mutationListeners) listener(records);
        this.scheduleDOM();
      });
      observer.observe(document.body, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: ["aria-expanded"] });
      this.observer = observer;
      this.onDispose(() => observer.disconnect());
    }
    this.mutationListeners.push(listener);
  }

  /** Tells `listener` of each thread T3 shows: null while one is being read, then the thread. */
  onContext(listener: (context: ThreadContext | null) => void): void {
    this.contextListeners.push(listener);
  }

  /** Shows an error in Customizations. */
  displayError(error: unknown): void {
    this.shownError = messageOf(error);
    this.scheduleDOM();
  }

  async saveSettings(): Promise<void> {
    try {
      await this.api.request("save-settings", this.settings);
    } catch (error) {
      this.displayError(error);
    }
  }

  /** Reads which thread T3 shows, after T3 changed its route. */
  async updateContext(): Promise<void> {
    const host = this.host;
    if (!host) return;
    const generation = ++this.contextGeneration;
    if (location.hash.includes("/settings")) {
      this.scheduleDOM();
      return;
    }
    this.current = null;
    for (const listener of this.contextListeners) listener(null);
    this.scheduleDOM();
    if (!host.context) return;
    try {
      const next = await host.context();
      if (this.api.signal.aborted || generation !== this.contextGeneration) return;
      this.current = next;
      if (next) sessionStorage.setItem(viewKey + ":context", JSON.stringify(next));
      const key = projectKey(next);
      if (key !== this.lastProject) {
        this.repositories.override = null;
        this.lastProject = key;
      }
      for (const listener of this.contextListeners) listener(next);
      this.scheduleDOM();
    } catch (error) {
      if (!this.api.signal.aborted && generation === this.contextGeneration) {
        this.hostError = error instanceof Error ? error : new Error(String(error));
        this.displayError(error);
        this.api.reportError(error);
      }
    }
  }

  /**
   * Follows T3's route: on hashchange, and every 300 ms for a route T3 changes without one. Before
   * T3's hooks are found a change only redraws.
   */
  watchRoute(): void {
    const changed = () => {
      if (this.routeSeen === location.hash) return;
      this.routeSeen = location.hash;
      if (this.host) void this.updateContext();
      else this.scheduleDOM();
    };
    const timer = setInterval(changed, 300);
    this.onDispose(() => clearInterval(timer));
    this.listen(window, "hashchange", () => {
      this.routeSeen = location.hash;
      if (this.host) void this.updateContext();
    });
  }
}
