const socketRuntime = new WebSocket("ws://127.0.0.1:8089");

const units = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

function slowBytes(x) {
  let l = 0;
  let n = parseInt(x, 10) || 0;

  while (n >= 1024 && ++l) {
    n = n / 1024;
  }

  return n.toFixed(n < 10 && l > 0 ? 1 : 0) + " " + units[l];
}

socketRuntime.addEventListener("open", () => {
  socketRuntime.send("loop");
});

// /*
//  * List of web socket matchers * special for high performance
//  */
const estats = "{" + '"' + "stats"; // os stats
// const bftc = "{" + '"' + "buffer" + '"'; // buffer
// const ptp = "{" + '"' + "pengine" + '"' + ":" + '"'; // engine
// const selectFile = "{" + '"' + "fpath" + '"' + ":" + '"'; // selected file
// const ptpe = "{" + '"' + "epath" + '"' + ":" + '"'; // engine created
// const dfpath = "{" + '"' + "dfpath" + '"'; // deleted file
// const deptc = "{" + '"' + "depath" + '"'; // deleted campaign
// const cfin = "{" + '"' + "finished" + '"'; // crawl finished
// const vpaths = "{" + '"' + "path" + '"' + ":" + '"'; // valid paths
// const ipaths = "{" + '"' + "invalid_path" + '"' + ":" + '"'; // invalid paths
// const epaths = "{" + '"' + "error_path" + '"' + ":" + '"'; // errors paths
// const vlicense = "{" + '"' + "license"; // valid license;
// const cstarted = "{" + '"' + "started"; // started campaigns from pause
// const cpaused = "{" + '"' + "paused"; // paused campaigns from pause

// // defaults for program entry
// let defaultOptionSet = false;
// let firstCheck = false;

function eventSub(event) {
  const data = event.data;

  if (data && data?.startsWith(estats)) {
    const { stats } = JSON.parse(data);
    const {
      cpu_usage,
      // load_avg_min,
      network_received,
      network_transmited,
      network_total_transmitted,
      memory_free,
      // memory_used,
      memory_total,
    } = stats;

    const networkReceived = `${slowBytes(network_received)} / s`;
    const networkTransmited = `${slowBytes(network_transmited)} / s`;
    const networkTotal = `${slowBytes(network_total_transmitted)}`;

    postMessage(
      JSON.stringify({
        stats: Object.assign({}, stats, {
          network_received: networkReceived,
          network_transmited: networkTransmited,
          network_total_transmitted: networkTotal,
        }),
      })
    );
  } else {
    postMessage(data);
  }

  // // license handling
  // if (raw.startsWith(vlicense)) {
  //   const data = JSON.parse(raw);
  //   const { license } = data || {};

  //   // set local storage of license enabled and display view
  //   if (license) {
  //     localStorage.setItem("authed", "true");
  //   } else {
  //     localStorage.removeItem("authed");

  //     if (firstCheck) {
  //       window.alert("License is invalid");
  //     }
  //   }

  //   if (!firstCheck) {
  //     firstCheck = true;
  //   }
  // }

  // if (raw.startsWith(cfin)) {
  //   const data = JSON.parse(raw);
  //   const { finished, time } = data || {};

  //   // setStatus(finished, CellStatus.FINISHED);
  //   console.log(time);
  //   return;
  // }

  // if (raw.startsWith(ptp)) {
  //   const np = JSON.parse(raw);
  //   const { pengine, ploc } = np || {};
  //   // const eitem = engines.get()[pengine];

  //   // if (eitem) {
  //   //   eitem.total = ploc ?? eitem.total ?? 0;

  //   //   engines.notify(pengine);

  //   //   if (determineFinished(eitem)) {
  //   //     setStatus(pengine, CellStatus.FINISHED);
  //   //   }
  //   // }

  //   return;
  // }

  // if (raw.startsWith(epaths)) {
  //   const np = JSON.parse(raw);
  //   const { error_path, error_url } = np || {};
  //   // const item = engines.get()[error_path];

  //   // if (item && error_url && item.errorUrls && !item.errorUrls.has(error_url)) {
  //   //   item.errorUrls.add(error_url);

  //   //   if (selectedEngine.get() === error_path) {
  //   //     addErrorLogs(error_url);
  //   //   }

  //   //   if (determineFinished(item)) {
  //   //     setStatus(error_path, CellStatus.FINISHED);
  //   //   }

  //   //   engines.notify(error_path);
  //   // }

  //   return;
  // }

  // if (raw.startsWith(ipaths)) {
  //   const np = JSON.parse(raw);

  //   const { invalid_url: url, invalid_path: path } = np || {};

  //   // const item = engines.get()[path];

  //   // if (item && item.invalidUrls && !item.invalidUrls.has(url)) {
  //   //   item.invalidUrls.add(url);

  //   //   if (selectedEngine.get() === path) {
  //   //     addInvalidLogs(url);
  //   //   }
  //   //   if (determineFinished(item)) {
  //   //     setStatus(path, CellStatus.FINISHED);
  //   //   }
  //   // }

  //   return;
  // }

  // if (raw.startsWith(vpaths)) {
  //   const np = JSON.parse(raw);
  //   const { url, path } = np || {};

  //   // const item = engines.get()[path];
  //   // if (item && url && item.urls && !item.urls.has(url)) {
  //   //   item.urls.add(url);
  //   //   if (selectedEngine.get() === path) {
  //   //     addValidLogs(url);
  //   //   }
  //   //   if (determineFinished(item)) {
  //   //     setStatus(path, CellStatus.FINISHED);
  //   //   }
  //   // }

  //   return;
  // }

  // if (raw.startsWith(ptpe)) {
  //   const np = JSON.parse(raw);
  //   const path = np && np.epath;

  //   // if (!engines.get()[path]) {
  //   //   const engineSource = {
  //   //     total: 0,
  //   //     urls: new Set(),
  //   //     invalidUrls: new Set(),
  //   //     errorUrls: new Set(),
  //   //     patterns: np?.patterns,
  //   //     paths: np?.paths,
  //   //     status: CellStatus.READY,
  //   //     sourceCode: np?.source_match ?? true,
  //   //   };

  //   //   engines.setKey(path, engineSource);
  //   // }
  //   return;
  // }

  // if (raw.startsWith(cstarted)) {
  //   const np = JSON.parse(raw);
  //   const path = np && np.path;

  //   // const item = engines.get()[path];

  //   // if (item) {
  //   //   setStatus(path, CellStatus.RUNNING);
  //   // }
  //   return;
  // }

  // if (raw.startsWith(cpaused)) {
  //   const np = JSON.parse(raw);
  //   const path = np && np.path;

  //   // const item = engines.get()[path];

  //   // if (item) {
  //   //   setStatus(path, CellStatus.PAUSED);
  //   // }
  //   return;
  // }

  // if (raw.startsWith(dfpath)) {
  //   const np = JSON.parse(raw);
  //   const path = np && np.dfpath;

  //   // if (fileMap.get()[path]) {
  //   //   const kid = "fskeys_" + path;
  //   //   const cell = document.getElementById(kid);
  //   //   cell.remove();
  //   //   fileMap.setKey(path, undefined);
  //   // }
  //   return;
  // }

  // // delete engine
  // if (raw.startsWith(deptc)) {
  //   const np = JSON.parse(raw);
  //   const path = np && np.depath;

  //   // if (engines.get()[path]) {
  //   //   engines.setKey(path, undefined);
  //   // }
  //   return;
  // }
}

socketRuntime.addEventListener("message", eventSub);

export {};
