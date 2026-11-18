// The Atelier repositories the Desks serve, and which one each T3 project maps to.
import type { ReposResponse } from "../../desk/src/api/ReposResponse";
import type { RepoSummary } from "../../desk/src/api/RepoSummary";
import { deskGet, type ExtensionApi, type RepositoryOverride, type Settings } from "./host/api";
import type { ProjectRef } from "./host/adapter";

/** A repository as a Desk reports it, with the Desk that serves it. */
export type Repository = {
  /** Its name on its Desk. */
  repo: string;
  /** Its `origin` as T3 names a project's repository (`github.com/spadaval/atelier`), or null. */
  identity: string | null;
  /** The prefix of its record IDs: its name, unless config.json's desk.repositories gives another. */
  slug: string;
  /** The repository identities of T3 projects that map to it by themselves. */
  keys: string[];
  desk: string;
};

/** What a Desk's `GET /api/repos` answered, or is still reading. */
export type DeskStatus = { loading: true } | { repos: RepoSummary[] } | { error: Error };

/** A record a link names, with the repository its slug or its Desk says it is in. */
export type RecordRef = { repo: string; kind: "item" | "mission"; id: string };

export const repositoryPath = (repo: { repo: string }): string => `/r/${encodeURIComponent(repo.repo)}`;
export const recordPath = (record: RecordRef): string => `/r/${encodeURIComponent(record.repo)}/${record.kind === "mission" ? "mission" : "item"}/${encodeURIComponent(record.id)}`;
// A repository is named by its Desk page, such as http://buildbot:3780/r/atelier, since two Desks
// may serve repositories of the same name. A bare name, as older settings hold, counts only with
// one Desk configured: with several it cannot say which Desk it meant.
export const repositoryKey = (repo: { desk: string; repo: string }): string => `${repo.desk}${repositoryPath(repo)}`;
export const projectKey = (project: ProjectRef | null): string => (project ? `${project.environmentId}:${project.projectId}` : "");

/**
 * Repositories as the Desks report them (GET /api/repos), each with the Desk that serves it.
 * A repository matches a T3 project when its identity equals the project's
 * repositoryIdentity.canonicalKey, unless the settings map that project to another.
 */
export class Repositories {
  list: Repository[] = [];
  readonly deskStatus: Map<string, DeskStatus>;
  /**
   * A repository a record link opened for one project, which the Desk panel shows in place of the
   * project's own until the project, or a setting, changes.
   */
  override: { projectKey: string; repository: string } | null = null;

  constructor(
    private readonly api: ExtensionApi,
    readonly deskUrls: string[],
    private readonly overrides: RepositoryOverride[],
    private readonly settings: Settings,
  ) {
    this.deskStatus = new Map(deskUrls.map((desk) => [desk, { loading: true }]));
  }

  /** Reads each Desk's repositories; a Desk that cannot be read has an error and none. */
  async load(): Promise<void> {
    await Promise.all(
      this.deskUrls.map(async (desk) => {
        try {
          this.deskStatus.set(desk, { repos: (await deskGet<ReposResponse>(this.api, desk, "/api/repos")).repos });
        } catch (error) {
          this.deskStatus.set(desk, { error: error instanceof Error ? error : new Error(String(error)) });
        }
      }),
    );
    if (this.api.signal.aborted) return;
    this.list = this.deskUrls.flatMap((desk) => {
      const status = this.deskStatus.get(desk);
      return (status && "repos" in status ? status.repos : []).map((repo) => {
        const override = this.overrides.find((entry) => entry.repo === repo.name);
        return { repo: repo.name, identity: repo.identity, slug: override?.slug ?? repo.name, keys: [repo.identity, ...(override?.repositoryKeys ?? [])].filter((key): key is string => Boolean(key)), desk };
      });
    });
  }

  /** The repository a setting names, by its Desk page or, with one Desk, by name. */
  find(value: string | null | undefined): Repository | null {
    if (!value) return null;
    return this.list.find((repo) => repositoryKey(repo) === value) ?? (this.deskUrls.length === 1 ? this.list.find((repo) => repo.repo === value) : null) ?? null;
  }

  /** The repository whose identity is the project's, ignoring the settings. */
  automatic(project: ProjectRef | null): Repository | null {
    return this.list.find((repo) => project?.repositoryKey != null && repo.keys.includes(project.repositoryKey)) ?? null;
  }

  /** The repository a T3 project maps to: its manual mapping, else the one with its identity. */
  mapped(project: ProjectRef | null): Repository | null {
    const key = projectKey(project);
    if (Object.hasOwn(this.settings.mappings, key)) return this.find(this.settings.mappings[key]);
    return this.automatic(project);
  }

  /** The repository the Desk panel shows for the project T3 shows. */
  selected(current: ProjectRef | null): Repository | null {
    if (this.override?.projectKey === projectKey(current)) return this.find(this.override.repository);
    if (this.settings.followProject) return this.mapped(current);
    // A pinned repository that cannot be found shows as not linked; it never falls back to another.
    return this.settings.pinnedRepo ? this.find(this.settings.pinnedRepo) : (this.mapped(current) ?? this.list[0] ?? null);
  }
}
