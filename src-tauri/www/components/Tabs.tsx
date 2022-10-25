import "../css/feed.css";
import { LogErrors, LogValid } from "./Log";
import { useState } from "react";

export const Tabs = () => {
  const [primaryFocused, setFocused] = useState<boolean>(true);

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
        <LogValid />
      </div>

      <div
        className={primaryFocused ? "hidden" : ""}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <LogErrors />
      </div>
    </div>
  );
};
