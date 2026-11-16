import { Route, Routes } from "react-router";
import { Crumbs } from "./components/Crumbs";
import { Shell } from "./components/Shell";
import { Empty } from "./components/ui";
import { AgentsPage } from "./pages/Agents";
import { InboxPage } from "./pages/Inbox";
import { ItemPageView } from "./pages/Item";
import { MissionPageView } from "./pages/Mission";
import { DisputedPage } from "./pages/Disputed";
import { MotionPage } from "./pages/Motion";
import { ProposedPage } from "./pages/Proposed";
import { ReposPage } from "./pages/Repos";
import { MissionControlPage } from "./pages/MissionControl";
import { RoadmapPage } from "./pages/Roadmap";
import { ThisThreadPage } from "./pages/ThisThread";

export function App() {
  return (
    <Shell>
      <Routes>
        <Route path="/" element={<ReposPage />} />
        <Route path="/inbox" element={<InboxPage />} />
        <Route path="/t" element={<ThisThreadPage />} />
        <Route path="/home/:repo" element={<RoadmapPage />} />
        <Route path="/mission/:repo/:id" element={<MissionControlPage />} />
        <Route path="/proposed" element={<ProposedPage />} />
        <Route path="/motion" element={<MotionPage />} />
        <Route path="/disputed" element={<DisputedPage />} />
        <Route path="/agents" element={<AgentsPage />} />
        <Route path="/r/:repo/agents" element={<AgentsPage />} />
        <Route path="/r/:repo" element={<MissionPageView />} />
        <Route path="/r/:repo/mission/:id" element={<MissionPageView />} />
        <Route path="/r/:repo/item/:id" element={<ItemPageView />} />
        <Route
          path="*"
          element={
            <div className="page narrow">
              <Crumbs trail={[{ label: "Not found" }]} />
              <Empty title="No page here.">Start from the repositories or the inbox.</Empty>
            </div>
          }
        />
      </Routes>
    </Shell>
  );
}
