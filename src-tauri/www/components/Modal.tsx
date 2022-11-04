import "../styles/forms.css";

import Modal from "react-modal";

import { modalStore, ModalType } from "../stores/app";
import { useStore } from "@nanostores/react";
import { SettingsBar } from "./SettingsBar";
import { CampaignCreate } from "./CampaignCreate";
import { Settings } from "./Settings";

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

  const modalTitle =
    modalState === ModalType.CAMPAIGN ? "New Campaign" : "Settings";

  return (
    <Modal
      id="campaign-create-form"
      isOpen={modalState !== ModalType.CLOSED}
      onRequestClose={closeModal}
      shouldCloseOnOverlayClick
      style={customStyles}
    >
      <div>
        <SettingsBar title={modalTitle} />
        {modalState === ModalType.CAMPAIGN ? <CampaignCreate /> : null}
        {modalState === ModalType.SETTINGS ? <Settings /> : null}
      </div>
    </Modal>
  );
};
