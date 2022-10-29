// files for uploading
export const fileMap = new Map();

export const baseConfig = {
  initialTarget: "",
};

// set the initial file in the ui
let setConf = false;

// mutate selected option element mutate list.
export const selectedFileOptionMutate = ({
  path,
  fileSelect: f,
}: {
  path: string;
  fileSelect?: HTMLInputElement;
}) => {
  if (!fileMap.has(path)) {
    fileMap.set(path, {});
    // file select

    const fileSelect =
      f ?? (document.getElementById("target-select") as HTMLInputElement);

    for (const [key] of fileMap) {
      const kid = "fskeys_" + key;
      const item = document.getElementById(kid);

      if (!item) {
        const cellSelect = document.createElement("option");
        cellSelect.id = kid;
        // @ts-ignore
        cellSelect.name = "fsselect";
        cellSelect.value = key;
        cellSelect.innerText = key;
        fileSelect?.appendChild(cellSelect);
      }
    }

    if (!setConf) {
      let i = 0;
      for (let item of fileSelect.options) {
        if (item.value === baseConfig.initialTarget) {
          fileSelect.options[i].selected = true;
          setConf = true;
          break;
        }
        i++;
      }
    }
  }
};
