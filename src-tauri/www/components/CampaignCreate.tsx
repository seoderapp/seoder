import "../styles/forms.css";

import { socket } from "../events/sockets";
import { engines } from "../stores/engine";
import { fileList, modalStore, ModalType } from "../stores/app";
import { useState } from "react";
import { useStore } from "@nanostores/react";

// todo: refactor modal outside for central components
export const CampaignCreate = () => {
  const [source, setSourceOnly] = useState<boolean>(true);
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

    const etarget: HTMLInputElement = eform.querySelector(
      'select[name="target"]'
    );

    if (engine && engine.value) {
      const m = JSON.stringify({
        name: engine.value,
        paths: epaths.value.length ? epaths.value : "/",
        patterns: epatterns.value,
        source,
        target: etarget.value,
      });

      if (engines.get()[m]) {
        window.alert("Please enter a different engine name.");
      } else {
        socket.send("create-engine " + m);
        socket.send("list-engines"); // todo: send new engine created on submit or add optimistic update
        closeModal();
      }
    } else {
      window.alert("Please enter the engine name.");
    }
  };

  const closeModal = () => modalStore.set(ModalType.CLOSED);

  const onSetSource = () => setSourceOnly(true);

  const onRemoveSource = () => setSourceOnly(false);

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

        <div className="gutter" style={{ paddingBottom: "1.2rem" }}>
          <label htmlFor="dname">Choose domain list</label>
          <div className="ph flex-row center-align">
            <div>
              <select name="target" id="dname">
                {$flist.map((key) => {
                  return (
                    <option key={key} id={"fsskeys_" + key} value={key}>
                      {key}
                    </option>
                  );
                })}
              </select>
            </div>
            {/* <div className="flex">
              <button
                type="button"
                className="button edit button-sm"
                onClick={onFileAdd}
              >
                Add
              </button>
            </div> */}
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
