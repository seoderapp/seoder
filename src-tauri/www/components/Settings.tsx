import { useStore } from "@nanostores/react";
import {
  fileList,
  hunterioKey,
  lowPowerSet,
  modalStore,
  ModalType,
  proxySet,
  selectedFile,
  torSet,
} from "../stores/app";
import LicenseInput from "./LicenseInput";
import { Switch } from "./Switch";
import { socket } from "../events/sockets";
import { selectedFileOptionMutate } from "../utils/file-set";

const checkedToBool = (value: string) => {
  if (typeof value === "string") {
    return value === "on" ? true : false;
  }
  return value;
};

export const Settings = () => {
  const $flist = useStore(fileList);
  const $selectedTarget = useStore(selectedFile);
  const $tor = useStore(torSet);
  const $proxy = useStore(proxySet);
  const $lowpower = useStore(lowPowerSet);
  const $hunterio = useStore(hunterioKey);

  const onLicenseSubmit = (event) => {
    event.preventDefault();
    const ulicense = document.getElementById("ulicense");
    const slicense: HTMLInputElement = ulicense.querySelector(
      'input[name="license"]'
    );

    if (slicense && slicense.value) {
      socket.send("set-license " + slicense.value);

      modalStore.set(ModalType.CLOSED);
    } else {
      window.alert("Please enter a license.");
    }
  };

  const onFileUpload = async (event) => {
    event.preventDefault();
    const url = "http://localhost:7050/upload";
    const request = new XMLHttpRequest();
    request.open("POST", url, true);
    request.onload = function () {};
    request.onerror = function () {
      // request failed
    };
    const fileValue = event.target["file"].value;
    if (fileValue) {
      const formData = new FormData(event.target as HTMLFormElement);

      request.send(formData);
      const optimisticPath = fileValue.replace(/^.*[\\\/]/, "");

      selectedFileOptionMutate({
        path: optimisticPath,
      });
    }
  };

  const onDeleteFile = async () => {
    const cf = await window.confirm(
      `Are, you sure you want to delete the file ${$selectedTarget}?`
    );

    if (cf && $selectedTarget) {
      socket.send("delete-file " + $selectedTarget);
    }
  };

  const onChangeFile = (event) => {
    selectedFile.set(event.target.value);
    socket.send("set-list " + event.target.value);
  };

  const onProxyChange = (event) => {
    const checked = checkedToBool(event.target.checked);

    proxySet.set(checked);
    socket.send("set-proxy " + checked);
  };

  const onLowPowerChange = (event) => {
    const checked = checkedToBool(event.target.checked);

    lowPowerSet.set(checked);
    socket.send("set-buffer " + checked);
  };

  const onTorChange = (event) => {
    const checked = checkedToBool(event.target.checked);

    torSet.set(checked);
    socket.send("set-tor " + checked);
  };

  const onEditFile = () => {
    modalStore.set(ModalType.EDIT);
  };

  // set local hunter IO key
  const onHunterSubmit = (event) => {
    event.preventDefault();
    const v = event.target.hunter.value;

    hunterioKey.set(v);
  };

  return (
    <div className="form-container">
      <div>
        <div id="proxyform" className="ph frame flex-row center-align gap">
          <label htmlFor="proxy-select">Use Proxy</label>
          <Switch id="proxy-select" onChange={onProxyChange} checked={$proxy} />
        </div>

        <div id="torform" className="ph frame flex-row center-align gap">
          <label htmlFor="tor-select">Use Tor</label>
          <Switch id="tor-select" onChange={onTorChange} checked={$tor} />
        </div>

        <div id="lowpowerform" className="ph frame flex-row center-align gap">
          <label htmlFor="lowpower-select">Low Power</label>
          <Switch
            id="lowpower-select"
            onChange={onLowPowerChange}
            checked={$lowpower}
          />
        </div>

        <div className="ph frame flex-row center-align">
          <label htmlFor="target-select">Target</label>
          <div className="ph">
            <select
              name="target"
              id="target-select"
              onChange={onChangeFile}
              value={$selectedTarget}
            >
              {$flist.map((key) => {
                return (
                  <option key={key} id={"fskeys_" + key} value={key}>
                    {key}
                  </option>
                );
              })}
            </select>
          </div>
          <div className="flex align-end">
            <button
              type="button"
              className="button edit button-sm"
              onClick={onEditFile}
            >
              Edit File
            </button>
          </div>
          <div className="flex align-end">
            <div>
              <button
                type="button"
                className="button delete button-sm"
                onClick={onDeleteFile}
              >
                Delete
              </button>
            </div>
          </div>
        </div>

        <form
          id="uploadform"
          className="ph frame flex-row center-align"
          encType="multipart/form-data"
          method="post"
          onSubmit={onFileUpload}
        >
          <label htmlFor="uploadfile">Crawl list</label>
          <div className="ph">
            <input type="file" accept=".txt" name="file" id="uploadfile" />
          </div>
          <button className="btn-primary button button-sm" type="submit">
            Upload
          </button>
        </form>

        <form
          onSubmit={onHunterSubmit}
          className="ph frame flex-row center-align"
        >
          <label htmlFor="hunter">Hunter IO Key</label>
          <div className="ph">
            <input
              type="text"
              placeholder="xxx-xxx-xxx-xxx-xxx"
              name="hunter"
              className="button-sm"
              id="hunterio"
              value={$hunterio}
            />
          </div>
          <button className="btn-primary button button-sm" type="submit">
            Set Key
          </button>
        </form>
        <div
          style={{
            padding: "0.1rem 0.5rem",
            fontSize: "0.85rem",
            color: "#A1A7AD",
          }}
        >
          Get your <a href={"https://hunter.io/api-keys"}>Hunter.io API</a> key
          for prospects.
        </div>
        <form id="ulicense" className="ph" onSubmit={onLicenseSubmit}>
          <LicenseInput className="pless" />
        </form>
      </div>
    </div>
  );
};
