import React, { useState } from "react";
import { selectedFileOptionMutate } from "../utils/file-set";

export const onFileUpload = async (event) => {
  event?.preventDefault();

  const newValue = document?.getElementById("uploadform");
  const input = newValue?.querySelector("input") as HTMLInputElement;

  // validate input files
  if (input && input?.files?.length) {
    const url = "http://localhost:7050/upload";
    const request = new XMLHttpRequest();
    request.open("POST", url, true);
    request.onload = function () {};
    request.onerror = function () {
      // request failed
    };
    const formData = new FormData();

    const newPath = input.value;

    const optimisticPath = newPath.replace(/^.*[\\\/]/, "");

    const blob = new Blob([await input.files[0].arrayBuffer()], {
      type: "text/plain",
    });

    const file = new File([blob], optimisticPath, {
      type: "text/plain",
    });

    formData.append("file", file);

    request.send(formData);

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
  onChange?(x?: any): any;
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
  onChange,
}: UploadProps) => {
  const [inputState, setInput] = useState<string>("");

  const onInputChange = async (event) => {
    event.preventDefault();
    const fileValue = event.target.value;
    if (fileValue) {
      const optimisticPath = fileValue.replace(/^.*[\\\/]/, "");

      setInput(optimisticPath);

      if (onChange) {
        onChange(optimisticPath);
        await onFileUpload(event);
      }
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
