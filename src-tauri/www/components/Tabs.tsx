import "../css/feed.css";
import { Log } from "./Log";
import { useState } from "react";

export const Tabs = () => {
  const [primaryFocused, setFocused] = useState<boolean>(true);

  const onClickPrimary = () => {
    setFocused(true);
  };

  const onClickSecondary = () => {
    setFocused(false);
  };

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
        <Log id={"feed-log"} emptyId={"feed-start"} />
      </div>

      <div
        className={primaryFocused ? "hidden" : ""}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <Log id={"feed-errors"} emptyId={"feed-errors"} />
      </div>
    </div>
  );
};
