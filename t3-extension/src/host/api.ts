// What the loader gives the extension: the injected script's `api` (t3-custom.mjs, buildBundle),
// and the answers of the injector's requests (t3-custom.mjs, daemon's `request`). The loader stays
// plain JavaScript, so these types restate it; a change there is a change here.

/** A repository override from config.json's `desk.repositories`. */
export type RepositoryOverride = { repo: string; slug?: string; repositoryKeys?: string[] };

/** The settings the Customizations section edits, saved in config.json (checkSettings in t3-custom.mjs). */
export type Settings = {
  enabled: boolean;
  references: boolean;
  followProject: boolean;
  autoUpdate: boolean;
  /** A repository by its Desk page (`<desk>/r/<name>`), or by bare name in older settings. */
  pinnedRepo: string | null;
  /** T3 project (`<environment>:<project>`) to a repository as `pinnedRepo` names one, or "" for not linked. */
  mappings: Record<string, string>;
};

/** What the client sees of config.json (clientConfig in t3-custom.mjs). */
export type ClientConfig = { urls: string[]; repositories: RepositoryOverride[]; settings: Partial<Settings> };

/** update.json: the last release lookup or update (update in t3-custom.mjs). */
export type ReleaseCheck = {
  checkedAt: string;
  installed: string;
  latest: { version: string; tag: string } | null;
  newer: boolean;
  error: string | null;
};

/** The injector's state, as `status` answers it (fullState in t3-custom.mjs). */
export type InjectorStatus = {
  phase: string;
  version: string | null;
  lastAppliedAt: string | null;
  lastError: string | null;
  /** The installed release, or `development` for a home installed from a checkout. */
  release: string;
  update: ReleaseCheck | null;
  /** An update changed t3-custom.mjs, which runs until the injector restarts. */
  restartNeeded: boolean;
};

/** Each request the injector answers, with its payload and its answer. */
export type Requests = {
  /** A read of a configured Desk, of a path the loader's gate admits (deskReadUrl). */
  "desk-get": { payload: { desk: string; path: string }; answer: unknown };
  status: { payload: undefined; answer: InjectorStatus };
  /** Installs the newest release when it is newer, then reinjects. */
  update: { payload: undefined; answer: ReleaseCheck };
  "open-folder": { payload: undefined; answer: { opened: true } };
  "save-settings": { payload: Settings; answer: unknown };
};

/** A failed request: the injector's message, and the Desk's HTTP status for a `desk-get`. */
export type RequestError = Error & { status?: number | null };

export type ExtensionApi = {
  /** A shadow root of #t3-custom-root, at the end of <body>. */
  root: ShadowRoot;
  /** Aborted when the extension is reloaded or stopped. */
  signal: AbortSignal;
  onCleanup(callback: () => void): void;
  log(...args: unknown[]): void;
  reloadScripts(): void;
  config: ClientConfig;
  /** The injector's home folder, which holds config.json. */
  home: string;
  request<Action extends keyof Requests>(action: Action, ...payload: Requests[Action]["payload"] extends undefined ? [] : [Requests[Action]["payload"]]): Promise<Requests[Action]["answer"]>;
  reportError(error: unknown): void;
  reportProbes(probes: Record<string, Probe>): void;
  version: string;
};

/** A hook's state: found (true), missing (false), or not on screen yet (null), and why. */
export type Probe = { ok: boolean | null; detail: string };

/** Reads `path` from the Desk at `desk` as the type the Desk's API gives for it (desk/src/api). */
export function deskGet<T>(api: Pick<ExtensionApi, "request">, desk: string, path: string): Promise<T> {
  return api.request("desk-get", { desk, path }) as Promise<T>;
}

/** An error's message, whatever was thrown. */
export function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
