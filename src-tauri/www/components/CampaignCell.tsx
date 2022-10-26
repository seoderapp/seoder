import { useStore } from "@nanostores/react";
import {
  EngineProps,
  engines,
  errorLogs,
  invalidLogs,
  selectAction,
  selectedEngine,
  validLogs,
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

const onPauseEvent = (path: string) => {
  socket.send("set-stopped " + path);
};

const onStartedEvent = (path: string) => {
  socket.send("set-started " + path);
};

export enum CellStatus {
  READY = "Ready",
  PAUSED = "Paused",
  RUNNING = "Running",
  FINISHED = "Finished",
}

// determine cell status class
const cellStatusClass = (item: EngineProps): string => {
  if (item.status === CellStatus.FINISHED) {
    return "engine-status engine-status-finished";
  }
  if (item.status === CellStatus.RUNNING) {
    return "engine-status engine-status-running";
  }
  return "engine-status";
};

export const CampaignCell = ({
  item,
  path,
}: {
  key: string;
  path: string;
  item: EngineProps;
}) => {
  const $selected = useStore(selectedEngine);
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

    if (item.status === CellStatus.RUNNING) {
      return alert("Campaign already in progress");
    }

    item.invalidUrls.clear();
    item.errorUrls.clear();
    item.urls.clear();
    item.status = CellStatus.RUNNING;

    engines.notify(path);

    if ($selected === path) {
      errorLogs.set([]);
      validLogs.set([]);
      invalidLogs.set([]);
    }

    onRunEvent(path);
  };

  const onPausePress = (event) => {
    event.preventDefault();
    event.stopPropagation();

    if (item.status === CellStatus.PAUSED) {
      item.status = CellStatus.RUNNING;
      onStartedEvent(path);
    } else {
      item.status = CellStatus.PAUSED;
      onPauseEvent(path);
    }

    engines.notify(path);
  };

  const onDeletePress = (event) => {
    event.preventDefault();
    event.stopPropagation();
    onDeleteEvent(path);
    if ($selected === path) {
      selectAction(path);
    }
  };

  const engineStatusClass = cellStatusClass(item);

  const bgClass =
    item.status === CellStatus.RUNNING ? " engine-background-running" : "";

  const baseClass = path === $selected ? "cell-btn-active" : "";

  let cellRunProps = {
    title: "Run",
    icon: "/assets/unpause.svg",
    onPress: onRunPress,
  };

  if (item.status === CellStatus.RUNNING) {
    cellRunProps = {
      title: "Pause",
      icon: "/assets/pause.svg",
      onPress: onPausePress,
    };
  }

  if (item.status === CellStatus.PAUSED) {
    cellRunProps = {
      title: "Start",
      icon: "/assets/unpause.svg",
      onPress: onPausePress, // handle same method to unpause
    };
  }

  const showBar =
    item.status === CellStatus.RUNNING || item.status === CellStatus.PAUSED;

  const totalCurrentCount =
    item.invalidUrls.size + item.errorUrls.size + item.urls.size;

  return (
    <li
      className={`engine-item`}
      id={"engine_" + path}
      tabIndex="0"
      role="button"
      aria-pressed={pressed ? "true" : "false"}
      onClick={selectItem}
    >
      <div className={`cell-btn ${baseClass}${bgClass}`}>
        <div className={"row full-w"}>
          <div className={"flex"}>
            <div className={"engine-title"}>{path}</div>
            <div id={"engine_stats_" + path} className={"engine-stats"}>
              {item.valid} / {item.total}
            </div>
            <div className={"row engine-paths"}>{item?.paths}</div>
            <div className={"row gutter-t expand-width"}>
              <button className={"btn-base"} onClick={cellRunProps.onPress}>
                <img src={cellRunProps.icon} alt={""} />
                <div>{cellRunProps.title}</div>
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
          <div>
            <div
              id={"engine_status_" + path}
              className={`${engineStatusClass} gutter`}
            >
              {item.status}
            </div>
            <div className={`${showBar ? "engine-bar" : "hidden"}`}>
              <div
                style={{
                  width: `${(totalCurrentCount / item.total) * 100}%`,
                }}
              />
            </div>
          </div>
        </div>
      </div>
    </li>
  );
};
