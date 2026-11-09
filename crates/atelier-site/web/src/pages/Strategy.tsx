import { useParams } from "react-router";
import { useApi } from "../api";
import type { StrategyResponse } from "../api/StrategyResponse";
import { Markdown } from "../components/Markdown";
import { Facts, Loading, Page } from "../components/Page";
import { shortCommit } from "../text";
import { MissionNav, missionCrumbs } from "./Mission";

/** `/<repo>/mission/<id>/strategy`: the strategy's Markdown as committed. */
export function StrategyPage() {
  const { repo = "", id = "" } = useParams();
  const loaded = useApi<StrategyResponse>(`/api/${encodeURIComponent(repo)}/mission/${encodeURIComponent(id)}/strategy`);
  return (
    <Loading loaded={loaded} crumbs={missionCrumbs(repo, id, "Strategy")}>
      {(strategy) => (
        <Page crumbs={missionCrumbs(repo, id, "Strategy")} title={`Strategy: ${strategy.path}`}>
          <MissionNav repo={repo} id={strategy.mission} strategy current="strategy" />
          <Facts
            facts={[
              ["Mission", <code>{strategy.mission}</code>],
              ["Read from", <code>{shortCommit(strategy.base_commit)}</code>],
            ]}
          />
          <Markdown text={strategy.markdown} />
        </Page>
      )}
    </Loading>
  );
}
