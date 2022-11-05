import "../styles/forms.css";

import Modal from "react-modal";

import { modalStore, ModalType } from "../stores/app";
import { useStore } from "@nanostores/react";
import { SettingsBar } from "./SettingsBar";
import { CampaignCreate } from "./CampaignCreate";
import { Settings } from "./Settings";
import { Analytics } from "./Analytics";
import { EditFile } from "./EditFile";

Modal.setAppElement("#appProgram");

const customStyles = {
  content: {
    top: "50%",
    left: "50%",
    right: "auto",
    bottom: "auto",
    marginRight: "-50%",
    transform: "translate(-50%, -50%)",
    padding: 0,
  },
};

// todo: refactor modal outside for central components
export const AppModal = () => {
  const modalState = useStore(modalStore);

  const closeModal = () => {
    modalStore.set(ModalType.CLOSED);
  };

  const modalTitle = ModalType[modalState];

  const title = modalTitle === "CAMPAIGN" ? `New ${modalTitle}` : modalTitle;

  return (
    <Modal
      isOpen={modalState !== ModalType.CLOSED}
      onRequestClose={closeModal}
      shouldCloseOnOverlayClick
      style={customStyles}
    >
      <div style={{ position: "relative" }}>
        <SettingsBar title={title} />
        {modalState === ModalType.CAMPAIGN ? <CampaignCreate /> : null}
        {modalState === ModalType.SETTINGS ? <Settings /> : null}
        {modalState === ModalType.ANALYTICS ? <Analytics /> : null}
        {modalState === ModalType.EDIT ? <EditFile /> : null}
      </div>
    </Modal>
  );
};
