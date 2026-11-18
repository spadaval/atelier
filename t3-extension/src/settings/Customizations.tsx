// The extension's section on T3's Settings → General page: the injector and its release, the
// Desks, the settings saved in config.json, and which of T3's hooks were found. It is drawn as T3
// draws its own settings (apps/web/src/components/settings/settingsLayout.tsx: a titled section of
// grouped rows), with T3's Button, Badge, Switch and Select.
import { type ReactNode, useEffect, useState } from "react";
import type { Extension } from "../core/extension";
import { messageOf, type InjectorStatus, type Settings } from "../host/api";
import { hostProbes, type ProbeName } from "../host/adapter";
import { type Repository, projectKey, repositoryKey } from "../repositories";
import { Badge } from "../t3/ui/badge";
import { Button } from "../t3/ui/button";
import { cn } from "../t3/cn";
import { Select, SelectItem, SelectPopup, SelectTrigger, SelectValue } from "../t3/ui/select";
import { Switch } from "../t3/ui/switch";
import { deskHooks, viewMissing, views, type ViewName } from "../views";

export type CustomizationsActions = {
  /** Sets a boolean setting and saves it. */
  toggle(key: "enabled" | "references" | "followProject" | "autoUpdate", value: boolean): void;
  /** Maps the current project: "@automatic" to follow its identity, "" for not linked, or a repository key. */
  map(value: string): void;
  pin(value: string): void;
  removeMapping(project: string): void;
  showDesk(): void;
  /** Reads the Desks' repositories again. */
  reloadRepositories(): void;
};

export type CustomizationsProps = {
  ext: Extension;
  actions: CustomizationsActions;
  /** Counts each time the section is placed on the page, which reads the injector's status again. */
  placed: number;
};

/** A titled group of rows, as T3's SettingsSection and SettingsGroup draw one. */
function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="space-y-2.5">
      <div className="flex min-h-7 items-center px-3 sm:px-4">
        <h3 className="text-base font-semibold text-foreground">{title}</h3>
      </div>
      <div className="relative overflow-visible rounded-xl border border-border/60 bg-card/40 text-foreground shadow-xs/5 [&>*+*]:border-t [&>*+*]:border-border/50">{children}</div>
    </section>
  );
}

/** A row: its title and description beside its control, as T3's SettingsRow draws one. */
function Row({ title, description, control, children, ...props }: { title: ReactNode; description?: ReactNode; control?: ReactNode; children?: ReactNode; id?: string; hidden?: boolean }) {
  return (
    <div {...props} className={cn("@container/settings-row px-3 sm:px-4", children ? "pt-3 pb-3" : "py-3")}>
      <div className="flex flex-col gap-3 @min-[32rem]/settings-row:grid @min-[32rem]/settings-row:grid-cols-[minmax(0,1fr)_auto] @min-[32rem]/settings-row:items-center @min-[32rem]/settings-row:gap-8">
        <div className="min-w-0 flex-1 space-y-1">
          <div className="flex min-h-5 items-center gap-1.5 text-sm font-medium text-foreground">{title}</div>
          {description ? <div className="max-w-2xl text-xs leading-normal text-pretty text-muted-foreground">{description}</div> : null}
        </div>
        {control ? <div className="flex w-full min-w-0 shrink-0 items-center gap-2 @min-[32rem]/settings-row:w-auto @min-[32rem]/settings-row:justify-end @min-[32rem]/settings-row:[&_[data-slot=select-trigger]]:min-w-40">{control}</div> : null}
      </div>
      {children}
    </div>
  );
}

/** A list of lines, one per thing; `missing` marks a line red. */
function Lines({ label, lines }: { label: string; lines: { text: ReactNode; missing?: boolean; key?: string }[] }) {
  return (
    <ul aria-label={label} className="space-y-1 px-3 py-3 text-xs leading-relaxed sm:px-4">
      {lines.map((line, index) => (
        <li key={line.key ?? index} className={cn("[overflow-wrap:anywhere]", line.missing && "text-destructive-foreground")}>
          {line.text}
        </li>
      ))}
    </ul>
  );
}

function SettingSwitch({ setting, label, settings, onChange }: { setting: "enabled" | "references" | "followProject" | "autoUpdate"; label: string; settings: Settings; onChange: CustomizationsActions["toggle"] }) {
  return <Row title={label} control={<Switch data-setting={setting} aria-label={label} checked={settings[setting]} onCheckedChange={(checked) => onChange(setting, checked)} />} />;
}

/** One of T3's selects over `items`; each item carries its value as data-value. */
function Choice({ id, label, value, items, disabled, onChange }: { id: string; label: string; value: string; items: [string, string][]; disabled?: boolean; onChange(value: string): void }) {
  return (
    <Select items={items.map(([value, label]) => ({ value, label }))} value={value} disabled={disabled} onValueChange={(next) => onChange(String(next ?? ""))}>
      <SelectTrigger id={id} aria-label={label} size="sm">
        <SelectValue />
      </SelectTrigger>
      <SelectPopup>
        {items.map(([key, text]) => (
          <SelectItem key={key} value={key} data-value={key}>
            {text}
          </SelectItem>
        ))}
      </SelectPopup>
    </Select>
  );
}

/** The release line: what is newest and when it was checked. */
function releaseLine(status: InjectorStatus): string {
  const update = status.update;
  const checked = update ? `checked ${new Date(update.checkedAt).toLocaleString()}` : "not checked yet";
  const newest = !update || update.error ? "" : update.newer && update.latest ? `release ${update.latest.version} is available` : update.latest ? `newest release ${update.latest.version}, up to date` : "no extension release published";
  return [newest, checked].filter(Boolean).join(" · ");
}

export function Customizations({ ext, actions, placed }: CustomizationsProps) {
  const { api, settings, repositories, deskUrls, current, host } = ext;
  const [status, setStatus] = useState<InjectorStatus | null>(null);
  const [installing, setInstalling] = useState<string | null>(null);
  async function readStatus() {
    try {
      const next = await api.request("status");
      if (api.signal.aborted) return;
      setStatus(next);
      ext.shownError = ext.hostError?.message ?? next.lastError ?? null;
      ext.scheduleDOM();
    } catch (error) {
      if (!api.signal.aborted) ext.displayError(error);
    }
  }
  useEffect(() => {
    void readStatus();
  }, [placed]);
  // The injector installs the release and reinjects the extension, which reads the status anew.
  function install(version: string) {
    setInstalling(version);
    void api
      .request("update")
      .then(
        () => readStatus(),
        (error) => {
          if (!api.signal.aborted) ext.displayError(error);
        },
      )
      .finally(() => setInstalling(null));
  }

  const update = status?.update ?? null;
  const releaseProblems = status ? [update?.error && `Release check failed: ${update.error}`, status.restartNeeded && "An update changed t3-custom.mjs: run t3-custom stop, then open T3 Custom again to load it."].filter(Boolean) : [];
  const offer = update?.newer && update.latest ? update.latest.version : null;

  const label = (repo: Repository) => (deskUrls.length > 1 ? `${repo.repo} (${repo.desk})` : repo.repo);
  const automatic = current ? repositories.automatic(current) : null;
  const mapped = current && Object.hasOwn(settings.mappings, projectKey(current)) ? settings.mappings[projectKey(current)] : "@automatic";
  const mappedRepo = repositories.find(mapped);
  const shown = repositories.selected(current) ?? repositories.list[0];
  const mappingItems: [string, string][] = [["@automatic", `Automatic: ${automatic ? label(automatic) : "no Desk repository has this identity"}`], ["", "Not linked"], ...repositories.list.map((repo): [string, string] => [repositoryKey(repo), label(repo)])];
  const repoItems = repositories.list.map((repo): [string, string] => [repositoryKey(repo), label(repo)]);

  const probe = (name: ProbeName) => hostProbes[name]?.ok === false;
  const deskMissing = deskHooks.filter(probe);
  const probeLines: { text: string; missing?: boolean }[] = [{ text: host ? `Desk panel: ${deskMissing.length ? `off, missing ${deskMissing.join(", ")}` : "on"}` : "Desk panel: looking for T3 hooks…", missing: deskMissing.length > 0 }];
  if (host) {
    probeLines.push({ text: `Opening threads from the Desk: ${host.findThread ? "on" : "off, missing module:environment-atoms"}`, missing: !host.findThread });
    for (const [name, view] of Object.entries(views) as [ViewName, (typeof views)[ViewName]][]) {
      const off = viewMissing(name);
      probeLines.push({ text: `${view.label}: ${off.length ? `off${view.off ? ` (${view.off})` : ""}, missing ${off.join(", ")}` : "on"}`, missing: off.length > 0 });
    }
  }
  for (const [name, state] of Object.entries(hostProbes)) probeLines.push({ text: `${state.ok ? "✓" : state.ok === false ? "✗" : "…"} ${name}: ${state.detail}`, missing: state.ok === false });

  const overrideLines: { text: ReactNode; key: string }[] = Object.entries(settings.mappings).map(([key, repo]) => ({
    key: `mapping:${key}`,
    text: (
      <span className="flex items-center gap-2">
        <span>
          Project {key} → {repo || "not linked"}
        </span>
        <Button size="xs" variant="outline" aria-label={`Remove override for ${key}`} onClick={() => actions.removeMapping(key)}>
          Remove
        </Button>
      </span>
    ),
  }));
  for (const entry of api.config.repositories) {
    overrideLines.push({ key: `config:${entry.repo}`, text: `config.json desk.repositories: ${entry.repo}${entry.slug ? `, slug ${entry.slug}` : ""}${entry.repositoryKeys?.length ? `, keys ${entry.repositoryKeys.join(", ")}` : ""}` });
  }

  return (
    <div className="space-y-6 text-foreground">
      <div className="space-y-2 px-3 sm:px-4">
        <h2 className="text-lg font-semibold">Customizations</h2>
        <p className="text-xs text-muted-foreground">
          {status ? `Injector ${status.phase} · version ${status.version} · last applied ${status.lastAppliedAt ? new Date(status.lastAppliedAt).toLocaleTimeString() : "not yet"}` : "Reading injector status…"}
        </p>
        <p className="text-xs whitespace-pre-wrap text-destructive-foreground [overflow-wrap:anywhere]" hidden={!ext.shownError}>
          {ext.shownError ?? ""}
        </p>
        <div className="flex flex-wrap gap-2 pt-1">
          <Button id="reload-scripts" size="sm" variant="outline" onClick={() => api.reloadScripts()}>
            Reload scripts
          </Button>
          <Button
            id="refresh-status"
            size="sm"
            variant="outline"
            onClick={() => {
              void readStatus();
              actions.reloadRepositories();
            }}
          >
            Refresh status
          </Button>
          <Button size="sm" variant="outline" onClick={() => void api.request("open-folder").catch((error) => ext.displayError(error))}>
            Open customization folder
          </Button>
          <Button size="sm" variant="outline" disabled={!current || !host} onClick={actions.showDesk}>
            Open Desk
          </Button>
        </div>
      </div>

      <Section title="Release">
        <Row
          title={
            <>
              Extension
              <span id="release-badge" hidden={!status}>
                {status ? (
                  <Badge variant="outline" data-release-badge="">
                    {status.release}
                  </Badge>
                ) : null}
              </span>
            </>
          }
          description={
            <>
              <span id="release-status">{installing ? `Installing ${installing}…` : status ? releaseLine(status) : "Reading release status…"}</span>
              <span id="release-error" className="block text-destructive-foreground" hidden={!releaseProblems.length}>
                {releaseProblems.join(" ")}
              </span>
            </>
          }
          control={
            <Button id="update-release" size="sm" hidden={!offer} disabled={installing !== null} onClick={() => offer && install(offer)}>
              {offer ? `Update to ${offer}` : ""}
            </Button>
          }
        />
        <SettingSwitch setting="autoUpdate" label="Install new extension releases by themselves (checked at start and daily)" settings={settings} onChange={actions.toggle} />
      </Section>

      <Section title="Desks">
        <Lines
          label="Desks"
          lines={deskUrls.map((desk) => {
            const state = repositories.deskStatus.get(desk);
            const text =
              !state || "loading" in state
                ? "reading…"
                : "error" in state
                  ? `unreachable: ${messageOf(state.error)}`
                  : `reachable, ${state.repos.length} repositories: ${state.repos.map((repo) => `${repo.name} (${repo.error ? `could not be read: ${repo.error}` : (repo.identity ?? "no identity")})`).join(", ")}`;
            return { text: `${desk}: ${text}`, missing: Boolean(state && ("error" in state || ("repos" in state && state.repos.some((repo) => repo.error)))) };
          })}
        />
      </Section>

      <Section title="Settings">
        <Row title="Saved in" description={`${api.home}${api.home.includes("\\") ? "\\" : "/"}config.json`} />
        <SettingSwitch setting="enabled" label="Enable the Atelier views (Desk panel, sidebar, layer, project strip)" settings={settings} onChange={actions.toggle} />
        <SettingSwitch setting="references" label="Show Atelier reference links and hovercards" settings={settings} onChange={actions.toggle} />
        <SettingSwitch setting="followProject" label="Follow the current T3 project" settings={settings} onChange={actions.toggle} />
        <Row
          title="Project repository"
          description={current ? `Current project: ${current.projectName}${current.repositoryKey ? ` (${current.repositoryKey})` : " (no repository identity)"}` : "Open a T3 thread to link its project."}
          control={<Choice id="project-mapping" label="Atelier repository for current T3 project" value={mappedRepo ? repositoryKey(mappedRepo) : mapped} items={mappingItems} disabled={!current} onChange={actions.map} />}
        />
        <Row id="pinned-row" hidden={settings.followProject} title="Pinned repository" control={<Choice id="pinned-repository" label="Pinned Atelier repository" value={shown ? repositoryKey(shown) : ""} items={repoItems} onChange={actions.pin} />} />
      </Section>

      <Section title="Manual overrides">
        <Lines label="Manual repository overrides" lines={overrideLines.length ? overrideLines : [{ key: "none", text: "None: projects follow the repository identity the Desk reports." }]} />
      </Section>

      <Section title="T3 hooks">
        <Lines label="T3 hooks used by this extension" lines={probeLines} />
      </Section>
    </div>
  );
}
