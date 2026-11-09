import { Link } from "react-router";
import { useApi } from "../api";
import type { ReposResponse } from "../api/ReposResponse";
import { Loading, Page, ItemLine } from "../components/Page";
import { shortCommit } from "../text";

/** `/`: every repository, with a link to each of its missions. */
export function Home() {
  const loaded = useApi<ReposResponse>("/api/repos");
  return (
    <Loading loaded={loaded} crumbs={[]}>
      {({ repos }) => (
        <Page crumbs={[]} title="Repositories">
          {repos.length === 0 ? <p className="muted">No Atelier repositories under the server's roots.</p> : null}
          {repos.map((repo) => (
            <section key={repo.repo}>
              <h2>{repo.repo}</h2>
              {repo.base_branch ? (
                <p className="muted">
                  {repo.base_branch}
                  {repo.base_commit ? ` at ${shortCommit(repo.base_commit)}` : ""}
                  {repo.counts
                    ? ` · ${repo.counts.work.ready} ready, ${repo.counts.work.active} active, ${repo.counts.work.human} need a human, ${repo.counts.work.done} done`
                    : ""}
                </p>
              ) : null}
              {repo.error ? (
                <p className="error" role="alert">
                  {repo.error}
                </p>
              ) : repo.missions.length === 0 ? (
                <p className="muted">No missions.</p>
              ) : (
                <ul className="rows">
                  {repo.missions.map((mission) => (
                    <li key={mission.id}>
                      <Link to={`/${repo.repo}/mission/${mission.id}`}>
                        <ItemLine id={mission.id} status={mission.status} title={mission.title} />
                      </Link>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          ))}
        </Page>
      )}
    </Loading>
  );
}
