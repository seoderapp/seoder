import { atom, map, computed } from "nanostores";
import { persistentAtom } from "@nanostores/persistent";

export enum ModalType {
  CLOSED,
  CAMPAIGN,
  SETTINGS,
  ANALYTICS,
  EDIT,
  CONTACTS,
}

// determine if modal is open and type
export const modalStore = atom<ModalType>(ModalType.CLOSED);

// files stores
export const fileMap = map<{ [k: string]: any }>({});

// selected file target
export const selectedFile = atom<string>();

// selected contacts for modal
export const contactsModalData = atom<{ domain: string; contacts: string[] }>();

// get invalid logs
export const fileList = computed(fileMap, (files) => {
  return Object.keys(files);
});

// selected file target
export const proxySet = atom<boolean>(false);
export const lowPowerSet = atom<boolean>(false);
export const torSet = atom<boolean>(false);

// hunterio state
export const hunterioKey = persistentAtom<string>("");
