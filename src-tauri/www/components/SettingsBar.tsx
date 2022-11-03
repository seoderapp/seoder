import { Close } from "./Svgs/Close";
import { modalStore, ModalType } from "../stores/app";

export interface Props {
  title: string;
}

export const SettingsBar = ({ title }: Props) => {
  const onSettingsClick = () => {
    modalStore.set(ModalType.CLOSED);
    const settingsContainer = document.getElementById("settings-container");

    if (settingsContainer) {
      settingsContainer.className = "hidden";
    }
  };
  return (
    <div className="settings-bar">
      <h3>{title}</h3>
      <button onClick={onSettingsClick}>
        <Close />
      </button>
    </div>
  );
};
