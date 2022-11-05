import { atom, map, computed } from "nanostores";

export enum ModalType {
  CLOSED,
  CAMPAIGN,
  SETTINGS,
  ANALYTICS,
  EDIT,
}

// determine if modal is open and type
export const modalStore = atom<ModalType>(ModalType.CLOSED);

// files stores
export const fileMap = map<{ [k: string]: any }>({});

// selected file target
export const selectedFile = atom<string>();

// get invalid logs
export const fileList = computed(fileMap, (files) => {
  return Object.keys(files);
});

// selected file target
export const proxySet = atom<boolean>(false);
export const lowPowerSet = atom<boolean>(false);
export const torSet = atom<boolean>(false);
