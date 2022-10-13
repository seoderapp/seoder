// check if socket already ready
const socket =
  typeof sock === "undefined" ? new WebSocket("ws://127.0.0.1:8080") : sock;

socket.addEventListener("open", () => {
  socket.send("list-campaigns");
  socket.send("list-engines");
  socket.send("list-totals");
  socket.send("list-files");
  socket.send("feed");
  socket.send("config");

  setTimeout(() => {
    socket.send("list-campaign-stats");
  });

  setInterval(() => {
    socket.send("list-campaigns");
  }, 3500);

  setInterval(() => {
    socket.send("feed");
  }, 1000);

  setInterval(() => {
    socket.send("list-campaign-stats");
  }, 5000);

  setInterval(() => {
    socket.send("list-totals");
    socket.send("list-files");
    socket.send("config");
  }, 30000);
});

const stats = document.getElementById("feed-stats");
const cpu = document.getElementById("cpu-stats");
const cpua = document.getElementById("cpu-stats-average");
const netstats = document.getElementById("network-stats");
const memstats = document.getElementById("memory-stats");

const pathMap = new Map();
const fileMap = new Map();
const engineMap = new Map();

let initialTarget = "";

socket.addEventListener("message", (event) => {
  const raw = event.data;
  
  if (raw.startsWith("{" + '"' + "stats")) {
    const data = JSON.parse(event.data);
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

  const ptpal = "{" + '"' + "apath" + '"' + ":" + '"';

  if (raw.startsWith(ptpal)) {
    const np = JSON.parse(raw);
    const { apath, ploc } = np || {};
    const path = apath;

    if (pathMap.has(path) && typeof ploc !== "undefined") {
      const item = pathMap.get(path);
      const cell = document.getElementById("campaign_" + path);

      item.total = ploc ?? 0 
      item.valid = np.count ?? item.valid ?? 0;

      if (cell && cell.firstChild && cell.firstChild.firstChild.nextSibling) {
        cell.firstChild.firstChild.nextSibling.nextSibling.textContent =
          "( " + item.valid + " / " + item.total + " ) ";
      }
    }

    return;
  }

  const ptp = "{" + '"' + "path" + '"' + ":" + '"';

  if (raw.startsWith(ptp)) {
    const list = document.getElementById("campaign-list");
    const np = JSON.parse(raw);

    const { path, pengine, url } = np || {};
    
    if(pathMap.has(path)) {
      const item = pathMap.get(path);

      if(item.urls && !item.urls.has(path)) {
        item.urls.add(url);
      }
    }
    if (!pathMap.has(path)) {
      pathMap.set(path, {
        urls: new Set()
      });
      const cell = document.createElement("li");
      cell.className = "campaign-item";
      cell.id = "campaign_" + path;

      const cellContentBlock = document.createElement("div");
      cellContentBlock.className = "flex center";

      const cellTitle = document.createElement("div");
      cellTitle.textContent = path;

      const cellEngine = document.createElement("div");
      cellEngine.textContent = pengine || "engine_default";

      const cellStats = document.createElement("div");

      cellStats.textContent = "( 0/" + 0 + " )";

      const cellBtnBlock = document.createElement("div");
      cellBtnBlock.className = "row";

      const cellBtnRunButton = document.createElement("button");
      const cellBtnDeleteButton = document.createElement("button");

      cellBtnDeleteButton.textContent = "Delete";
      cellBtnRunButton.textContent = "Run";

      cellBtnRunButton.addEventListener("click", (event) => {
        socket.send("run-campaign " + path);
        event.preventDefault();
      });

      cellBtnDeleteButton.addEventListener("click", (event) => {
        socket.send("delete-campaign " + path);
        event.preventDefault();
      });

      // actions
      cellBtnBlock.appendChild(cellBtnRunButton);
      cellBtnBlock.appendChild(cellBtnDeleteButton);

      // content
      cellContentBlock.appendChild(cellTitle);
      cellContentBlock.appendChild(cellEngine);
      cellContentBlock.appendChild(cellStats);

      cell.appendChild(cellContentBlock);
      cell.appendChild(cellBtnBlock);

      list.appendChild(cell);
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
    const list = document.getElementById("engine-list");
    const np = JSON.parse(raw);
    const path = np && np.epath;

    if (!engineMap.has(path)) {
      engineMap.set(path, {})
      const cell = document.createElement("li");
      cell.className = "engine-item";
      cell.id = "engine_" + path;

      const cellContentPaths = document.createElement("div");
      const cellContentBlock = document.createElement("div");
      cellContentPaths.className = "row";

      cellContentBlock.className = "flex center";

      const cellTitle = document.createElement("div");
      cellTitle.textContent = path;

      cellContentPaths.textContent = np.paths;

      cellContentBlock.appendChild(cellTitle);
      cellContentBlock.appendChild(cellContentPaths);

      // engine list item
      const cellBtnBlock = document.createElement("div");
      cellBtnBlock.className = "row";

      const cellBtnDeleteButton = document.createElement("button");
      cellBtnDeleteButton.textContent = "Delete";

      cellBtnDeleteButton.addEventListener("click", (event) => {
        socket.send("delete-engine " + path);
        event.preventDefault();
      });

      cellBtnBlock.appendChild(cellBtnDeleteButton);
      cell.appendChild(cellContentBlock);
      cell.appendChild(cellBtnBlock);
      list.appendChild(cell);

      // engine select
      const engineSelect = document.getElementById("engine-select");

      for (const [key, _] of engineMap) {
        const kid = "ekeys_" + key;
        const item = document.getElementById(kid);

        if (!item) {
          const cellContainer = document.createElement("div");
          const cellLabel = document.createElement("label");
          const cellSelect = document.createElement("input");

          cellLabel.htmlFor = kid;

          cellSelect.id = kid;
          cellSelect.name = "eselect";
          cellSelect.value = key;
          cellSelect.type = "radio";

          cellLabel.innerText = key;
          cellContainer.appendChild(cellLabel);
          cellContainer.appendChild(cellSelect);
          engineSelect.appendChild(cellContainer);
        }
      }
    }
    return;
  }

  const ptc = "{" + '"' + "count" + '"';

  if (raw.startsWith(ptc)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.path;

    if (pathMap.has(path)) {
      const item = pathMap.get(path);
      item.total = item.total ?? 0;
      item.valid = np.count ?? item.valid ?? 0;

      const cell = document.getElementById("campaign_" + path);
      if (cell && cell.firstChild && cell.firstChild.firstChild.nextSibling) {
        cell.firstChild.firstChild.nextSibling.nextSibling.textContent =
          "( " + item.valid + " / " + item.total + " ) ";
      }
    }
  }

  const dptc = "{" + '"' + "dcpath" + '"';

  if (raw.startsWith(dptc)) {
    // parse json for now
    const np = JSON.parse(raw);
    const path = np && np.dcpath;

    if (pathMap.has(path)) {
      const cell = document.getElementById("campaign_" + path);
      cell.remove();
      pathMap.delete(path);
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

      fileMap.delete(path)
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
      engineMap.delete(path)
    }
  }
});

const cform = document.getElementById("cform");
const rform = document.getElementById("rform");
const eform = document.getElementById("eform");
const uploadform = document.getElementById("uploadform");
const fsform = document.getElementById("fsform");
const bufferform = document.getElementById("bufferform");

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

const proxyform = document.getElementById("proxyform");

proxyform.addEventListener("submit", (event) => {
  const prox = proxyform.querySelector('input[name="proxy"]');

  socket.send("set-proxy " + prox.checked);

  event.preventDefault();
});

const fsDelete = document.getElementById("fs-delete");

fsDelete.addEventListener("click", (event) => {
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

cform.addEventListener("submit", (event) => {
  const campaign = cform.querySelector('input[name="cname"]');
  let cengine = "";

  Array.from(cform.querySelectorAll('input[type="radio"]')).forEach(function (
    ele
  ) {
    if (ele.checked === true) {
      cengine = ele.value;
    }
  });

  if (!cengine) {
    window.alert("Please select an engine");
  } else if (!campaign) {
    window.alert("Please enter a campaign name");
  } else {
    socket.send(
      "create-campaign " +
        JSON.stringify({ name: campaign.value, engine: cengine })
    );
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
