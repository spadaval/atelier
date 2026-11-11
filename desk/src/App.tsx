import { Route, Routes } from "react-router";
import { Crumbs } from "./components/Crumbs";
import { Shell } from "./components/Shell";
import { Empty } from "./components/ui";
import { InboxPage } from "./pages/Inbox";
import { ItemPageView } from "./pages/Item";
import { MissionPageView } from "./pages/Mission";
import { ReposPage } from "./pages/Repos";

export function App() {
  return (
    <Shell>
      <Routes>
        <Route path="/" element={<ReposPage />} />
        <Route path="/inbox" element={<InboxPage />} />
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
