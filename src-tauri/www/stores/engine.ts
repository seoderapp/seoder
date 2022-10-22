import { map, atom, action } from "nanostores";

export const engines = map({});

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
