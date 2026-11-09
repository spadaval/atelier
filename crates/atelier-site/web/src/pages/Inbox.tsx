import { Link } from "react-router";
import { useApi } from "../api";
import type { InboxRepo } from "../api/InboxRepo";
import type { InboxResponse } from "../api/InboxResponse";
import type { MissionSummary } from "../api/MissionSummary";
import { ItemLine, Loading, Page } from "../components/Page";
import { shortCommit } from "../text";

/** A mission's page, named by the review it opens on. */
function MissionLink({ repo, mission }: { repo: string; mission: MissionSummary }) {
  const review = mission.review === "brief" ? "Brief" : "Debrief";
  return (
    <Link to={`/${repo}/mission/${mission.id}`}>
      {review}: {mission.title}
    </Link>
  );
}

function RepoInbox({ repo }: { repo: InboxRepo }) {
  return (
    <section>
      <h2>
        <Link to={`/${repo.repo}`}>{repo.repo}</Link>
      </h2>
      {repo.base_branch ? (
        <p className="muted">
          {repo.base_branch}
          {repo.base_commit ? ` at ${shortCommit(repo.base_commit)}` : ""}
        </p>
      ) : null}
      {repo.error ? (
        <p className="error" role="alert">
          {repo.error}
        </p>
      ) : null}
      {repo.items.length > 0 ? (
        <>
          <h3>Needs a Human</h3>
          <ul className="rows">
            {repo.items.map(({ item, missions }) => (
              <li key={item.id}>
                <Link to={`/${repo.repo}/item/${item.id}`}>
                  <ItemLine
                    id={item.id}
                    issueType={item.issue_type}
                    status={item.status}
                    priority={item.priority}
                    title={item.title}
                  />
                </Link>
                {missions.length > 0 ? (
                  <ul className="links">
                    {missions.map((mission) => (
                      <li key={mission.id}>
                        <MissionLink repo={repo.repo} mission={mission} />
                      </li>
                    ))}
                  </ul>
                ) : null}
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {repo.drafts.length > 0 ? (
        <>
          <h3>Draft Missions to Approve</h3>
          <ul className="rows">
            {repo.drafts.map((mission) => (
              <li key={mission.id}>
                <Link to={`/${repo.repo}/mission/${mission.id}`}>
                  <ItemLine id={mission.id} status={mission.status} title={mission.title} />
                </Link>
              </li>
            ))}
          </ul>
        </>
      ) : null}
    </section>
  );
}

/** `/inbox`: what waits on the human in every repository, linking the page to read. */
export function InboxPage() {
  const loaded = useApi<InboxResponse>("/api/inbox");
  const crumbs = [{ label: "Inbox" }];
  return (
    <Loading loaded={loaded} crumbs={crumbs}>
      {({ count, repos }) => (
        <Page crumbs={crumbs} title="Inbox">
          <p className="muted">
            {count === 0 ? "Nothing waits on you." : `${count} ${count === 1 ? "entry waits" : "entries wait"} on you.`}
          </p>
          {repos.map((repo) => (
            <RepoInbox key={repo.repo} repo={repo} />
          ))}
        </Page>
      )}
    </Loading>
  );
}

/** A link to `/inbox` with how many entries wait there, once known. */
export function InboxLink() {
  const loaded = useApi<InboxResponse>("/api/inbox");
  return (
    <p className="inbox-link">
      <Link to="/inbox">Inbox{loaded.state === "ok" ? ` (${loaded.body.count})` : ""}</Link>
    </p>
  );
}
