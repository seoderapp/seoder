import { useStore } from "@nanostores/react";
import { socket } from "../../events/sockets";
import {
  engines,
  errorLogs,
  invalidLogs,
  selectedEngine,
  validLogs,
} from "../../stores/engine";
import { CellStatus } from "../CampaignCell";
import { RunSvg } from "../Svgs/Run";

export const RunSelectedButton = () => {
  const $selected = useStore(selectedEngine);

  const onSubmitEvent = (event) => {
    event.preventDefault();
    const eitem = engines.get()[$selected];

    if (eitem) {
      if (eitem.status === CellStatus.RUNNING) {
        return alert("Campaign already in progress");
      }

      eitem.urls.clear();
      eitem.invalidUrls.clear();
      eitem.status = CellStatus.RUNNING;

      engines.notify($selected);

      validLogs.set([]);
      errorLogs.set([]);
      invalidLogs.set([]);

      socket.send("run-campaign " + $selected);
    } else {
      alert("Please select a campaign");
    }
  };

  return (
    <form
      id="rsform"
      onSubmit={onSubmitEvent}
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
