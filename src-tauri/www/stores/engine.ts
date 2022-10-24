import { map, atom, action, computed } from "nanostores";
import { CellStatus } from "../components/CampaignCell";

export type EngineProps = {
  total: number;
  valid: number;
  urls: Set<string | unknown>;
  invalidUrls: Set<string | unknown>;
  paths?: string[];
  patterns?: string[];
  status?: CellStatus;
};

export interface Engine {
  [name: string]: EngineProps;
}

// engine list
export const engines = map<Engine>();

// get engine list
export const enginesList = computed(engines, (all) => {
  const keys = Object.keys(all);

  return keys;
});

// active engine selected
export const selectedEngine = atom<string>("");

// select or deselect engine
export const selectAction = action(
  selectedEngine,
  "select",
  (store, selected) => {
    if (store.get() === selected) {
      store.set("");
    } else {
      store.set(selected);
    }
    return store.get();
  }
);

// backcompat

// files for uploading
export const fileMap = new Map();
// engine map
export const engineMap = new Map();
