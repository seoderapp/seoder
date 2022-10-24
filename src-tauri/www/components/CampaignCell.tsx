import { useStore } from "@nanostores/react";
import {
  EngineProps,
  engines,
  selectAction,
  selectedEngine,
  engineMap,
  setStatus,
} from "../stores/engine";
import { useState } from "react";
import { socket } from "../events/sockets";

declare global {
  var __TAURI__: any;
}

const onExportEvent = async (path: string) => {
  const filePath = await window.__TAURI__.dialog.save({
    multiple: false,
    filters: [
      {
        name: path,
        extensions: ["txt"],
      },
    ],
  });

  const url = "http://localhost:7050/download/" + path + "/valid/links.txt";
  const request = new XMLHttpRequest();

  // todo: Post endpoint to write to file instead of get
  request.open("GET", url, true);

  request.onload = async function () {
    if (filePath && request.response) {
      await window.__TAURI__.fs.writeTextFile(filePath, request.response);
    }
  };

  request.onerror = function () {
    alert("Issue downloading the file.");
  };

  request.send(new FormData());
};

const onDeleteEvent = (path: string) => {
  engines.setKey(path, undefined);

  socket.send("delete-engine " + path);
};

const onRunEvent = (path: string) => {
  socket.send("run-campaign " + path);
};

export enum CellStatus {
  READY = "Ready",
  RUNNING = "Running",
  FINISHED = "Finished",
}

export const CampaignCell = ({
  item,
  path,
}: {
  key: string;
  path: string;
  item: EngineProps;
}) => {
  const selected = useStore(selectedEngine);
  const [pressed, setPressed] = useState<boolean>();

  const selectItem = (event) => {
    event.preventDefault();
    setPressed(true);
    selectAction(path);
    setPressed(false);
  };

  const onExportEventPress = async (event) => {
    event.preventDefault();
    event.stopPropagation();
    await onExportEvent(path);
  };

  const onRunPress = (event) => {
    event.preventDefault();
    event.stopPropagation();
    item.invalidUrls.clear();
    item.urls.clear();

    engines.setKey(path, {
      ...item,
      urls: new Set(),
      invalidUrls: new Set(),
      status: CellStatus.RUNNING,
    });

    onRunEvent(path);
  };

  const onDeletePress = (event) => {
    event.preventDefault();
    event.stopPropagation();
    onDeleteEvent(path);
    if (selected === path) {
      selectAction(path);
    }
  };

  const engineStatusClass =
    item.status === CellStatus.FINISHED
      ? " engine-status-finished"
      : item.status === CellStatus.RUNNING
      ? " engine-status-running"
      : "";

  return (
    <li
      className={"engine-item"}
      id={"engine_" + path}
      tabIndex="0"
      role="button"
      aria-pressed={pressed ? "true" : "false"}
      onClick={selectItem}
    >
      <div
        className={path === selected ? "cell-btn cell-btn-active" : "cell-btn"}
      >
        <div className={"row full-w"}>
          <div className={"flex"}>
            <div className={"engine-title"}>{path}</div>
            <div id={"engine_stats_" + path} className={"engine-stats"}>
              {item.valid} / {item.total}
            </div>
            <div className={"row engine-paths"}>{item?.paths}</div>
            <div className={"row gutter-t full-w-120"}>
              <button className={"btn-base"} onClick={onRunPress}>
                <img src={"/assets/unpause.svg"} alt={""} />
                <div>Run</div>
              </button>
              <button className={"btn-base"} onClick={onExportEventPress}>
                <img src={"/assets/export.svg"} alt={""} />
                <div>Export</div>
              </button>
              <button
                className={"btn-base active-delete"}
                onClick={onDeletePress}
              >
                <img src={"/assets/trashcan.svg"} alt={""} />
                <div>Delete</div>
              </button>
            </div>
          </div>
          <div
            id={"engine_status_" + path}
            className={`engine-status${engineStatusClass}`}
          >
            {item.status}
          </div>
        </div>
      </div>
    </li>
  );
};
