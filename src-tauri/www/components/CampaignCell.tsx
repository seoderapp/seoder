import { useMemo, useState, useCallback } from "react";
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
import { socket } from "../events/sockets";
import { KeyWords } from "./Svgs/KeyWords";
import { Folder } from "./Svgs/Folder";
import {
  etemplates,
  modalStore,
  ModalType,
  selectedTemplateCreate,
} from "../stores/app";

declare global {
  var __TAURI__: any;
}

const onExportEvent = async (path: string) => {
  const filePath = await window.__TAURI__.dialog.save({
    multiple: false,
    filters: [
      {
        name: path,
        extensions: ["txt", "csv"],
      },
    ],
  });

  if (filePath) {
    const request = new XMLHttpRequest();

    // TODO: Post endpoint to write to file instead of get
    request.open(
      "GET",
      "http://localhost:7050/download/engines/" + path + "/valid/links.txt",
      true
    );

    request.onload = async function () {
      if (request.response) {
        const data = filePath.endsWith(".csv")
          ? `domain\n${request.response
              .split("\n")
              .map((line) => line.split(/\s+/).join(","))
              .join("\n")}`
          : request.response;
        await window.__TAURI__.fs.writeTextFile(filePath, data);
      }
    };

    request.onerror = function () {
      alert("Issue exporting the file.");
    };

    request.send(new FormData());
  }
};

const onDeleteEvent = (path: string) => {
  engines.setKey(path, undefined);

  socket.send("delete-engine " + path);
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
  item: itemMap,
  path,
}: {
  path: string;
  item: string;
}) => {
  const $selected = useStore(selectedEngine);
  const $engines = useStore(engines);
  const [pressed, setPressed] = useState<boolean>();

  const item = $engines[itemMap]

  const selectItem = useCallback(
    (event) => {
      event.preventDefault();
      setPressed(true);
      selectAction(path);
      setPressed(false);
    },
    [setPressed]
  );

  const onExportEventPress = useCallback(
    async (event) => {
      event.preventDefault();
      event.stopPropagation();
      await onExportEvent(path);
    },
    [path]
  );

  const onRunPress = useCallback(
    (event) => {
      event.preventDefault();
      event.stopPropagation();

      if (item.status === CellStatus.RUNNING) {
        return alert("Campaign already in progress");
      }

      if (item.status !== CellStatus.PAUSED) {
        if (selectedEngine.get() === path) {
          errorLogs.set([]);
          validLogs.set([]);
          invalidLogs.set([]);
        }
        item.invalidUrls.clear();
        item.errorUrls.clear();
        item.urls.clear();
      }

      item.status = CellStatus.RUNNING;

      engines.notify(path);
      socket.send("run-campaign " + path);
    },
    [item.status, path]
  );

  const onPausePress = useCallback(
    (event) => {
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
    },
    [item.status, path]
  );

  const onDeletePress = useCallback(
    async (event) => {
      event.preventDefault();
      event.stopPropagation();

      const confirm = await window.confirm(
        "Are you sure you want to delete this campaign?"
      );

      if (confirm) {
        onDeleteEvent(path);

        if (etemplates.get()[path]) {
          etemplates.setKey(path, undefined);
        }
      }
    },
    [path]
  );

  const onClickEmailTemplate = useCallback(
    async (event) => {
      event.preventDefault();
      event.stopPropagation();

      selectedTemplateCreate.set(path);
      modalStore.set(ModalType.EMAIL);
    },
    [path]
  );

  const engineStatusClass = cellStatusClass(item);

  const baseClass = path === $selected ? "cell-btn-active" : "";
  const itemStatus = item.status;
  const bgClass =
    itemStatus === CellStatus.RUNNING ? " engine-background-running" : "";

  const cellRunProps = useMemo(() => {
    if (itemStatus === CellStatus.RUNNING) {
      return {
        title: "Pause",
        icon: "/assets/pause.svg",
        onPress: onPausePress,
      };
    }

    if (itemStatus === CellStatus.PAUSED) {
      return {
        title: "Start",
        icon: "/assets/unpause.svg",
        onPress: onPausePress, // handle same method to unpause
      };
    }

    return {
      title: "Run",
      icon: "/assets/unpause.svg",
      onPress: onRunPress,
    };
  }, [itemStatus]);

  const showBar =
    itemStatus === CellStatus.RUNNING || itemStatus === CellStatus.PAUSED;

  const pagePaths = item.paths;
  const pagePatterns = item.patterns;

  const paths = useMemo(() => {
    if (pagePaths && Array.isArray(item.paths)) {
      return pagePaths.join(",");
    }
    return pagePaths;
  }, [pagePaths]);

  const pats = useMemo(() => {
    if (pagePatterns && Array.isArray(item.patterns)) {
      return pagePatterns.join(",");
    }
    return pagePatterns;
  }, [pagePatterns]);

  const totalCurrentCount = item.invalidUrls.size + item.errorUrls.size + item.urls.size;

  return (
    <li
      className={`engine-item`}
      id={"engine_" + path}
      tabIndex={0}
      role="button"
      aria-pressed={pressed ? "true" : "false"}
      onClick={selectItem}
    >
      <div className={`cell-btn ${baseClass}${bgClass}`}>
        <div className={"full-w"}>
          <div className={"flex row"}>
            <div className={"flex"}>
              <div className={"engine-title"}>{path}</div>
              <div id={"engine_stats_" + path} className={"engine-stats"}>
                {item.urls.size} / {item.total}
              </div>
              <div className="gutter-t flex gap-sm">
                <div className={"row campaign-paths"}>
                  <Folder /> {paths}
                </div>
                <div className={"row campaign-paths"}>
                  <KeyWords /> {pats}
                </div>
              </div>
            </div>
            <div>
              <div
                id={"engine_status_" + path}
                className={`${engineStatusClass} gutter`}
              >
                {item.status}
              </div>
              <div
                style={{
                  justifyContent: "flex-end",
                  display: "flex",
                  padding: "0.02rem 0",
                }}
              >
                <button
                  onClick={onClickEmailTemplate}
                  type="button"
                  style={{ backgroundColor: "transparent" }}
                >
                  <svg viewBox="0 0 24 24" width="16" height="16">
                    <title>Email Template</title>
                    <path
                      fill="none"
                      stroke="#000"
                      strokeWidth="2"
                      d="M1 4h22v16H1V4zm0 1 11 8.5L23 5"
                    ></path>
                  </svg>
                </button>
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
        <div className={"row gutter-t expand-width"}>
          <button className={"btn-base"} onClick={cellRunProps.onPress}>
            <img src={cellRunProps.icon} alt={""} />
            <div>{cellRunProps.title}</div>
          </button>
          <button className={"btn-base"} onClick={onExportEventPress}>
            <img src={"/assets/export.svg"} alt={""} />
            <div>Export</div>
          </button>
          <button className={"btn-base active-delete"} onClick={onDeletePress}>
            <img src={"/assets/trashcan.svg"} alt={""} />
            <div>Delete</div>
          </button>
        </div>
      </div>
    </li>
  );
};
