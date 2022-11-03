import { atom } from "nanostores";

export enum ModalType {
  CLOSED,
  CAMPAIGN,
}

// determine if modal is open and type
export const modalStore = atom<ModalType>(ModalType.CLOSED);
