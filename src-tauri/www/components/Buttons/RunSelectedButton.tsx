import { useStore } from "@nanostores/react";
import { selectedEngine } from "../../stores/engine";
import { RunSvg } from "../Svgs/Run";

export const RunSelectedButton = () => {
  const $selected = useStore(selectedEngine);

  return (
    <form
      id="rsform"
      className="panel-btn-ctn full-w"
      style={{ opacity: !!$selected ? 1 : 0.4 }}
    >
      <button
        id="run-selected"
        className="panel-btn full-w center"
        type="submit"
        disabled={!$selected}
      >
        <RunSvg />
        <div>Run Selected</div>
      </button>
    </form>
  );
};
