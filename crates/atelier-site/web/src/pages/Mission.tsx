import type { ReactNode } from "react";
import { Link, useParams } from "react-router";
import { useApi } from "../api";
import type { BriefResponse } from "../api/BriefResponse";
import type { DebriefResponse } from "../api/DebriefResponse";
import type { MissionResponse } from "../api/MissionResponse";
import type { MissionSummary } from "../api/MissionSummary";
import { Loading, Page, type Crumb } from "../components/Page";
import { BriefView } from "./Brief";
import { DebriefView } from "./Debrief";

export function missionCrumbs(repo: string, id: string, page?: string): Crumb[] {
  const crumbs: Crumb[] = [{ label: repo }, { label: id, to: page ? `/${repo}/mission/${id}` : undefined }];
  if (page) crumbs.push({ label: page });
  return crumbs;
}

/** Links to a mission's brief, debrief and, when it has one, strategy. */
export function MissionNav({
  repo,
  id,
  strategy,
  current,
}: {
  repo: string;
  id: string;
  strategy: boolean;
  current?: "brief" | "debrief" | "strategy";
}) {
  const pages: ["brief" | "debrief" | "strategy", string][] = [
    ["brief", "Brief"],
    ["debrief", "Debrief"],
  ];
  if (strategy) pages.push(["strategy", "Strategy"]);
  return (
    <nav className="tabs" aria-label="Mission pages">
      {pages.map(([page, label]) =>
        page === current ? (
          <span key={page} aria-current="page">
            {label}
          </span>
        ) : (
          <Link key={page} to={`/${repo}/mission/${id}/${page}`}>
            {label}
          </Link>
        ),
      )}
    </nav>
  );
}

/** `/<repo>/mission/<id>`: the review the server names for the mission. */
export function MissionPage() {
  const { repo = "", id = "" } = useParams();
  const loaded = useApi<MissionResponse>(`/api/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}`);
  return (
    <Loading loaded={loaded} crumbs={missionCrumbs(repo, id)}>
      {({ mission }) => <MissionReview repo={repo} mission={mission} />}
    </Loading>
  );
}

function MissionReview({ repo, mission }: { repo: string; mission: MissionSummary }) {
  const crumbs = missionCrumbs(repo, mission.id);
  const nav = <MissionNav repo={repo} id={mission.id} strategy={mission.links.strategy !== null} />;
  if (mission.review === "brief") {
    return <BriefReview crumbs={crumbs} nav={nav} path={mission.links.brief} title={mission.title} />;
  }
  return <DebriefReview crumbs={crumbs} nav={nav} path={mission.links.debrief} title={mission.title} />;
}

type ReviewProps = { crumbs: Crumb[]; nav: ReactNode; path: string; title: string };

function BriefReview({ crumbs, nav, path, title }: ReviewProps) {
  const loaded = useApi<BriefResponse>(path);
  return (
    <Loading loaded={loaded} crumbs={crumbs}>
      {({ brief }) => (
        <Page crumbs={crumbs} title={`Brief: ${title}`}>
          {nav}
          <BriefView brief={brief} />
        </Page>
      )}
    </Loading>
  );
}

function DebriefReview({ crumbs, nav, path, title }: ReviewProps) {
  const loaded = useApi<DebriefResponse>(path);
  return (
    <Loading loaded={loaded} crumbs={crumbs}>
      {({ debrief }) => (
        <Page crumbs={crumbs} title={`Debrief: ${title}`}>
          {nav}
          <DebriefView debrief={debrief} />
        </Page>
      )}
    </Loading>
  );
}
