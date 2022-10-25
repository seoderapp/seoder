import "../styles/feed.css";
import { LogErrors, LogInvalid, LogValid } from "./Log";
import { useState } from "react";

enum TabType {
  "valid",
  "invalid",
  "errors",
}

export const Tabs = () => {
  const [focused, setFocused] = useState<TabType>(TabType.valid);

  const onClickValid = () => setFocused(TabType.valid);
  const onClickInvalid = () => setFocused(TabType.invalid);
  const onClickErrors = () => setFocused(TabType.errors);

  const validFocused = focused === TabType.valid;
  const invalidFocused = focused === TabType.invalid;
  const errorFocused = focused === TabType.errors;

  return (
    <div className={"feed"}>
      <div className={"flex-row"} role="tablist">
        <button
          className={`tab${validFocused ? " tab-active" : ""}`}
          onClick={onClickValid}
          type="button"
          role="tab"
          id={"tab-console"}
          aria-selected={validFocused}
        >
          Valids
        </button>
        <button
          className={`tab${invalidFocused ? " tab-active" : ""}`}
          onClick={onClickInvalid}
          type="button"
          role="tab"
          id={"tab-error"}
          aria-selected={invalidFocused}
        >
          Invalids
        </button>
        <button
          className={`tab${errorFocused ? " tab-active" : ""}`}
          onClick={onClickErrors}
          type="button"
          role="tab"
          id={"tab-invalid"}
          aria-selected={errorFocused}
        >
          Errors
        </button>
      </div>

      <div
        className={validFocused ? "" : "hidden"}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <LogValid scrolling={validFocused} />
      </div>

      <div
        className={invalidFocused ? "" : "hidden"}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <LogInvalid scrolling={invalidFocused} />
      </div>

      <div
        className={errorFocused ? "" : "hidden"}
        role="tabpanel"
        aria-labelledby="tab-console"
      >
        <LogErrors scrolling={errorFocused} />
      </div>
    </div>
  );
};
