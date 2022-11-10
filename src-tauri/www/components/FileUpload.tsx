import React, { useState } from "react";
import { selectedFileOptionMutate } from "../utils/file-set";

const onFileUpload = (event) => {
  event.preventDefault();
  const fileValue = event.target["file"].value;

  if (fileValue) {
    const url = "http://localhost:7050/upload";
    const request = new XMLHttpRequest();
    request.open("POST", url, true);
    request.onload = function () {};
    request.onerror = function () {
      // request failed
    };
    const formData = new FormData(event.target as HTMLFormElement);

    request.send(formData);

    const optimisticPath = fileValue.replace(/^.*[\\\/]/, "");

    selectedFileOptionMutate({
      path: optimisticPath,
    });
  } else {
    window.alert("Please upload a file.");
  }
};

type UploadProps = {
  label?: string;
  submitTitle?: string;
  labelClassName?: string;
  formless?: boolean;
};

const labelButtonStyle: React.CSSProperties = {
  border: "2px solid #ddd",
  borderRadius: 4,
  textAlign: "center",
  fontSize: "0.85rem",
  paddingTop: "0.5rem",
  paddingBottom: "0.5rem",
};

const labelStyle: React.CSSProperties = {
  ...labelButtonStyle,
  cursor: "pointer",
};

export const FileUpload = ({
  label,
  submitTitle,
  labelClassName = "",
  formless,
}: UploadProps) => {
  const [inputState, setInput] = useState<string>("");

  const onInputChange = (event) => {
    event.preventDefault();
    const fileValue = event.target.value;
    if (fileValue) {
      const optimisticPath = fileValue.replace(/^.*[\\\/]/, "");

      setInput(optimisticPath);
    }
  };

  const disabled = !inputState;

  if (formless) {
    return (
      <div
        id="uploadform"
        className="frame flex-row center-align"
        style={{ gap: "1rem" }}
      >
        <label
          htmlFor="uploadfile"
          className={labelClassName}
          style={labelStyle}
        >
          {label ?? "Crawl list"}
        </label>
        <input
          type="file"
          accept=".txt"
          name="file"
          onChange={onInputChange}
          id="uploadfile"
          style={{ opacity: 0, width: 5 }}
        />
        <div className="preview">
          <p id="preview">{inputState || "No files selected"}</p>
        </div>
        <button
          className={`button${disabled ? " disabled" : ""}`}
          type="button"
          style={labelButtonStyle}
          onClick={onFileUpload}
          disabled={disabled}
        >
          {submitTitle ?? "Upload"}
        </button>
      </div>
    );
  }

  return (
    <form
      id="uploadform"
      className="ph frame flex-row center-align"
      encType="multipart/form-data"
      style={{ gap: "1rem" }}
      method="post"
      onSubmit={onFileUpload}
    >
      <label htmlFor="uploadfile" className={labelClassName} style={labelStyle}>
        {label ?? "Crawl list"}
      </label>
      <input
        type="file"
        accept=".txt"
        name="file"
        id="uploadfile"
        onChange={onInputChange}
        style={{ opacity: 0, width: 5 }}
      />
      <div className="preview">
        <p id="preview">{inputState || "No files selected"}</p>
      </div>
      <button
        className={`button${disabled ? " disabled" : ""}`}
        type="submit"
        style={labelButtonStyle}
        disabled={disabled}
      >
        {submitTitle ?? "Upload"}
      </button>
    </form>
  );
};
