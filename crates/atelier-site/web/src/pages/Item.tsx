import type { ReactNode } from "react";
import { Link, useParams } from "react-router";
import { useApi } from "../api";
import type { ItemLink } from "../api/ItemLink";
import type { ItemResponse } from "../api/ItemResponse";
import type { ItemReview } from "../api/ItemReview";
import { Markdown } from "../components/Markdown";
import { Facts, ItemLine, Loading, Page, Section } from "../components/Page";
import { localDate, shortCommit } from "../text";
import { repoCrumbs } from "./Board";

/** How a link another record declares reads from this item's side. */
const incomingWords: Record<string, string> = {
  advances: "advanced by",
  blocked_by: "blocks",
  blocks: "blocked by",
  discovered_from: "led to",
  related: "related",
  supersedes: "superseded by",
};

/** How a link reads from this item's side: `blocked by`, `advanced by`. */
export function linkRole(link: ItemLink): string {
  if (link.incoming) return incomingWords[link.role] ?? `${link.role.replace(/_/g, " ")} (from the other item)`;
  return link.role.replace(/_/g, " ");
}

function LinkRow({ repo, link }: { repo: string; link: ItemLink }) {
  const page = link.issue_type === "mission" ? `/${repo}/mission/${link.id}` : `/${repo}/item/${link.id}`;
  const line = <ItemLine id={link.id} issueType={link.issue_type ?? undefined} status={link.status ?? undefined} title={link.title ?? undefined} />;
  return (
    <li>
      <span className="muted">{linkRole(link)}</span> {link.title === null ? line : <Link to={page}>{line}</Link>}
    </li>
  );
}

/** One review event, as `atelier review list` names it. */
function ReviewRow({ review }: { review: ItemReview }) {
  const head: ReactNode[] = [
    <strong key="event">{review.event.replace(/_/g, " ")}</strong>,
    " ",
    <code key="id">{review.id}</code>,
    ` ${localDate(review.created_at)} by ${review.identity} at `,
    <code key="commit">{shortCommit(review.commit)}</code>,
  ];
  if (review.recommendation !== null) head.push(`, recommends ${review.recommendation}`);
  const disposition = review.disposition;
  return (
    <li className="review">
      <p>{head}</p>
      <Markdown text={review.summary} />
      {disposition === null ? (
        review.event === "finding" ? (
          <p className="warn">⚠ No decision yet.</p>
        ) : null
      ) : (
        <p>
          Decided <strong>{disposition.decision}</strong> by {disposition.identity} on{" "}
          {localDate(disposition.recorded_at)}
          {disposition.follow_up !== null ? (
            <>
              , follow-up <code>{disposition.follow_up}</code>
            </>
          ) : null}
          : {disposition.reason}
        </p>
      )}
    </li>
  );
}

/** `/<repo>/item/<id>`: one item's record, links and review events. */
export function ItemPage() {
  const { repo = "", id = "" } = useParams();
  const loaded = useApi<ItemResponse>(`/api/${encodeURIComponent(repo)}/item/${encodeURIComponent(id)}`);
  return (
    <Loading loaded={loaded} crumbs={repoCrumbs(repo, id)}>
      {({ item }) => {
        const issue = item.issue;
        const facts: [string, ReactNode][] = [
          [
            "Item",
            <ItemLine id={issue.id} issueType={issue.issue_type} status={issue.status} priority={issue.priority} cues={[item.state]} />,
          ],
        ];
        if (item.labels.length > 0) facts.push(["Labels", item.labels.join(", ")]);
        if (issue.workflow_branch) facts.push(["Branch", <code>{issue.workflow_branch.work_branch}</code>]);
        if (issue.closed_at) facts.push(["Closed", localDate(issue.closed_at)]);
        return (
          <Page crumbs={repoCrumbs(repo, issue.id)} title={issue.title}>
            <Facts facts={facts} />
            <Markdown text={item.body} />
            <Section title="Links" show={item.links.length > 0}>
              <ul className="rows">
                {item.links.map((link) => (
                  <LinkRow key={`${link.role} ${link.incoming} ${link.id}`} repo={repo} link={link} />
                ))}
              </ul>
            </Section>
            <Section title="Reviews">
              {item.reviews.length === 0 ? (
                <p className="muted">No review events on this item.</p>
              ) : (
                <ul className="rows">
                  {item.reviews.map((review) => (
                    <ReviewRow key={review.id} review={review} />
                  ))}
                </ul>
              )}
            </Section>
          </Page>
        );
      }}
    </Loading>
  );
}
