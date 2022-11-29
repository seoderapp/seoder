import "../styles/license.css";

import { useState } from "react";

export default function LicenseInput({ className, value }: { className?: string, value?:string }) {
  const [license, setLicense] = useState<string>(value ?? "");

  const onChangeTextEvent = (e) => {
    setLicense(e.target.value);
  };

  const disabled = license.length < 30;

  return (
    <>
      <div className={`license-form${className ? ` ${className}` : ""}`}>
        <label htmlFor="license">License Key</label>
        <input
          name="license"
          placeholder="XXXX-XXXX-XXXXX-XXXXX-XXXXXX"
          type="text"
          value={license}
          className={`license-input`}
          onChange={onChangeTextEvent}
        />
        <div className="gutter-t">
          <button
            type="submit"
            style={{ opacity: disabled ? 0.6 : 1 }}
            className={"submit"}
            disabled={disabled}
          >
            Verify License Key
          </button>
        </div>
      </div>
    </>
  );
}
