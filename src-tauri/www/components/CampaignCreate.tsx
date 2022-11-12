import "../styles/forms.css";

import { socket } from "../events/sockets";
import { createEngine, engines } from "../stores/engine";
import {
  fileList,
  fileMap,
  modalStore,
  ModalType,
  selectedFile,
} from "../stores/app";
import { useState } from "react";
import { useStore } from "@nanostores/react";
import { FileUpload } from "./FileUpload";

// todo: refactor modal outside for central components
export const CampaignCreate = () => {
  const $selectedf = useStore(selectedFile);

  const [source, setSourceOnly] = useState<boolean>(true);
  const [$newfile, setInput] = useState<string>("");
  const $flist = useStore(fileList);

  const onSubmitEvent = (event) => {
    event.preventDefault();
    const eform = document.getElementById("eform");

    const engine: HTMLInputElement = eform.querySelector('input[name="ename"]');
    const epaths: HTMLInputElement = eform.querySelector(
      'input[name="epaths"]'
    );
    const epatterns: HTMLInputElement = eform.querySelector(
      'input[name="epatterns"]'
    );

    if (engine && engine.value) {
      const fileTarget = $newfile || $selectedf;

      const p = {
        name: engine.value,
        paths: epaths.value.length ? epaths.value : "/",
        patterns: epatterns.value,
        target: fileTarget,
        source,
      };

      const m = JSON.stringify(p);

      if (engines.get()[m]) {
        window.alert("Please enter a different engine name.");
      } else {
        socket.send("create-engine " + m);
        createEngine(engine.value, p);
        closeModal();
      }
    } else {
      window.alert("Please enter the engine name.");
    }
  };

  const closeModal = () => modalStore.set(ModalType.CLOSED);

  const onSetSource = () => setSourceOnly(true);

  const onRemoveSource = () => setSourceOnly(false);

  const onInputChange = (fileValue: string) => {
    if (fileValue) {
      const optimisticPath = fileValue.replace(/^.*[\\\/]/, "");

      setInput(optimisticPath);
    }
  };

  const onFileChange = (event) => {
    setInput(event.target.value);
  };

  // const onFileAdd = () => {};

  return (
    <form id="eform" onSubmit={onSubmitEvent}>
      <div className="form-container">
        <div>
          <label htmlFor="ename">Campaign Name</label>
          <input
            id="ename"
            name="ename"
            placeholder="Name"
            type="text"
            className="form-control"
          />
        </div>
        <div className="gutter">
          <label htmlFor="epatterns">Keywords</label>
          <input
            name="epatterns"
            id="epatterns"
            placeholder="bitcoin, motorcycles, *cats*"
            type="text"
            className="form-control"
          />
          <p>Words are case insensitive and can utilize regex</p>
        </div>

        <div className="row">
          <button
            className={`button ${source ? "btn-primary" : ""}`}
            onClick={onSetSource}
            style={{ borderRadius: "8px 0px 0px 8px" }}
            type={"button"}
          >
            Source Code
          </button>
          <button
            className={`button ${source ? "" : "btn-primary"}`}
            onClick={onRemoveSource}
            style={{ borderRadius: "0px 8px 8px 0px" }}
            type={"button"}
          >
            Rendered Text
          </button>
        </div>

        <p>Choose where to crawl the keywords against</p>

        <div className="gutter">
          <label htmlFor="dname">Choose domain list</label>
          <div className="flex-row center-align gap-xl" style={{ gap: "1rem" }}>
            <select
              name="target"
              id="dname"
              value={$newfile || $selectedf}
              onChange={onFileChange}
            >
              {$flist.map((key) => (
                <option key={key} id={"fsskeys_" + key} value={key}>
                  {key}
                </option>
              ))}
            </select>
            <FileUpload
              label={"Add"}
              labelClassName={""}
              formless
              onChange={onInputChange}
              preview={false}
            />
          </div>
        </div>

        <div className="optional">Optional</div>
        <div className="seperator"></div>

        <div className="seperator-sm"></div>
        <div>
          <label htmlFor="epaths">Paths</label>
          <input
            name="epaths"
            id="epaths"
            placeholder="/home, /welcome, /about"
            type="text"
            className="form-control"
          />
          <p>Choose which paths you want the crawler to find keywords in</p>
        </div>
      </div>
      <div className="gutter-t">
        <button type="submit" className="button btn-primary full-w">
          Add Campaign
        </button>
      </div>
    </form>
  );
};
