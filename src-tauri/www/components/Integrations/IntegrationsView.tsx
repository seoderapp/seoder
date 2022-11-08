import { useStore } from "@nanostores/react";
import { hunterioKey, modalStore, ModalType } from "../../stores/app";
import { HunterIOSVG } from "../Svgs/HunterIO";

export const IntegrationsView = () => {
  const $hunterio = useStore(hunterioKey);

  // set local hunter IO key
  const onHunterSubmit = (event) => {
    event.preventDefault();
    const v = event.target.hunter.value;

    hunterioKey.set(v);
    modalStore.set(ModalType.CLOSED);
  };

  return (
    <div className="form-container" style={{ padding: "1rem" }}>
      <h3>Enhance your experience</h3>
      <form
        onSubmit={onHunterSubmit}
        className="ph frame flex-row center-align gap"
      >
        <HunterIOSVG />
        <label htmlFor="hunter" className="ph">
          Hunter IO Key
        </label>
        <div className="ph">
          <input
            type="text"
            placeholder="xxx-xxx-xxx-xxx-xxx"
            name="hunter"
            className="button-sm"
            id="hunterio"
            value={$hunterio}
            style={{ border: "1px solid #ccc" }}
          />
        </div>
        <button className="btn-base button-sm" type="submit">
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
    </div>
  );
};
