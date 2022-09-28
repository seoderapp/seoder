pub const RAW_JS_TOP: &'static str = r#"
<script>

  const socket = new WebSocket("ws://127.0.0.1:8080");
"#;

pub const RAW_JS_CPU_STATS: &'static str = r#"
<script>
  let ctx = document.getElementById('cpu-stats').getContext('2d');
  let al = 0;
  let start = 0;
  let cw = ctx.canvas.width;
  let ch = ctx.canvas.height; 

  ctx.font='2rem system-ui';
  ctx.lineWidth = 17;
  ctx.fillStyle = '#4285f4';
  ctx.strokeStyle = '#4285f4';
  ctx.textAlign = 'center';

  function setProgress(diff) {
    ctx.clearRect(0, 0, cw, ch);
    ctx.fillText(diff.toFixed(0)+'%', cw*.52, ch*.5+5, cw+12);
    ctx.beginPath();
    ctx.arc(100, 100, 75, start, (Math.PI * diff) / 50);
    ctx.stroke();
  }

</script>
"#;

pub const RAW_JS: &'static str = r#"
  socket.addEventListener("open", (event) => {
    socket.send("list-campaigns");
    socket.send("list-engines");
    socket.send("feed");
    socket.send("list-totals");

    setTimeout(() => {
      socket.send("list-campaign-stats")
    })

    setInterval(() => {
      socket.send("list-campaigns");
    }, 3500)
    setInterval(() => {
      socket.send("feed");
    }, 1000)

    setInterval(() => {
      socket.send("list-campaign-stats")
    }, 5000)

    setInterval(() => {
      socket.send("list-totals")
    }, 30000)

  });

  const stats = document.getElementById("feed-stats");
  const cpu = document.getElementById("cpu-stats");
  const cpua = document.getElementById("cpu-stats-average");
  const netstats = document.getElementById("network-stats");
  const memstats = document.getElementById("memory-stats");

  const pathMap = {};
  const engineMap = {};


  const units = ['bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
   
  function slowBytes(x){
    let l = 0;
    let n = parseInt(x, 10) || 0;

    while(n >= 1024 && ++l){
        n = n/1024;
    }
    
    return(n.toFixed(n < 10 && l > 0 ? 1 : 0) + ' ' + units[l]);
  }

  socket.addEventListener("message", (event) => {
    const raw = event.data;

    if (raw.startsWith("{" + '"' + "stats")) {
      const data = JSON.parse(event.data);
      const { cpu_usage, load_avg_min, network_received, network_transmited, network_total_transmitted, memory_free, memory_used, memory_total } = data.stats;

      setProgress(cpu_usage);

      let nextAVG = load_avg_min.toFixed(2);

      if (cpua.innerHTML !== "1 min avg " + load_avg_min.toFixed(2) + "%") {
        cpua.innerHTML = "1 min avg " + load_avg_min.toFixed(2) +"%";
      }

      netstats.innerHTML = slowBytes(network_received) + ' | ' + slowBytes(network_transmited) + ' | '+ slowBytes(network_total_transmitted);
      memstats.innerHTML = slowBytes(memory_free) + ' | ' + slowBytes(memory_used) + ' | ' +  slowBytes(memory_total);

      return;
    }

    const ptpal = "{" + '"' + "apath" + '"' + ":" + '"';

    if (raw.startsWith(ptpal)) {
      const list = document.getElementById("campaign-list");
      const np = JSON.parse(raw);
      const { apath, pengine, ploc } = np || {};
      const path = apath;

      if(path in pathMap && typeof ploc !== "undefined" ) {
          pathMap[path] = {
            total: ploc ?? 0,
            valid: np.count ?? pathMap[path].valid ?? 0
          };
          
          const cell = document.getElementById("campaign_" + path);

          if (cell && cell.firstChild && cell.firstChild.firstChild.nextSibling) {
            cell.firstChild.firstChild.nextSibling.nextSibling.textContent = "( " + pathMap[path].valid + " / " + pathMap[path].total + " ) ";
          }
      }

      return;
    }

    const ptp = "{" + '"' + "path" + '"' + ":" + '"';

    if (raw.startsWith(ptp)) {
      const list = document.getElementById("campaign-list");
      const np = JSON.parse(raw);
      const { path, pengine } = np || {};

      if(path in pathMap === false) {
          pathMap[path] = 0;
          const cell = document.createElement("li");  
          cell.className = "campaign-item";
          cell.id = "campaign_" + path;

          const cellContentBlock = document.createElement("div");  
          cellContentBlock.className = "flex center";

          const cellTitle = document.createElement("div");  
          cellTitle.textContent = path;

          const cellEngine = document.createElement("div");  
          cellEngine.textContent = pengine ||  "engine_default";

          const cellStats = document.createElement("div"); 
          cellStats.textContent = "( 0/" + 0 + " )";
          
          const cellBtnBlock = document.createElement("div");  
          cellBtnBlock.className = "row";

          const cellBtnRunButton = document.createElement("button");  
          const cellBtnDeleteButton = document.createElement("button");  

          cellBtnDeleteButton.textContent = "Delete";
          cellBtnRunButton.textContent = "Run";

          cellBtnRunButton.addEventListener("click", (event) => {
            const name = event.path[2].firstChild.firstChild.textContent;
            socket.send("run-campaign " + name)
            event.preventDefault();
          });

          cellBtnDeleteButton.addEventListener("click", (event) => {
            const name = event.path[2].firstChild.firstChild.textContent;
            socket.send("delete-campaign " + name)
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

    // todo delete pipe message on delete

    const ptpe = "{" + '"' + "epath" + '"' + ":" + '"';

    if (raw.startsWith(ptpe)) {
      const list = document.getElementById("engine-list");
      const np = JSON.parse(raw);
      const path = np && np.epath;

      if(path in engineMap === false) {
          engineMap[path] = 0;
          const cell = document.createElement("li");  
          cell.className = "engine-item";
          cell.id = "engine_" + path;

          const cellContentBlock = document.createElement("div");  
          cellContentBlock.className = "flex center";

          const cellTitle = document.createElement("div");  
          cellTitle.textContent = path;

          cellContentBlock.appendChild(cellTitle);

          // engine list item
          const cellBtnBlock = document.createElement("div");  
          cellBtnBlock.className = "row";

          const cellBtnDeleteButton = document.createElement("button");  
          cellBtnDeleteButton.textContent = "Delete";

          cellBtnDeleteButton.addEventListener("click", (event) => {
            const name = event.path[2].firstChild.textContent;
            socket.send("delete-engine " + name)
            event.preventDefault();
          });
        
          cellBtnBlock.appendChild(cellBtnDeleteButton);
          cell.appendChild(cellContentBlock);
          cell.appendChild(cellBtnBlock);
          list.appendChild(cell);

          // engine select
          const engineSelect = document.getElementById("engine-select");

          const eKeys = Object.keys(engineMap);

          eKeys.forEach((key) => {
            const inputName = "eselect" + key;
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
          });


     
      }
      return;
    }

    const ptc = "{" + '"' + "count" + '"';
      
    if (raw.startsWith(ptc)) {
      // parse json for now
      const np = JSON.parse(raw);
      const path = np && np.path;

      if(path in pathMap) {
          pathMap[path] = {
            total: pathMap[path].total ?? 0,
            valid: np.count ?? pathMap[path].valid ?? 0
          };
          const cell = document.getElementById("campaign_" + path);
          if (cell && cell.firstChild && cell.firstChild.firstChild.nextSibling) {
            cell.firstChild.firstChild.nextSibling.nextSibling.textContent = "( " + pathMap[path].valid + " / " + pathMap[path].total + " ) ";
          }
      }
    }

    const dptc = "{" + '"' + "dcpath" + '"';

    if (raw.startsWith(dptc)) {
      // parse json for now
      const np = JSON.parse(raw);
      const path = np && np.dcpath;

      if(path in pathMap) {
          const cell = document.getElementById("campaign_" + path);
          cell.remove();
          delete pathMap[path];
      }
    }

    const deptc = "{" + '"' + "depath" + '"';

    if (raw.startsWith(deptc)) {
      // parse json for now
      const np = JSON.parse(raw);
      const path = np && np.depath;

      if(path in engineMap) {
          const cell = document.getElementById("engine_" + path);
          cell.remove();
          delete engineMap[path];

          const kid = "ekeys_" + path;
          const kitem = document.getElementById(kid);

          kitem.remove();
      }
    }
  });

  const cform = document.getElementById("cform");
  const rform = document.getElementById("rform");
  const eform = document.getElementById("eform");
  const uploadform = document.getElementById("uploadform");

  uploadform.addEventListener("submit", (event) => {
    const url = "http://localhost:8001/upload";
    const request = new XMLHttpRequest();

    request.open('POST', url, true);

    request.onload = function() {
      // console.log(request.responseText);
      // todo: upload complete
    };
  
    request.onerror = function() {
      // request failed
    };

    request.send(new FormData(event.target));
    event.preventDefault();
  });
  
  rform.addEventListener("submit", (event) => {
    socket.send("run-all-campaigns");
    event.preventDefault();
  });
  
  cform.addEventListener("submit", (event) => {
    const campaign = cform.querySelector('input[name="cname"]');
    let cengine = "";

    Array.from(cform.querySelectorAll('input[type="radio"]')).forEach(function(ele){
      if(ele.checked === true) { 
        cengine = ele.value;
      } 
    });

    if (!cengine) {
      window.alert("Please select an engine");
    } else if (!campaign) {
      window.alert("Please enter a campaign name");
    } else {
      socket.send("create-campaign " + JSON.stringify({ name: campaign.value, engine: cengine }));
    }

    event.preventDefault();
  });
  
  eform.addEventListener("submit", (event) => {
    const engine = eform.querySelector('input[name="ename"]');
    const epaths = eform.querySelector('input[name="epaths"]');
    const epatterns = eform.querySelector('input[name="epatterns"]');

    if (engine && engine.value) {
      const m = JSON.stringify({ name: engine.value, paths: epaths.value, patterns: epatterns.value });

      socket.send("create-engine " + m);
      socket.send("list-engines");
    } else {
      window.alert("Please enter a engine name");
    }
    event.preventDefault();

  });

</script>
"#;
