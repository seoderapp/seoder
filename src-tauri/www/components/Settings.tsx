import { useStore } from "@nanostores/react";
import {
  fileList,
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
import { FileUpload } from "./FileUpload";

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

  const onDeleteFile = async () => {
    const cf = await window.confirm(
      `Are, you sure you want to delete the file ${$selectedTarget}?`
    );

    if (cf && $selectedTarget) {
      socket.send("delete-file " + $selectedTarget);
    }

    if ($flist.length) {
      const nextTarget = $flist[0];

      selectedFile.set(nextTarget);

      socket.send("set-list " + nextTarget);
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

  const onIntegrationClick = (event) => {
    event.preventDefault();
    modalStore.set(ModalType.INTEGRATIONS);
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
              Edit
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

        <FileUpload label={"Add File"} />

        <div className="ph frame">
          <button
            type={"button"}
            onClick={onIntegrationClick}
            className="btn-primary button button-sm"
          >
            Integrations
          </button>
        </div>

        <form id="ulicense" className="ph" onSubmit={onLicenseSubmit}>
          <LicenseInput className="pless" />
        </form>
      </div>
    </div>
  );
};
