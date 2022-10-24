import "../css/feed.css";
import { Log } from "./Log";
import { useState } from "react";
import { useStore } from "@nanostores/react";
import { selectedItem } from "../stores/engine";

export const Tabs = () => {
  const [primaryFocused, setFocused] = useState<boolean>(true);
  const item = useStore(selectedItem);

  const onClickPrimary = () => setFocused(true);
  const onClickSecondary = () => setFocused(false);

  return (
    <div className={"feed"}>
      <div className={"flex-row"} role="tablist">
        <button
          className={`tab${primaryFocused ? " tab-active" : ""}`}
          onClick={onClickPrimary}
          type="button"
          role="tab"
          id={"tab-console"}
          aria-selected={primaryFocused}
        >
          Console
        </button>
        <button
          className={`tab${!primaryFocused ? " tab-active" : ""}`}
          onClick={onClickSecondary}
          type="button"
          role="tab"
          id={"tab-error"}
          aria-selected={!primaryFocused}
        >
          Errors
        </button>
      </div>

      <div
        className={primaryFocused ? "" : "hidden"}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <Log id={"feed-log"} emptyId={"feed-start"} logs={item?.urls} />
      </div>

      <div
        className={primaryFocused ? "hidden" : ""}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <Log
          id={"error-log"}
          emptyId={"error-start"}
          logs={item?.invalidUrls}
        />
      </div>
    </div>
  );
};
