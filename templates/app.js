// check if socket already ready
const socket = new WebSocket("ws://127.0.0.1:8080");
// feed loop
const socketRuntime = new WebSocket("ws://127.0.0.1:8089");

socketRuntime.addEventListener("open", () => {
  socketRuntime.send("loop");
});

socket.addEventListener("open", () => {
  socket.send("list-engines");
  socket.send("list-totals");
  socket.send("list-files");
  socket.send("feed");
  socket.send("config");

  setTimeout(() => {
    socket.send("list-campaign-stats");
  });
});

const stats = document.getElementById("feed-stats");
const cpu = document.getElementById("cpu-stats");
const cpua = document.getElementById("cpu-stats-average");
const netstats = document.getElementById("network-stats");
const memstats = document.getElementById("memory-stats");
const logfeed = document.getElementById("feed-log");
const list = document.getElementById("engine-list");
const settingsContainer = document.getElementById("settings-container");
const settingsBtn = document.getElementById("settings-button");
const settingsBtnCls = document.getElementById("settings-button-close");
const campaignBtnCls = document.getElementById("campaign-button-close");

const newCampaignButton = document.getElementById("new-campaign-button");
const campaignCreateForm = document.getElementById("campaign-create-form");

// license control
const program = document.getElementById("appProgram");
const elicense = document.getElementById("elicense");

newCampaignButton.addEventListener("click", () => {
  campaignCreateForm.className = "block";
});

settingsBtn.addEventListener("click", () => {
  settingsContainer.className = "block";
});

settingsBtn.addEventListener("click", () => {
  settingsContainer.className = "block";
});

campaignBtnCls.addEventListener("click", () => {
  campaignCreateForm.className = "hidden";
});

settingsBtnCls.addEventListener("click", () => {
  settingsContainer.className = "hidden";
});

let selected = "";

if (localStorage.getItem("authed")) {
  program.className = "row block";
} else {
  elicense.className = "block";
}

// files for uploading
const fileMap = new Map();
// engine map
const engineMap = new Map();

let initialTarget = "";

// todo toggle license button on main form

function eventSub(event) {
  const raw = event.data;

  // license handling
  if (raw.startsWith("{" + '"' + "license")) {
    const data = JSON.parse(raw);
    const { license } = data || {};

    // set local storage of license enabled and display view
    if (license) {
      program.className = "row block";
      elicense.className = "hidden";
      localStorage.setItem("authed", true);
    } else {
      program.className = "row hidden";
      elicense.className = "block";
      localStorage.removeItem("authed");
    }
  }

  if (raw.startsWith("{" + '"' + "stats")) {
    const data = JSON.parse(raw);
    const {
      cpu_usage,
      load_avg_min,
      network_received,
      network_transmited,
      network_total_transmitted,
      memory_free,
      memory_used,
      memory_total,
    } = data.stats;

    setProgress(cpu_usage);

    if (cpua.innerHTML !== "1 min avg " + load_avg_min.toFixed(2) + "%") {
      cpua.innerHTML = "1 min avg " + load_avg_min.toFixed(2) + "%";
    }

    netstats.innerHTML =
      slowBytes(network_received) +
      " | " +
      slowBytes(network_transmited) +
      " | " +
      slowBytes(network_total_transmitted);
    memstats.innerHTML =
      slowBytes(memory_free) +
      " | " +
      slowBytes(memory_used) +
      " | " +
      slowBytes(memory_total);

    return;
  }

  const ptp = "{" + '"' + "pengine" + '"' + ":" + '"';

  if (raw.startsWith(ptp)) {
    const np = JSON.parse(raw);
    const { pengine, ploc } = np || {};

    if (engineMap.has(pengine)) {
      const item = engineMap.get(pengine);
      item.total = ploc ?? item.total ?? 0;
      item.valid = item.valid ?? 0;

      const cell = document.getElementById("engine_stats_" + pengine);

      if (cell) {
        cell.textContent = "( " + item.valid + " / " + item.total + " ) ";
      }
    }
    return;
  }

  if (raw.startsWith("{" + '"' + "path" + '"' + ":" + '"')) {
    const np = JSON.parse(raw);
    const { url, path } = np || {};

    if (engineMap.has(path)) {
      const item = engineMap.get(path);

      // todo: swap current log
      if (url && item.urls && !item.urls.has(url)) {
        item.urls.add(url);

        const logc = document.createElement("li");
        logc.innerText = url;

        logfeed.appendChild(logc);
      }
    }
    return;
  }

  const selectFile = "{" + '"' + "fpath" + '"' + ":" + '"';

  if (raw.startsWith(selectFile)) {
    const np = JSON.parse(raw);
    const path = np && np.fpath;

    if (!fileMap.has(path)) {
      fileMap.set(path, {});
      // file select
      const fileSelect = document.getElementById("target-select");

      if (initialTarget) {
        const kid = "fskeys_" + initialTarget;
        const item = document.getElementById(kid);
        if (!item) {
          const cellSelect = document.createElement("option");

          cellSelect.id = kid;
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
            const cellSelect = document.createElement("option");

            cellSelect.id = kid;
            cellSelect.name = "fsselect";
            cellSelect.value = key;
            cellSelect.innerText = key;

            fileSelect.appendChild(cellSelect);
          }
        }
      }
    }
    return;
  }

  const ptpe = "{" + '"' + "epath" + '"' + ":" + '"';

  if (raw.startsWith(ptpe)) {
    const np = JSON.parse(raw);
    const path = np && np.epath;

    if (!engineMap.has(path)) {
      engineMap.set(path, {
        total: 0,
        valid: 0,
        urls: new Set(),
      });
      const cell = document.createElement("li");

      const cellButtonWrapper = document.createElement("button");

      const cellBtnBlock = document.createElement("div");
      const cellContentBlock = document.createElement("div");
      const cellContentPaths = document.createElement("div");
      const cellTitle = document.createElement("div");
      const cellStats = document.createElement("div");
      const cellBtnDeleteButton = document.createElement("button");

      const cellBtnRunButton = document.createElement("button");

      cellButtonWrapper.className = "cell-btn";

      cell.className = "engine-item";
      cellBtnBlock.className = "row";
      cellBtnDeleteButton.className = "active-delete";

      cell.id = "engine_" + path;
      cellStats.id = "engine_stats_" + path;

      cellContentPaths.className = "row";
      cellContentBlock.className = "flex center";

      cellTitle.textContent = path;
      cellContentPaths.textContent = np.paths;
      cellStats.textContent = "( 0/" + 0 + " )";
      cellBtnDeleteButton.textContent = "Delete";
      cellBtnRunButton.textContent = "Run";

      cellContentBlock.appendChild(cellTitle);
      cellContentBlock.appendChild(cellContentPaths);
      cellContentBlock.appendChild(cellStats);

      // todo: replace feed
      cellButtonWrapper.addEventListener("click", (event) => {
        if (engineMap.has(path)) {
          // clear on new select
          if (selected && selected !== path) {
            logfeed.replaceChildren();
            // TODO: replace feed with new links
          }
        }

        const active = document.getElementsByClassName(
          "cell-btn cell-btn-active"
        );

        if (active) {
          for (let i = 0; i < active.length; i++) {
            active[i].className = "cell-btn";
          }
        }
        selected = path;

        cellButtonWrapper.className = "cell-btn cell-btn-active";
        event.preventDefault();
      });

      cellBtnRunButton.addEventListener("click", (event) => {
        if (engineMap.has(path)) {
          logfeed.replaceChildren();
        }

        socket.send("run-campaign " + path);
        event.preventDefault();
      });

      cellBtnDeleteButton.addEventListener("click", (event) => {
        if (engineMap.has(path)) {
          engineMap.delete(path);
          logfeed.replaceChildren();
        }
        socket.send("delete-engine " + path);
        cell.remove();
        event.preventDefault();
      });

      cellBtnBlock.appendChild(cellBtnRunButton);
      cellBtnBlock.appendChild(cellBtnDeleteButton);
      cellButtonWrapper.appendChild(cellContentBlock);
      cellButtonWrapper.appendChild(cellBtnBlock);

      // button wrapper
      cell.appendChild(cellButtonWrapper);
      list.appendChild(cell);
    }
    return;
  }

  const ptc = "{" + '"' + "count" + '"';

  if (raw.startsWith(ptc)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.path;

    if (engineMap.has(path)) {
      const item = engineMap.get(path);
      item.total = item.total ?? 0;
      item.valid = np.count ?? item.valid ?? 0;

      const cell = document.getElementById("engine_stats_" + path);

      if (cell) {
        cell.textContent = "( " + item.valid + " / " + item.total + " ) ";
      }
    }
  }

  const dfpath = "{" + '"' + "dfpath" + '"';

  if (raw.startsWith(dfpath)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.dfpath;

    if (fileMap.has(path)) {
      const kid = "fskeys_" + path;
      const cell = document.getElementById(kid);

      cell.remove();

      fileMap.delete(path);
    }
  }

  const bftc = "{" + '"' + "buffer" + '"';

  let defaultOptionSet = false;

  if (raw.startsWith(bftc)) {
    // parse json for now
    const np = JSON.parse(raw);

    if (!defaultOptionSet) {
      defaultOptionSet = true;
      initialTarget = np.target;
      const buffer = document.getElementById("buffer-select");
      const proxyform = document.getElementById("proxy-select");

      buffer.value = np.buffer;
      proxyform.checked = np.proxy;
    }
  }

  const deptc = "{" + '"' + "depath" + '"';

  if (raw.startsWith(deptc)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.depath;

    if (engineMap.has(path)) {
      const cell = document.getElementById("engine_" + path);
      const kitem = document.getElementById("ekeys_" + path);

      cell.remove();
      kitem.remove();
      engineMap.delete(path);
    }
  }
}

socket.addEventListener("message", eventSub);
socketRuntime.addEventListener("message", eventSub);

const rsform = document.getElementById("rsform");
const rform = document.getElementById("rform");
const eform = document.getElementById("eform");
const proxyform = document.getElementById("proxyform");
const uploadform = document.getElementById("uploadform");
const fsform = document.getElementById("fsform");
const bufferform = document.getElementById("bufferform");
const fsDelete = document.getElementById("fs-delete");

uploadform.addEventListener("submit", (event) => {
  const url = "http://localhost:8001/upload";
  const request = new XMLHttpRequest();

  request.open("POST", url, true);

  request.onload = function () {
    // console.log(request.responseText);
    // todo: upload complete
  };

  request.onerror = function () {
    // request failed
  };

  request.send(new FormData(event.target));
  event.preventDefault();
});

rsform.addEventListener("submit", (event) => {
  if (selected) {
    if (engineMap.has(selected)) {
      const e = engineMap.get(selected);
      e.url = [];
      // todo: replace it with new selected engine map
      logfeed.replaceChildren();
    }

    socket.send("run-campaign " + selected);
  } else {
    alert("Please select a campaign");
  }

  event.preventDefault();
});

rform.addEventListener("submit", (event) => {
  socket.send("run-all-campaigns");
  event.preventDefault();
});

bufferform.addEventListener("submit", (event) => {
  const buffer = bufferform.querySelector('input[name="buffer"]');
  const selected = buffer.value;

  socket.send("set-buffer " + selected);

  event.preventDefault();
});

proxyform.addEventListener("submit", (event) => {
  const prox = proxyform.querySelector('input[name="proxy"]');

  socket.send("set-proxy " + prox.checked);

  event.preventDefault();
});

fsDelete.addEventListener("click", () => {
  const cf = window.confirm("Are, you sure you want to delete this file?");

  if (cf) {
    const campaign = fsform.querySelector('select[name="target"]');
    const selected = campaign.value;

    socket.send("delete-file " + selected);
  }
});

fsform.addEventListener("submit", (event) => {
  const campaign = fsform.querySelector('select[name="target"]');
  const selected = campaign.value;

  if (!selected) {
    window.alert("Please select a list type");
  } else {
    socket.send("set-list " + selected);
  }

  event.preventDefault();
});

eform.addEventListener("submit", (event) => {
  const engine = eform.querySelector('input[name="ename"]');
  const epaths = eform.querySelector('input[name="epaths"]');
  const epatterns = eform.querySelector('input[name="epatterns"]');

  if (engine && engine.value) {
    const m = JSON.stringify({
      name: engine.value,
      paths: epaths.value,
      patterns: epatterns.value,
    });

    socket.send("create-engine " + m);
    socket.send("list-engines");
  } else {
    window.alert("Please enter a engine name");
  }
  event.preventDefault();
});

elicense.addEventListener("submit", (event) => {
  const slicense = elicense.querySelector('input[name="license"]');

  if (slicense && slicense.value) {
    socket.send("set-license " + slicense.value);
  } else {
    window.alert("Please enter a license.");
  }
  event.preventDefault();
});
