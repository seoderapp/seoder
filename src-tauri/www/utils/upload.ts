// todo: invalidate get cache
export const onFileUploadEvent = async (
  fileName: string,
  fileValue: string,
  options?: { url?: string; cacheUrl?: string }
) => {
  const { url, cacheUrl } = options ?? {
    url: "http://localhost:7050/upload",
    cacheUrl: "http://localhost:7050/download/files/",
  };

  const request = new XMLHttpRequest();
  request.open("POST", url, true);
  request.onload = function () {};
  request.onerror = function () {};

  if (fileValue) {
    const form = new FormData();

    const blob = new Blob([fileValue.trim()], {
      type: "text/plain",
    });

    const file = new File([blob], fileName, {
      type: "text/plain",
    });

    form.append("file", file);

    request.send(form);

    try {
      const cache = await caches.open("v1");
      await cache.delete(cacheUrl + fileName);
    } catch (e) {
      console.error(e);
    }
  }
};
