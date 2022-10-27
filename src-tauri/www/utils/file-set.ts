import { fileMap } from "../stores/engine";

// mutate selected option element mutate list.
export const selectedFileOptionMutate = ({
  initialTarget,
  path,
  fileSelect: f,
}: {
  initialTarget?: string;
  path: string;
  fileSelect?: HTMLInputElement;
}) => {
  if (!fileMap.has(path)) {
    fileMap.set(path, {});
    // file select
    const fileSelect =
      f ?? (document.getElementById("target-select") as HTMLInputElement);

    if (fileSelect) {
      // run after initial target setting
      if (initialTarget) {
        const kid = "fskeys_" + initialTarget;
        const item = document.getElementById(kid);
        if (!item) {
          const cellSelect: HTMLOptionElement =
            document.createElement("option");

          cellSelect.id = kid;
          // @ts-ignore
          cellSelect.name = "fsselect";
          cellSelect.value = initialTarget;
          cellSelect.innerText = initialTarget;

          fileSelect.appendChild(cellSelect);
        }
      }

      for (const [key, _] of fileMap) {
        if (key !== initialTarget) {
          const kid = "fskeys_" + key;
          const item = document.getElementById(kid);

          if (!item) {
            const cellSelect: HTMLOptionElement =
              document.createElement("option");

            cellSelect.id = kid;
            // @ts-ignore
            cellSelect.name = "fsselect";
            cellSelect.value = key;
            cellSelect.innerText = key;

            fileSelect.appendChild(cellSelect);
          }
        }
      }
    }
  }
};
