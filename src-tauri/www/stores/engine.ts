import { map, atom, action, computed } from "nanostores";
import { CellStatus } from "../components/CampaignCell";

export type EngineProps = {
  total: number;
  valid: number;
  urls: Set<string | unknown>;
  invalidUrls: Set<string | unknown>;
  paths?: string[];
  patterns?: string[];
  status: CellStatus;
};

export interface Engine {
  [name: string]: EngineProps;
}

// engine list
export const engines = map<Engine>();

// get engine list
export const enginesList = computed(engines, (all) => Object.keys(all));

// active engine selected
export const selectedEngine = atom<string>("");

// valid list of logs
export const validLogs = atom<string[]>([]);
export const errorLogs = atom<string[]>([]);

export function addValidLogs(next: string) {
  validLogs.set([...validLogs.get(), next]);
}

export function addErrorLogs(next: string) {
  errorLogs.set([...errorLogs.get(), next]);
}

// get active engine selected
export const selectedItem = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return selected && eng[selected];
  }
);

// get errors logs
export const selectedErrors = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected]?.invalidUrls;
  }
);

// get errors logs
export const selectedValid = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected]?.urls;
  }
);

// select or deselect engine
export const selectAction = action(
  selectedEngine,
  "select",
  (store, selected) => {
    if (store.get() === selected) {
      store.set("");
    } else {
      store.set(selected);
      const valid = Array.from(selectedValid.get() ?? []) as string[];
      const invalid = Array.from(selectedErrors.get() ?? []) as string[];

      validLogs.set(valid);
      errorLogs.set(invalid);
    }

    return store.get();
  }
);

// set engine item status
export const setStatus = action(
  engines,
  "setStatus",
  (store, path: string, status: CellStatus) => {
    const item = store.get()[path];

    if (item) {
      item.status = status;
      store.notify(path);
    }
  }
);

// files for uploading
export const fileMap = new Map();
