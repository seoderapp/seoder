import { useState, SyntheticEvent } from "react";
import "../css/license.css";

export default function LicenseInput() {
  const [license, setLicense] = useState<string>("");

  const onChangeTextEvent = (e: SyntheticEvent<HTMLFormElement>) => {
    setLicense(e.target.value);
  };

  const disabled = license.length < 30;

  const clear = () => {
    const ele = document.getElementById("settings-container");
    if (ele && ele.className !== "hidden") {
      ele.className = "hidden";
    }
  };

  return (
    <>
      <div className="license-form">
        <label htmlFor="license">License Key</label>
        <input
          name="license"
          placeholder="XXXX-XXXX-XXXXX-XXXXX-XXXXXX"
          type="text"
          className="license-input"
          onChange={onChangeTextEvent}
        />
        <div className="gutter-t">
          <button
            type="submit"
            style={{ opacity: disabled ? 0.6 : 1 }}
            className={"submit"}
            disabled={disabled}
            onClick={clear}
          >
            Verify License Key
          </button>
        </div>
      </div>
    </>
  );
}
