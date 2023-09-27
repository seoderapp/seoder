import { map, atom, action, computed } from "nanostores";
import { CellStatus } from "../components/CampaignCell";

export type EngineProps = {
  total: number;
  urls: Set<string | unknown>;
  invalidUrls: Set<string | unknown>;
  errorUrls: Set<string | unknown>;
  paths?: string[];
  patterns?: string[];
  status: CellStatus;
  contacts?: Map<string, { contacts: string[] }>;
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
export const invalidLogs = atom<string[]>([]);
export const errorLogs = atom<string[]>([]);

export function addValidLogs(next: string) {
  validLogs.set([...validLogs.get(), next]);
}

export function addInvalidLogs(next: string) {
  invalidLogs.set([...invalidLogs.get(), next]);
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

// get invalid logs
export const selectedInvalid = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected]?.invalidUrls;
  }
);

// get valid logs
export const selectedValid = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected]?.urls;
  }
);

// get errors logs
export const selectedErrors = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected]?.errorUrls;
  }
);

export const selectedContacts = computed(
  [engines, selectedEngine],
  (eng, selected) => {
    return eng[selected] && eng[selected]?.contacts;
  }
);

// select or deselect engine
export const selectAction = action(
  selectedEngine,
  "select",
  (store, selected: string) => {
    if (store.get() === selected) {
      store.set("");
    } else {
      store.set(selected);
      const valid = Array.from(selectedValid.get() ?? []) as string[];
      const invalid = Array.from(selectedInvalid.get() ?? []) as string[];
      const errors = Array.from(selectedErrors.get() ?? []) as string[];

      validLogs.set(valid);
      invalidLogs.set(invalid);
      errorLogs.set(errors);
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

// standard methods until moved to action
export const createEngine = action(
  engines,
  "create",
  (store, path: string, np: any = {}) => {
    const item = store.get()[path];

    if (item) {
      // merge all keys
      Object.keys(np).forEach((key) => {
        const ikey = item[key];

        if (
          !ikey ||
          (Array.isArray(ikey) && !ikey.length && Array.isArray(np[key]))
        ) {
          const it = np[key];

          if (
            typeof it === "string" &&
            (key === "patterns" || key === "paths")
          ) {
            // convert to array
            item[key] = it.split(",");
          } else {
            item[key] = it;
          }
        }
      });

      store.notify(path);
    } else {
      const engineSource = {
        total: np?.ploc ?? 0,
        urls: new Set(),
        invalidUrls: new Set(),
        errorUrls: new Set(),
        // email contacts
        contacts: new Map(),
        patterns: np?.patterns,
        paths: np?.paths,
        status: CellStatus.READY,
        sourceCode: np?.source_match ?? true,
        // todo social media contacts
      };

      store.setKey(path, engineSource);
    }
  }
);
