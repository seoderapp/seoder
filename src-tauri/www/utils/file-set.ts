// files for uploading
import { fileMap } from "../stores/app";

// mutate selected option element mutate list.
export const selectedFileOptionMutate = ({ path }: { path: string }) => {
  if (!fileMap.get()[path]) {
    fileMap.setKey(path, {});
  }
};
