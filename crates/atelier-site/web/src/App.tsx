import { Route, Routes } from "react-router";
import { ErrorPage } from "./components/Page";
import { BoardPage } from "./pages/Board";
import { BriefPage } from "./pages/Brief";
import { DebriefPage } from "./pages/Debrief";
import { Home } from "./pages/Home";
import { ItemPage } from "./pages/Item";
import { MissionPage } from "./pages/Mission";
import { StrategyPage } from "./pages/Strategy";

/** The site's routes; the server answers every one of them with this app. */
export function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/:repo" element={<BoardPage />} />
      <Route path="/:repo/item/:id" element={<ItemPage />} />
      <Route path="/:repo/mission/:id" element={<MissionPage />} />
      <Route path="/:repo/mission/:id/brief" element={<BriefPage />} />
      <Route path="/:repo/mission/:id/debrief" element={<DebriefPage />} />
      <Route path="/:repo/mission/:id/strategy" element={<StrategyPage />} />
      <Route path="*" element={<ErrorPage crumbs={[]} status={404} error="No page here." />} />
    </Routes>
  );
}
