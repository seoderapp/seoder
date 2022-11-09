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
          style={labelButtonStyle}
        >
          {label ?? "Crawl list"}
        </label>
        <input
          type="file"
          accept=".txt"
          name="file"
          onChange={onInputChange}
          id="uploadfile"
          style={{ opacity: 0, width: 10 }}
        />
        <div className="preview">
          <p id="preview">
            {inputState || "No files currently selected for upload"}
          </p>
        </div>
        <button
          className="btn-primary button"
          type="button"
          style={labelButtonStyle}
          onClick={onFileUpload}
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
      <label
        htmlFor="uploadfile"
        className={labelClassName}
        style={labelButtonStyle}
      >
        {label ?? "Crawl list"}
      </label>
      <input
        type="file"
        accept=".txt"
        name="file"
        id="uploadfile"
        onChange={onInputChange}
        style={{ opacity: 0, width: 10 }}
      />
      <div className="preview">
        <p id="preview">
          {inputState || "No files currently selected for upload"}
        </p>
      </div>
      <button
        className="btn-primary button"
        type="submit"
        style={labelButtonStyle}
      >
        {submitTitle ?? "Upload"}
      </button>
    </form>
  );
};
