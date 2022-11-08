import { useEffect, useRef } from "react";
import { useStore } from "@nanostores/react";

import lx from "@lexical/react/LexicalComposer";
import pk from "@lexical/react/LexicalRichTextPlugin";
import epkg from "@lexical/react/LexicalContentEditable.js";
import hpkg from "@lexical/react/LexicalHistoryPlugin.js";
import rpkg from "@lexical/react/LexicalComposerContext";

import pkg from "lexical";

import {
  etemplates,
  modalStore,
  ModalType,
  selectedTemplateCreate,
} from "../stores/app";
import rc from "@lexical/rich-text";
import lli from "@lexical/list";
import lxc from "@lexical/code";

const { $getRoot, $createParagraphNode, $createTextNode } = pkg;
const { HistoryPlugin } = hpkg;
const { ContentEditable } = epkg;
const { RichTextPlugin } = pk;
const { LexicalComposer } = lx;
const { useLexicalComposerContext } = rpkg;
const { HeadingNode, QuoteNode } = rc;
const { ListItemNode, ListNode } = lli;
const { CodeHighlightNode, CodeNode } = lxc;

const editorConfig = {
  namespace: "seoder_files",
  onError: () => {},
  nodes: [
    HeadingNode,
    ListNode,
    ListItemNode,
    QuoteNode,
    CodeNode,
    CodeHighlightNode,
    // TableNode,
    // TableCellNode,
    // TableRowNode,
    // AutoLinkNode,
    // LinkNode,
  ],
};

function CustomEditPlugin() {
  const [editor] = useLexicalComposerContext();
  const $selected = useStore(selectedTemplateCreate);

  const currentFile = useRef<string>("");

  useEffect(() => {
    // todo: move to on edit modal press and background
    if ($selected !== currentFile.current) {
      currentFile.current = $selected;

      if ($selected) {
        editor.update(() => {
          const root = $getRoot();
          const paragraphNode = $createParagraphNode();

          const item = etemplates.get()[$selected];

          if (item) {
            const searchParams = new URLSearchParams(item);
            const body = searchParams.get("body");

            const textNode = $createTextNode(body);

            paragraphNode.append(textNode);
            root.append(paragraphNode);
          }
        });
      }
    }
  }, [$selected, editor]);

  return null;
}

function CustomSubmitPlugin() {
  const [editor] = useLexicalComposerContext();
  const $selected = useStore(selectedTemplateCreate);

  const onSubmitText = async () => {
    // todo: configm name
    editor._editorState.read(async () => {
      // Read the contents of the EditorState here.
      const root = $getRoot();

      const text = root.__cachedText;

      const subject = document.getElementById(
        "email-subject"
      ) as HTMLInputElement;

      etemplates.setKey(
        $selected,
        `?subject=${subject?.value ?? ""}&body=${text}`
      );
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

const EmailSubject = () => {
  const $selected = useStore(selectedTemplateCreate);
  const $etemplates = useStore(etemplates);

  const item = $etemplates[$selected];
  const searchParams = new URLSearchParams(item);

  const subject = searchParams.get("subject");

  return (
    <div
      className="settings-bar"
      style={{
        justifyContent: "flex-start",
        gap: "1rem",
        padding: "0.4rem 0.8rem",
      }}
    >
      <label htmlFor="email-subject">Subject</label>
      <input
        placeholder="Hey, just reaching out for.."
        id="email-subject"
        type="text"
        className="px"
        style={{
          border: "2px solid #ccc",
          borderRadius: "0.5rem",
          padding: "0.3rem 0.5rem",
        }}
        value={subject}
      ></input>
    </div>
  );
};

export const EmailCreate = () => {
  return (
    <div style={{ overflow: "hidden" }}>
      <EmailSubject />
      <div style={{ position: "relative", overflow: "auto" }}>
        <LexicalComposer initialConfig={editorConfig}>
          <div className="editor-container">
            <RichTextPlugin
              contentEditable={<ContentEditable className="editor-input" />}
              placeholder={
                <div className="editor-placeholder">
                  Enter the body template
                </div>
              }
              ErrorBoundary={null}
            />
            <HistoryPlugin />
            <CustomEditPlugin />
          </div>
          <CustomSubmitPlugin />
        </LexicalComposer>
      </div>
    </div>
  );
};
