import { useEffect, useRef } from "react";
import { useStore } from "@nanostores/react";

import lx from "@lexical/react/LexicalComposer";
import pk from "@lexical/react/LexicalPlainTextPlugin";
import epkg from "@lexical/react/LexicalContentEditable.js";
import hpkg from "@lexical/react/LexicalHistoryPlugin.js";
import rpkg from "@lexical/react/LexicalComposerContext";

import pkg from "lexical";

import { modalStore, ModalType, selectedFile } from "../stores/app";
import { onFileUploadEvent } from "../utils/upload";

const { $getRoot, $createParagraphNode, $createTextNode } = pkg;
const { HistoryPlugin } = hpkg;
const { ContentEditable } = epkg;
const { PlainTextPlugin } = pk;
const { LexicalComposer } = lx;
const { useLexicalComposerContext } = rpkg;

const editorConfig = {
  namespace: "seoder_files",
  onError: () => {},
};

const downloadFile = async (path) => {
  const url = "http://localhost:7050/download/files/" + path;
  const request = new XMLHttpRequest();

  return new Promise(async (resolve) => {
    request.open("GET", url, true);

    request.onload = function () {
      resolve(request.response);
    };

    request.onerror = function () {
      resolve("");
    };

    request.send(new FormData());
  });
};

function CustomEditPlugin() {
  const [editor] = useLexicalComposerContext();
  const $selectedFile = useStore(selectedFile);

  const currentFile = useRef<string>("");

  useEffect(() => {
    // todo: move to on edit modal press and background
    if ($selectedFile !== currentFile.current) {
      currentFile.current = $selectedFile;
      downloadFile($selectedFile).then((data: string) => {
        if (data) {
          editor.update(() => {
            const root = $getRoot();
            const paragraphNode = $createParagraphNode();
            const textNode = $createTextNode(data);

            paragraphNode.append(textNode);
            root.append(paragraphNode);
          });
        }
      });
    }
  }, [$selectedFile, editor]);

  return null;
}

function CustomSubmitPlugin() {
  const [editor] = useLexicalComposerContext();
  const $selectedFile = useStore(selectedFile);

  const onSubmitText = async () => {
    editor._editorState.read(async () => {
      // Read the contents of the EditorState here.
      const root = $getRoot();

      const text = root.__cachedText;

      await onFileUploadEvent($selectedFile, text);

      modalStore.set(ModalType.CLOSED);
    });
  };

  return (
    <button
      onClick={onSubmitText}
      type={"button"}
      style={{ width: "100%" }}
      className={"button btn-primary w-full"}
    >
      Save Changes
    </button>
  );
}

export const EditFile = () => {
  return (
    <div>
      <LexicalComposer initialConfig={editorConfig}>
        <div className="editor-container">
          <PlainTextPlugin
            contentEditable={<ContentEditable className="editor-input" />}
            placeholder={
              <div className="editor-placeholder">Enter some plain text...</div>
            }
            ErrorBoundary={null}
          />
          <HistoryPlugin />
          <CustomEditPlugin />
        </div>
        <CustomSubmitPlugin />
      </LexicalComposer>
    </div>
  );
};
